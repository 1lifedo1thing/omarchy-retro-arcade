use eframe::egui::{self, Key};
use std::{
    io::{self, Read, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
const WIDTH: usize = 1152;
const HEIGHT: usize = 790;
fn read_frame(reader: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut header = [0; 12];
    reader.read_exact(&mut header)?;
    if &header[..4] != b"OAR1"
        || u32::from_le_bytes(header[4..8].try_into().unwrap()) != WIDTH as u32
        || u32::from_le_bytes(header[8..12].try_into().unwrap()) != HEIGHT as u32
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid Circuit frame",
        ));
    }
    let mut bytes = vec![0; WIDTH * HEIGHT * 4];
    reader.read_exact(&mut bytes)?;
    Ok(bytes)
}
fn helper() -> io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or_else(|| io::Error::other("No executable directory"))?;
    let paths = [
        dir.join("../libexec/omarchy-retro-arcade/circuit"),
        dir.join("circuit"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../build/pinball/bin/omarchy-spacecadet-game"),
    ];
    paths.into_iter().find(|p| p.is_file()).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Circuit engine is missing. Reinstall Omarchy Arcade.",
        )
    })
}
struct Worker {
    child: Child,
    input: mpsc::SyncSender<String>,
    writer: Option<thread::JoinHandle<()>>,
    reader: Option<thread::JoinHandle<()>>,
}
impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.input.try_send("quit".into());
        let end = Instant::now() + Duration::from_secs(2);
        loop {
            match self.child.try_wait() {
                Ok(Some(_)) => break,
                _ if Instant::now() < end => thread::sleep(Duration::from_millis(10)),
                _ => {
                    let _ = self.child.kill();
                    let _ = self.child.wait();
                    break;
                }
            }
        }
        if let Some(writer) = self.writer.take() {
            let _ = writer.join();
        }
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}
pub struct Pinball {
    worker: Worker,
    pixels: Arc<Mutex<Option<Vec<u8>>>>,
    errors: mpsc::Receiver<String>,
    texture: Option<egui::TextureHandle>,
    error: Option<String>,
    focused: bool,
    input_enabled: bool,
    mouse_down: bool,
    started: Instant,
}
impl Pinball {
    pub fn ready(&self) -> bool {
        self.texture.is_some() || self.error.is_some()
    }
    pub fn finished(&mut self) -> bool {
        self.worker
            .child
            .try_wait()
            .ok()
            .flatten()
            .is_some_and(|s| s.success())
    }
    fn release_pointer(&mut self) {
        if self.mouse_down {
            // Release outside the worker UI so a blocked click cannot activate a menu item.
            self.send("mouse -1 -1 0".into());
            self.mouse_down = false;
        }
        self.send("mouse -1 -1 -1".into());
    }
    pub fn set_input_enabled(&mut self, enabled: bool) {
        if self.input_enabled && !enabled {
            self.release_pointer();
        }
        self.input_enabled = enabled;
    }
    pub fn pause(&mut self) {
        self.release_pointer();
        self.send("blur".into());
    }
    pub fn new(ctx: &egui::Context) -> io::Result<Self> {
        let mut child = Command::new(helper()?)
            .args(["--arcade-bridge", "--omarchy-table", "-sw"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .env_remove("OMARCHY_TEST_COMPACT")
            .env_remove("OMARCHY_TEST_TICKS")
            .env_remove("OMARCHY_TEST_SHOT")
            .env_remove("OMARCHY_TEST_SCREENSHOT")
            .spawn()?;
        let mut pipe = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("No Circuit input"))?;
        let (input, commands) = mpsc::sync_channel::<String>(256);
        let writer = thread::spawn(move || {
            while let Ok(command) = commands.recv() {
                if writeln!(pipe, "{command}").is_err() || command == "quit" {
                    break;
                }
            }
        });
        let mut output = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("No Circuit output"))?;
        let pixels = Arc::new(Mutex::new(None));
        let latest = pixels.clone();
        let ctx = ctx.clone();
        let (tx, errors) = mpsc::channel();
        let reader = thread::spawn(move || loop {
            match read_frame(&mut output) {
                Ok(bytes) => {
                    if let Ok(mut p) = latest.lock() {
                        *p = Some(bytes);
                    }
                    ctx.request_repaint();
                }
                Err(e) => {
                    let _ = tx.send(format!("Circuit stopped: {e}"));
                    ctx.request_repaint();
                    break;
                }
            }
        });
        Ok(Self {
            worker: Worker {
                child,
                input,
                writer: Some(writer),
                reader: Some(reader),
            },
            pixels,
            errors,
            texture: None,
            error: None,
            focused: true,
            input_enabled: true,
            mouse_down: false,
            started: Instant::now(),
        })
    }
    fn send(&mut self, command: String) {
        if self.error.is_none() {
            if let Err(e) = self.worker.input.try_send(command) {
                self.error = Some(format!("Circuit input failed: {e}"));
            }
        }
    }
}
fn keycode(key: Key) -> Option<i32> {
    Some(match key {
        Key::A => 97,
        Key::D => 100,
        Key::Z => 122,
        Key::X => 120,
        Key::Space => 32,
        Key::P => 112,
        Key::Escape => 27,
        Key::Enter => 13,
        Key::Tab => 9,
        Key::ArrowLeft => 1073741904,
        Key::ArrowRight => 1073741903,
        Key::ArrowUp => 1073741906,
        Key::ArrowDown => 1073741905,
        Key::F1 => 1073741882,
        Key::F2 => 1073741883,
        Key::N => 110,
        Key::M => 109,
        Key::Comma => 44,
        Key::Period => 46,
        _ => return None,
    })
}
impl eframe::App for Pinball {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if let Ok(e) = self.errors.try_recv() {
            self.error = Some(e);
        }
        if self.texture.is_none() && self.started.elapsed() > Duration::from_secs(15) {
            self.error.get_or_insert(
                "Circuit did not produce a frame. Return to Arcade and try again.".into(),
            );
        }
        if let Some(bytes) = self.pixels.lock().ok().and_then(|mut p| p.take()) {
            let image = egui::ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &bytes);
            if let Some(t) = &mut self.texture {
                t.set(image, egui::TextureOptions::LINEAR);
            } else {
                self.texture =
                    Some(ctx.load_texture("circuit-live", image, egui::TextureOptions::LINEAR));
            }
        }
        let focused = ctx.input(|i| i.focused) && self.input_enabled;
        if self.focused && !focused {
            self.pause();
        }
        self.focused = focused;
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                if let Some(error) = &self.error {
                    ui.heading("Circuit needs attention");
                    ui.label(error);
                    ui.label("Use Back to Arcade to choose another game.");
                    return;
                }
                let Some(texture) = &self.texture else {
                    ui.centered_and_justified(|ui| {
                        ui.spinner();
                    });
                    return;
                };
                let size = texture.size_vec2();
                let scale = (ui.available_width() / size.x).min(ui.available_height() / size.y);
                let image = ui.add(
                    egui::Image::new(texture)
                        .fit_to_exact_size(size * scale)
                        .sense(egui::Sense::click_and_drag()),
                );
                let rect = image.rect;
                if !focused {
                    return;
                }
                // Ownership matters: coordinates alone include overlapping host dialogs.
                let hovered = image.contains_pointer();
                for event in ctx.input(|i| i.events.clone()) {
                    match event {
                        egui::Event::Key {
                            key,
                            pressed,
                            repeat: false,
                            modifiers,
                            ..
                        } if focused => {
                            if let Some(code) = keycode(key) {
                                let mods = if modifiers.ctrl { 0x40 } else { 0 }
                                    | if modifiers.shift { 1 } else { 0 }
                                    | if modifiers.alt { 0x100 } else { 0 };
                                self.send(format!("key {code} {} {mods}", i32::from(pressed)));
                            }
                        }
                        egui::Event::PointerMoved(pos) if hovered => {
                            self.send(format!(
                                "mouse {} {} -1",
                                ((pos.x - rect.min.x) / scale) as i32,
                                ((pos.y - rect.min.y) / scale) as i32
                            ));
                        }
                        egui::Event::PointerButton {
                            pos,
                            button: egui::PointerButton::Primary,
                            pressed,
                            ..
                        } if (pressed && hovered) || (!pressed && self.mouse_down) => {
                            self.mouse_down = pressed;
                            self.send(format!(
                                "mouse {} {} {}",
                                ((pos.x - rect.min.x) / scale) as i32,
                                ((pos.y - rect.min.y) / scale) as i32,
                                i32::from(pressed)
                            ));
                        }
                        egui::Event::PointerMoved(_) | egui::Event::PointerGone
                            if !self.mouse_down =>
                        {
                            self.send("mouse -1 -1 -1".into());
                        }
                        egui::Event::MouseWheel { delta, .. } if hovered => {
                            let direction = delta.y.signum() as i32;
                            if direction != 0 {
                                self.send(format!("wheel {direction}"));
                            }
                        }
                        _ => {}
                    }
                }
            });
        ctx.request_repaint_after(Duration::from_millis(33));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_unbounded_or_unknown_frames() {
        assert!(read_frame(&mut &b"OAR1\xff\xff\xff\xff\xff\xff\xff\xff"[..]).is_err());
        assert!(read_frame(&mut &b"bad!\x80\x04\0\0\x16\x03\0\0"[..]).is_err());
    }
    #[test]
    fn complete_and_truncated_frames() {
        let mut data = b"OAR1\x80\x04\0\0\x16\x03\0\0".to_vec();
        data.resize(12 + WIDTH * HEIGHT * 4, 127);
        assert_eq!(
            read_frame(&mut data.as_slice()).unwrap().len(),
            WIDTH * HEIGHT * 4
        );
        data.pop();
        assert!(read_frame(&mut data.as_slice()).is_err());
    }
}
