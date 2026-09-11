//! Original short PCM cues; optional playback through the desktop's paplay.
use std::{
    io::Write,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
#[derive(Default)]
pub struct Sound {
    busy: Arc<AtomicBool>,
}
pub fn cue(finished: bool) -> Vec<u8> {
    let rate = 22050_u32;
    let samples = if finished { 6600 } else { 1800 };
    let mut bytes = Vec::with_capacity(44 + samples * 2);
    bytes.extend(b"RIFF");
    bytes.extend((36 + samples as u32 * 2).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes());
    bytes.extend(rate.to_le_bytes());
    bytes.extend((rate * 2).to_le_bytes());
    bytes.extend(2_u16.to_le_bytes());
    bytes.extend(16_u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend((samples as u32 * 2).to_le_bytes());
    for i in 0..samples {
        let t = i as f32 / rate as f32;
        let frequency = if finished && i > samples / 2 {
            660.
        } else {
            440.
        };
        let envelope = (1. - i as f32 / samples as f32) * (i as f32 / 100.).min(1.);
        let value = ((t * frequency * std::f32::consts::TAU).sin() * envelope * 2800.) as i16;
        bytes.extend(value.to_le_bytes());
    }
    bytes
}
impl Sound {
    pub fn available() -> bool {
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .any(|p| p.join("paplay").is_file())
    }
    pub fn play(&self, finished: bool) {
        if self.busy.swap(true, Ordering::Relaxed) {
            return;
        }
        let busy = self.busy.clone();
        thread::spawn(move || {
            // File input avoids a blocked pipe if the sound server stalls.
            if let Ok(mut file) = tempfile::NamedTempFile::new() {
                if file.write_all(&cue(finished)).is_ok() {
                    if let Ok(mut child) = Command::new("paplay")
                        .arg(file.path())
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                    {
                        let end = Instant::now() + Duration::from_secs(2);
                        while Instant::now() < end && matches!(child.try_wait(), Ok(None)) {
                            thread::sleep(Duration::from_millis(20));
                        }
                        let _ = child.kill();
                        let _ = child.wait();
                    }
                }
            }
            busy.store(false, Ordering::Relaxed);
        });
    }
}
