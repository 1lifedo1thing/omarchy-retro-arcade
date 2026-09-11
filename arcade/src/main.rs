mod pinball;
use eframe::egui::{self, Color32, Key, RichText, Vec2};
use fs2::FileExt;
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Game {
    Chess,
    Solitaire,
    Scram,
    Invaders,
    Pinball,
    Stack,
}
impl Game {
    const ALL: [Self; 6] = [
        Self::Pinball,
        Self::Solitaire,
        Self::Scram,
        Self::Invaders,
        Self::Chess,
        Self::Stack,
    ];
    fn id(self) -> &'static str {
        match self {
            Self::Stack => "stack",
            Self::Chess => "chess",
            Self::Solitaire => "solitaire",
            Self::Scram => "scram",
            Self::Invaders => "invaders",
            Self::Pinball => "pinball",
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Stack => "Stack",
            Self::Chess => "Chess",
            Self::Solitaire => "Solitaire",
            Self::Scram => "Scram",
            Self::Invaders => "Invaders",
            Self::Pinball => "Circuit Pinball",
        }
    }
    fn line(self) -> &'static str {
        match self {
            Self::Stack => "Make room. Go again.",
            Self::Chess => "Take your time. Make your move.",
            Self::Solitaire => "A quiet hand of Klondike.",
            Self::Scram => "Keep moving. They are behind you.",
            Self::Invaders => "Hold the line. Clear the sky.",
            Self::Pinball => "One more ball. One more high score.",
        }
    }
    fn image(self) -> egui::ImageSource<'static> {
        match self {
            Self::Stack => egui::include_image!("../../games/stack/docs/stack-game.png"),
            Self::Chess => egui::include_image!("../../games/chess/docs/preview.png"),
            Self::Solitaire => {
                egui::include_image!("../../games/solitaire/docs/screenshots/table.png")
            }
            Self::Scram => egui::include_image!("../../games/scram/docs/screenshots/charcoal.png"),
            Self::Invaders => egui::include_image!("../../games/invaders/docs/orbit-opening.png"),
            Self::Pinball => egui::include_image!("../../games/pinball/docs/upstream-circuit.png"),
        }
    }
}
trait ArcadeGame: eframe::App {
    fn suspend(&mut self) {}
    fn finished(&mut self) -> bool {
        false
    }
}
impl ArcadeGame for omarchy_stack::app::StackApp {
    fn suspend(&mut self) {
        self.suspend();
    }
    fn finished(&mut self) -> bool {
        omarchy_stack::app::StackApp::finished(self)
    }
}
impl ArcadeGame for omarchy_chess::ui::ChessApp {}
impl ArcadeGame for omarchy_solitaire::app::SolitaireApp {}
impl ArcadeGame for omarchy_scram::app::ScramApp {}
impl ArcadeGame for omarchy_invaders::App {}
impl ArcadeGame for pinball::Pinball {
    fn finished(&mut self) -> bool {
        self.finished()
    }
    fn suspend(&mut self) {
        self.pause();
    }
}
struct Active {
    game: Game,
    app: Box<dyn ArcadeGame>,
    _lock: Option<Box<dyn std::any::Any>>,
}
impl Drop for Active {
    fn drop(&mut self) {
        self.app.on_exit(None);
    }
}
struct Arcade {
    active: Option<Active>,
    selected: usize,
    error: Option<String>,
    about: bool,
    confirm_home: bool,
    theme: omarchy_chess::theme::Theme,
    themed: Instant,
    capture: Option<PathBuf>,
    frames: usize,
    initial: Option<Game>,
    _lock: File,
}
impl Arcade {
    fn open(&mut self, game: Game, ctx: &egui::Context) {
        let result = (|| -> Result<Active, String> {
            let mut lock: Option<Box<dyn std::any::Any>> = None;
            let app: Box<dyn ArcadeGame> = match game {
                Game::Stack => Box::new(omarchy_stack::app::StackApp::new()),
                Game::Chess => {
                    let dir = omarchy_chess::storage::state_dir();
                    lock = Some(Box::new(omarchy_chess::storage::SessionLock::acquire(
                        &dir,
                    )?));
                    Box::new(omarchy_chess::ui::ChessApp::new(dir))
                }
                Game::Solitaire => {
                    let dir = omarchy_solitaire::storage::state_dir();
                    lock = Some(Box::new(omarchy_solitaire::storage::SessionLock::acquire(
                        &dir,
                    )?));
                    Box::new(omarchy_solitaire::app::SolitaireApp::new(ctx, dir, None))
                }
                Game::Scram => {
                    let dir = omarchy_scram::storage::state_dir();
                    lock = Some(Box::new(
                        omarchy_scram::storage::SessionLock::acquire(&dir)
                            .map_err(|e| e.to_string())?,
                    ));
                    Box::new(omarchy_scram::app::ScramApp::new(ctx, dir, None))
                }
                Game::Invaders => Box::new(omarchy_invaders::App::new(
                    omarchy_invaders::storage::Store::open().map_err(|e| e.to_string())?,
                )),
                Game::Pinball => Box::new(pinball::Pinball::new(ctx).map_err(|e| e.to_string())?),
            };
            Ok(Active {
                game,
                app,
                _lock: lock,
            })
        })();
        match result {
            Ok(active) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                    "{} - Omarchy Arcade",
                    game.name()
                )));
                ctx.request_repaint();
                self.active = Some(active);
                self.error = None;
            }
            Err(e) => self.error = Some(e),
        }
    }
    fn home(&mut self, ctx: &egui::Context) {
        self.active = None;
        self.confirm_home = false;
        ctx.memory_mut(|m| *m = egui::Memory::default());
        ctx.set_style(egui::Style::default());
        ctx.send_viewport_cmd(egui::ViewportCommand::Title("Omarchy Arcade".into()));
    }
    fn shelf(&mut self, ctx: &egui::Context) {
        if self.themed.elapsed() > Duration::from_secs(2) {
            self.theme = omarchy_chess::theme::Theme::load();
            self.themed = Instant::now();
        }
        let t = &self.theme;
        let light =
            t.background.r() as u32 + t.background.g() as u32 + t.background.b() as u32 > 400;
        let mut visuals = if light {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        };
        visuals.panel_fill = t.background;
        visuals.window_fill = t.background;
        visuals.override_text_color = Some(t.foreground);
        visuals.selection.bg_fill = t.accent;
        ctx.set_visuals(visuals);
        let mut play = false;
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, Key::ArrowRight)
                || i.consume_key(egui::Modifiers::NONE, Key::ArrowDown)
            {
                self.selected = (self.selected + 1) % Game::ALL.len();
            }
            if i.consume_key(egui::Modifiers::NONE, Key::ArrowLeft)
                || i.consume_key(egui::Modifiers::NONE, Key::ArrowUp)
            {
                self.selected = (self.selected + Game::ALL.len() - 1) % Game::ALL.len();
            }
            play = i.consume_key(egui::Modifiers::NONE, Key::Enter);
        });
        let mut chosen = None;
        egui::TopBottomPanel::bottom("arcade-footer")
            .frame(
                egui::Frame::NONE
                    .fill(self.theme.background)
                    .inner_margin(20.),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("ARROWS  CHOOSE     ENTER  PLAY     CTRL+H  ARCADE")
                            .monospace()
                            .size(11.)
                            .weak(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("About").clicked() {
                            self.about = true;
                        }
                    });
                });
            });
        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(self.theme.background)
                    .inner_margin(28.),
            )
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("OMARCHY / SIX GOOD WAYS TO WASTE AN EVENING")
                        .monospace()
                        .size(11.)
                        .color(self.theme.accent),
                );
                ui.add_space(6.);
                ui.heading(RichText::new("Arcade.").size(52.).strong());
                ui.label(
                    RichText::new("Your computer. Your games. One more go.")
                        .size(16.)
                        .weak(),
                );
                ui.add_space(18.);
                let game = Game::ALL[self.selected];
                let height = (ui.available_height() - 160.).clamp(150., 460.);
                ui.horizontal(|ui| {
                    let image_width = (ui.available_width() * 0.64).max(250.);
                    egui::Frame::NONE
                        .fill(Color32::from_rgb(14, 17, 16))
                        .show(ui, |ui| {
                            ui.add(
                                egui::Image::new(game.image())
                                    .fit_to_exact_size(Vec2::new(image_width, height)),
                            );
                        });
                    ui.add_space(22.);
                    ui.vertical(|ui| {
                        ui.add_space(height * 0.16);
                        ui.label(
                            RichText::new(format!("0{} / 06", self.selected + 1))
                                .monospace()
                                .color(self.theme.accent),
                        );
                        ui.add_space(12.);
                        ui.heading(RichText::new(game.name()).size(29.));
                        ui.add_space(8.);
                        ui.label(game.line());
                        ui.add_space(22.);
                        if ui
                            .add_sized(
                                [160., 44.],
                                egui::Button::new(
                                    RichText::new("Play").size(19.).color(self.theme.background),
                                )
                                .fill(self.theme.accent)
                                .stroke(egui::Stroke::NONE),
                            )
                            .clicked()
                        {
                            play = true;
                        }
                        ui.add_space(12.);
                        ui.label(RichText::new("Offline. Always here.").small().weak());
                    });
                });
                ui.add_space(18.);
                ui.columns(Game::ALL.len(), |cols| {
                    for (i, col) in cols.iter_mut().enumerate() {
                        let g = Game::ALL[i];
                        let stroke = egui::Stroke::new(
                            if i == self.selected { 2_f32 } else { 1_f32 },
                            if i == self.selected {
                                self.theme.accent
                            } else {
                                self.theme.foreground.gamma_multiply(0.2)
                            },
                        );
                        let response = egui::Frame::NONE
                            .stroke(stroke)
                            .inner_margin(10.)
                            .show(col, |ui| {
                                ui.set_min_height(55.);
                                ui.label(
                                    RichText::new(format!("0{}", i + 1))
                                        .monospace()
                                        .small()
                                        .weak(),
                                );
                                ui.label(RichText::new(g.name()).strong());
                            })
                            .response;
                        if col
                            .interact(response.rect, response.id, egui::Sense::click())
                            .clicked()
                        {
                            chosen = Some(i);
                        }
                    }
                });
            });
        if let Some(i) = chosen {
            self.selected = i;
        }
        if play {
            self.open(Game::ALL[self.selected], ctx);
        }
    }
}
impl eframe::App for Arcade {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::F11)) {
            let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
        }
        if let Some(g) = self.initial.take() {
            self.open(g, ctx);
        }
        let mut home = ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, Key::H))
        });
        if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, Key::Q))
        }) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if let Some(a) = self.active.as_ref() {
            egui::TopBottomPanel::top("arcade-navigation").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    home |= ui.button("Back to Arcade    Ctrl+H").clicked();
                    ui.separator();
                    ui.label(a.game.name());
                });
            });
        }
        if home && self.active.is_some() {
            if self
                .active
                .as_ref()
                .is_some_and(|a| a.game == Game::Pinball)
            {
                self.confirm_home = true;
                if let Some(a) = self.active.as_mut() {
                    a.app.suspend();
                }
            } else {
                self.home(ctx);
            }
        }
        if self.confirm_home {
            let mut leave = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Enter));
            let mut cancel = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Escape));
            egui::Window::new("Return to Arcade?").collapsible(false).resizable(false)
                .anchor(egui::Align2::CENTER_CENTER,Vec2::ZERO).default_width(430.).show(ctx,|ui|{
                ui.label("This ends the current pinball game. Saved high scores and settings are kept.");
                ui.horizontal(|ui|{
                    leave|=ui.button("End game and return (Enter)").clicked();
                    cancel|=ui.button("Keep playing (Esc)").clicked();
                });
            });
            ctx.input_mut(|i| i.events.clear());
            if leave {
                self.home(ctx);
            } else if cancel {
                self.confirm_home = false;
            }
        }
        if let Some(a) = self.active.as_mut() {
            a.app.update(ctx, frame);
        } else {
            self.shelf(ctx);
        }
        if self.active.as_mut().is_some_and(|a| a.app.finished()) {
            self.home(ctx);
        }
        if let Some(error) = self.error.clone() {
            egui::Window::new("Could not open game").show(ctx, |ui| {
                ui.label(error);
                if ui.button("Close").clicked() {
                    self.error = None;
                }
            });
        }
        if self.about {
            egui::Window::new("About Omarchy Arcade").open(&mut self.about).show(ctx,|ui|{
            ui.heading("Omarchy Arcade");ui.label(concat!("Version ",env!("CARGO_PKG_VERSION")));ui.label("Six native games. A community project for Omarchy.");
            ui.label("Original game artwork and engines; credits and licences are included with the app.");ui.label("Ctrl+H returns to Arcade. Each game keeps its own controls and saves.");
        });
        }
        self.frames += 1;
        if let Some(path) = &self.capture {
            if self.frames > 60 {
                for event in ctx.input(|i| i.events.clone()) {
                    if let egui::Event::Screenshot { image, .. } = event {
                        let pixels: Vec<u8> =
                            image.pixels.iter().flat_map(|p| p.to_array()).collect();
                        match image::save_buffer(
                            path,
                            &pixels,
                            image.width() as u32,
                            image.height() as u32,
                            image::ColorType::Rgba8,
                        ) {
                            Ok(()) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                            Err(e) => {
                                eprintln!("Screenshot: {e}");
                            }
                        }
                    }
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
            ctx.request_repaint_after(Duration::from_millis(25));
        }
    }
    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        self.active = None;
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capture = None;
    let mut initial = None;
    let mut size = [1120., 860.];
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--version" => {
                println!("Omarchy Arcade {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--help" | "-h" => {
                println!("Omarchy Arcade\n--game chess|solitaire|scram|invaders|pinball|stack\n--screenshot PATH\n--compact\n--version\nCtrl+H: return to Arcade. Ctrl+Q: quit.");
                return Ok(());
            }
            "--game" => {
                let id = args.next().ok_or("Missing game")?;
                initial = Some(
                    Game::ALL
                        .into_iter()
                        .find(|g| g.id() == id)
                        .ok_or("Unknown game")?,
                );
            }
            "--screenshot" => {
                capture = Some(PathBuf::from(args.next().ok_or("Missing screenshot path")?))
            }
            "--compact" => size = [900., 760.],
            _ => return Err(format!("Unknown argument: {arg}").into()),
        }
    }
    let state = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))
        .ok_or("No state directory")?
        .join("omarchy-retro-arcade");
    std::fs::create_dir_all(&state)?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(state.join("session.lock"))?;
    lock.try_lock_exclusive()
        .map_err(|_| "Omarchy Arcade is already running")?;
    let image = egui_extras::image::load_svg_bytes_with_size(
        include_bytes!("../../packaging/omarchy-retro-arcade.svg"),
        Some(egui::load::SizeHint::Size(128, 128)),
    )?;
    let icon = egui::IconData {
        rgba: image.pixels.iter().flat_map(|p| p.to_array()).collect(),
        width: 128,
        height: 128,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_min_inner_size([900., 760.])
            .with_app_id("io.github.tcballard.omarchy-retro-arcade")
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "Omarchy Arcade",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Arcade {
                active: None,
                selected: 0,
                error: None,
                about: false,
                confirm_home: false,
                theme: omarchy_chess::theme::Theme::load(),
                themed: Instant::now(),
                capture,
                frames: 0,
                initial,
                _lock: lock,
            }))
        }),
    )?;
    Ok(())
}
