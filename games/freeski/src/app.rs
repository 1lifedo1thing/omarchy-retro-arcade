use crate::{
    engine::{Phase, Point, DT},
    input::Controls,
    render,
    storage::{self, Save},
    world::{self, Obstacle},
};
use eframe::egui::{self, Color32, Key, Rect, Sense, Vec2};
use omarchy_chess::theme::Theme;
use std::{
    collections::VecDeque,
    path::PathBuf,
    time::{Duration, Instant},
};

pub struct App {
    state: Save,
    path: PathBuf,
    obstacles: Vec<Obstacle>,
    controls: Controls,
    theme: Theme,
    themed: Instant,
    last: Instant,
    accumulator: f64,
    error: Option<String>,
    writable: bool,
    recovery: bool,
    help: bool,
    settings: bool,
    restart: bool,
    leave: bool,
    pause_reason: String,
    tracks: VecDeque<(Point, Point)>,
}
impl App {
    pub fn new() -> Result<Self, String> {
        Self::open(storage::path()?)
    }
    fn open(path: PathBuf) -> Result<Self, String> {
        let obstacles = world::practice();
        let (state, error, writable) = match storage::load(&path, &obstacles) {
            Ok(s) => (s, None, true),
            Err(e) => (Save::default(), Some(e), false),
        };
        let mut app = Self {
            state,
            path,
            obstacles,
            controls: Controls::default(),
            theme: Theme::load(),
            themed: Instant::now(),
            last: Instant::now(),
            accumulator: 0.,
            error,
            writable,
            recovery: !writable,
            help: false,
            settings: false,
            restart: false,
            leave: false,
            pause_reason: "Your run is saved. Resume when you are ready.".into(),
            tracks: VecDeque::new(),
        };
        app.flush();
        Ok(app)
    }
    fn flush(&mut self) {
        if self.writable {
            self.state.record_result();
            match storage::write(&self.path, &self.state, &self.obstacles) {
                Ok(()) => self.error = None,
                Err(e) => {
                    self.error = Some(e);
                    self.state.run.pause();
                    self.controls.clear();
                    self.accumulator = 0.;
                    self.pause_reason =
                        "Saving failed. Your run is paused; retry saving before continuing.".into();
                }
            }
        }
    }
    pub fn suspend(&mut self) {
        self.pause("Your run is saved. Resume when you are ready.");
    }
    pub fn finished(&mut self) -> bool {
        self.leave
    }
    fn pause(&mut self, reason: &str) {
        self.state.run.pause();
        self.controls.clear();
        self.accumulator = 0.;
        self.last = Instant::now();
        self.pause_reason = reason.into();
        self.flush();
    }
    fn begin(&mut self) {
        self.state.run.start();
        self.controls.clear();
        self.accumulator = 0.;
        self.last = Instant::now();
        self.flush();
    }
    fn new_run(&mut self) {
        self.state.restart();
        self.restart = false;
        self.controls.clear();
        self.tracks.clear();
        self.accumulator = 0.;
        self.flush();
    }
    fn blocked(&self) -> bool {
        self.help || self.settings || self.restart || self.recovery
    }
    pub fn show(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last).as_secs_f64();
        self.last = now;
        self.frame(ctx, elapsed);
    }
    fn frame(&mut self, ctx: &egui::Context, elapsed: f64) {
        if self.themed.elapsed() > Duration::from_secs(2) {
            self.theme = Theme::load();
            self.themed = Instant::now();
        }
        let mut visuals = if self.theme.light() {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        };
        visuals.panel_fill = self.theme.background;
        visuals.window_fill = self.theme.background;
        visuals.override_text_color = Some(self.theme.foreground);
        visuals.selection.bg_fill = self.theme.accent;
        ctx.set_visuals(visuals);
        arcade_presentation::apply(ctx);
        let focused = ctx.input(|i| i.focused);
        if !focused && self.state.run.phase == Phase::Running {
            self.pause("Focus changed. Your run is paused; resume when you are ready.");
        }
        let was_running = self.state.run.phase == Phase::Running && !self.blocked();
        let mut enter = false;
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, Key::F1) {
                self.pause("Help is open. Resume when you are ready.");
                self.help = true;
            }
            if i.consume_key(egui::Modifiers::CTRL, Key::Comma) {
                self.pause("Settings are open. Resume when you are ready.");
                self.settings = true;
            }
            if i.consume_key(egui::Modifiers::NONE, Key::Escape) {
                if self.help {
                    self.help = false;
                } else if self.settings {
                    self.settings = false;
                } else if self.restart {
                    self.restart = false;
                } else if !self.recovery {
                    if self.state.run.phase == Phase::Running {
                        self.pause("Take a breath. Your run is saved.");
                    } else if self.state.run.phase == Phase::Paused && focused {
                        self.begin();
                    }
                }
            }
            if !self.help && !self.settings && !self.recovery {
                enter = i.consume_key(egui::Modifiers::NONE, Key::Enter);
            }
        });
        let mut field = Rect::NOTHING;
        let mut brake = false;
        let mut start = false;
        let mut pause = false;
        let mut restart = false;
        let mut help = false;
        let mut settings = false;
        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .inner_margin(24.)
                    .fill(self.theme.background),
            )
            .show(ctx, |ui| {
                arcade_presentation::backdrop(ui);
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new("FreeSki").size(34.).monospace());
                    ui.separator();
                    ui.label("PRACTICE SLOPE");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.strong(format!("{:04.0} / 1200 m", self.state.run.distance));
                    ui.separator();
                    ui.label(format!("{} crashes left", 3 - self.state.run.crashes));
                    ui.separator();
                    ui.label(format!("{:02.0} km/h", self.state.run.speed * 3.6));
                    ui.separator();
                    ui.label(format!("Best {:.0} m", self.state.best_distance));
                    if self.state.run.tumble > 0 {
                        ui.colored_label(Color32::from_rgb(225, 153, 78), "Recovering");
                    } else if self.state.run.protection > 0 {
                        ui.label("Protected");
                    } else if self.state.run.jump.is_some() {
                        ui.label("Airborne");
                    }
                });
                ui.add_space(6.);
                ui.horizontal_wrapped(|ui| {
                    ui.add_enabled_ui(!self.blocked() && focused, |ui| {
                        match self.state.run.phase {
                            Phase::Ready => start = ui.button("Start skiing  Enter").clicked(),
                            Phase::Running => pause = ui.button("Pause  Esc").clicked(),
                            Phase::Paused => {}
                            _ => restart = ui.button("New run").clicked(),
                        }
                        if self.state.run.phase == Phase::Running {
                            let b = ui.add(
                                egui::Button::new("Hold to brake").sense(Sense::click_and_drag()),
                            );
                            brake = b.is_pointer_button_down_on();
                        }
                        help = ui.button("Help  F1").clicked();
                        settings = ui.button("Settings").clicked();
                    });
                });
                if let Some(error) = &self.error {
                    ui.colored_label(
                        if self.theme.light() {
                            Color32::DARK_RED
                        } else {
                            Color32::LIGHT_RED
                        },
                        error,
                    );
                }
                if !self.writable && !self.recovery {
                    ui.label("Playing without saving. The original save is retained.");
                }
                let (title, hint) = world::lesson(self.state.run.position.y);
                ui.add_space(6.);
                ui.label(egui::RichText::new(title).monospace());
                ui.label(hint);
                ui.add_space(12.);
                let (area, _) = ui
                    .allocate_exact_size(ui.available_size() - Vec2::new(0., 10.), Sense::hover());
                field = render::field(area.shrink(12.));
                let tracks: Vec<_> = self.tracks.iter().copied().collect();
                render::draw(
                    ui,
                    field,
                    &self.state.run,
                    &self.obstacles,
                    self.theme.accent,
                    self.state.reduced_effects,
                    &tracks,
                );
            });
        if pause {
            self.pause("Take a breath. Your run is saved.");
        }
        if help {
            self.pause("Help is open. Resume when you are ready.");
            self.help = true;
        }
        if settings {
            self.pause("Settings are open. Resume when you are ready.");
            self.settings = true;
        }
        if restart {
            self.new_run();
        }
        if start || (enter && self.state.run.phase == Phase::Ready && !self.blocked() && focused) {
            self.begin();
        }
        // Sampling is allowed at Ready for intentional keyboard starts. Overlays and
        // focus changes clear both input ownership and held keys until fresh release.
        if !self.blocked()
            && focused
            && matches!(self.state.run.phase, Phase::Ready | Phase::Running)
        {
            let skier = render::point(field, &self.state.run, self.state.run.position);
            let (input, intentional) = ctx.input(|i| self.controls.sample(i, field, skier, brake));
            if intentional && self.state.run.phase == Phase::Ready {
                self.state.run.start();
                self.flush();
            }
            if was_running && self.state.run.phase == Phase::Running {
                self.accumulator += elapsed;
                if self.accumulator > 0.25 {
                    self.pause("The slope fell behind. Paused so no collision time is skipped.");
                } else {
                    while self.accumulator + 1e-12 >= DT && self.state.run.phase == Phase::Running {
                        self.accumulator = (self.accumulator - DT).max(0.);
                        let before = self.state.run.position;
                        self.state.run.step(input, &self.obstacles);
                        if !self.state.reduced_effects
                            && self.state.run.jump.is_none()
                            && self.state.run.tumble == 0
                            && self.state.run.ticks.is_multiple_of(3)
                        {
                            let after = self.state.run.position;
                            if (before.x - after.x).hypot(before.y - after.y) < 1.
                                && after.y > before.y
                            {
                                self.tracks.push_back((before, after));
                            }
                            while self.tracks.len() > 240 {
                                self.tracks.pop_front();
                            }
                        }
                        if self.state.run.ended() || self.state.run.ticks.is_multiple_of(300) {
                            self.flush();
                        }
                    }
                }
            }
        } else {
            self.controls.clear();
            self.accumulator = 0.;
        }
        if self.recovery {
            egui::Modal::new(egui::Id::new("freeski-recovery")).show(ctx,|ui|{
                ui.set_max_width(420.);ui.heading("Your save needs attention");
                ui.label("The existing file could not be restored. It has not been changed. You can play without saving, or archive the original and begin a fresh practice run.");
                if ui.button("Play without saving").clicked() {self.recovery=false;self.controls.clear();}
                if ui.button("Archive original and start fresh").clicked() {
                    match storage::archive(&self.path) {Ok(_)=>{self.writable=true;self.recovery=false;self.state=Save::default();self.new_run();},Err(e)=>self.error=Some(e)}
                }
                if ui.button("Back to Arcade").clicked() {self.leave=true;}
            });
        } else if self.restart {
            egui::Modal::new(egui::Id::new("freeski-restart")).show(ctx, |ui| {
                ui.heading("Replace this run?");
                ui.label("Your unfinished run will be replaced. Records and preferences stay.");
                if ui.button("Replace run  Enter").clicked() || enter {
                    self.new_run();
                }
                if ui.button("Keep this run  Esc").clicked() {
                    self.restart = false;
                }
            });
        } else if self.help || self.settings {
            egui::Modal::new(egui::Id::new("freeski-help")).show(ctx,|ui|{
                ui.set_max_width(430.);
                if self.help {
                    ui.heading("Leave your first tracks");
                    ui.label("A / D or Left / Right carve. Release to point downhill. Move the pointer left or right of the skier to aim your turns. The last steering input takes control.");
                    ui.label("Hold S, Down, Space, the right mouse button on the slope, or Hold to brake. Striped ramps launch you automatically. Jump over low rocks; trees still cause a crash.");
                    ui.label("Three crashes end your run. The gold ring marks protection after recovery. Cross the finish flags to complete the 1,200 m practice slope.");
                    ui.label("Esc pauses or resumes. Ctrl+H returns to Arcade. Runs save automatically and reopen paused. Ctrl+, opens settings.");
                    ui.separator();ui.label("Original game and artwork by Omarchy Arcade contributors. GPL-3.0-or-later.");
                } else {
                    ui.heading("FreeSki settings");
                    if ui.checkbox(&mut self.state.reduced_effects,"Reduced effects (hide ski tracks)").changed() {self.tracks.clear();self.flush();}
                }
                if ui.button("Close  Esc").clicked() {self.help=false;self.settings=false;}
            });
        } else if self.state.run.phase == Phase::Paused {
            egui::Modal::new(egui::Id::new("freeski-pause")).show(ctx, |ui| {
                ui.set_max_width(410.);
                ui.heading("A moment on the mountain");
                ui.label(&self.pause_reason);
                if ui
                    .add_enabled(focused, egui::Button::new("Resume skiing  Enter"))
                    .clicked()
                    || (enter && focused)
                {
                    self.begin();
                }
                if self.error.is_some() && self.writable && ui.button("Retry saving").clicked() {
                    self.flush();
                }
                if ui.button("Restart practice slope").clicked() {
                    self.restart = true;
                }
                if ui.button("Settings").clicked() {
                    self.settings = true;
                }
                if ui.button("Back to Arcade").clicked() {
                    self.suspend();
                    self.leave = true;
                }
            });
        } else if self.state.run.ended() {
            egui::Modal::new(egui::Id::new("freeski-result")).show(ctx, |ui| {
                ui.heading(if self.state.run.phase == Phase::Finished {
                    "Fresh tracks. Slope complete."
                } else {
                    "Time for a fresh start."
                });
                ui.label(format!(
                    "Distance {:.0} m  /  Best {:.0} m",
                    self.state.run.distance, self.state.best_distance
                ));
                ui.label(format!(
                    "{} crashes  /  {:.1} seconds skiing",
                    self.state.run.crashes,
                    self.state.run.ticks as f64 / 60.
                ));
                if ui.button("New practice run  Enter").clicked() || enter {
                    self.new_run();
                }
                if ui.button("Back to Arcade").clicked() {
                    self.leave = true;
                }
            });
        }
        if self.state.run.phase == Phase::Running {
            ctx.request_repaint_after(Duration::from_millis(16));
        } else {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.show(ctx);
    }
    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        self.suspend();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Event, Modifiers, PointerButton, Pos2, RawInput};
    struct Harness {
        app: App,
        ctx: egui::Context,
        time: f64,
        dir: tempfile::TempDir,
        focused: bool,
        size: Vec2,
    }
    impl Harness {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let app = App::open(dir.path().join("freeski.json")).unwrap();
            let mut h = Self {
                app,
                ctx: egui::Context::default(),
                time: 0.,
                dir,
                focused: true,
                size: Vec2::new(900., 710.),
            };
            h.frame(vec![], DT);
            h.frame(vec![], DT);
            h
        }
        fn frame(&mut self, events: Vec<Event>, dt: f64) -> egui::FullOutput {
            self.time += dt;
            let mut elapsed = dt;
            self.ctx.run(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.size)),
                    time: Some(self.time),
                    focused: self.focused,
                    events,
                    ..Default::default()
                },
                |ctx| {
                    self.app.frame(ctx, elapsed);
                    elapsed = 0.;
                },
            )
        }
        fn key_event(key: Key, pressed: bool) -> Event {
            Event::Key {
                key,
                physical_key: None,
                pressed,
                repeat: false,
                modifiers: Modifiers::NONE,
            }
        }
        fn key(&mut self, key: Key) {
            self.frame(vec![Self::key_event(key, true)], DT);
            self.frame(vec![Self::key_event(key, false)], DT);
        }
        fn label(&mut self, label: &str) -> Pos2 {
            let out = self.frame(vec![], DT);
            out.shapes
                .iter()
                .rev()
                .find_map(|s| match &s.shape {
                    egui::Shape::Text(t) if t.galley.text() == label => {
                        Some(t.pos + t.galley.size() / 2.)
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("missing label {label}"))
        }
        fn click(&mut self, label: &str) {
            let pos = self.label(label);
            self.frame(
                vec![
                    Event::PointerMoved(pos),
                    Event::PointerButton {
                        pos,
                        button: PointerButton::Primary,
                        pressed: true,
                        modifiers: Modifiers::NONE,
                    },
                ],
                DT,
            );
            self.frame(
                vec![Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: false,
                    modifiers: Modifiers::NONE,
                }],
                DT,
            );
            self.frame(vec![], DT);
        }
    }
    #[test]
    fn mouse_flow_pause_settings_restart_cancel_and_reopen() {
        let mut h = Harness::new();
        h.click("Start skiing  Enter");
        assert_eq!(h.app.state.run.phase, Phase::Running);
        for _ in 0..100 {
            h.frame(vec![], DT);
        }
        h.click("Pause  Esc");
        assert_eq!(h.app.state.run.phase, Phase::Paused);
        let saved = h.app.state.run.clone();
        h.click("Restart practice slope");
        h.click("Keep this run  Esc");
        assert_eq!(h.app.state.run, saved);
        h.click("Settings");
        h.click("Reduced effects (hide ski tracks)");
        h.click("Close  Esc");
        assert!(h.app.state.reduced_effects);
        h.click("Resume skiing  Enter");
        assert_eq!(h.app.state.run.phase, Phase::Running);
        h.app.suspend();
        let reopened = App::open(h.dir.path().join("freeski.json")).unwrap();
        assert_eq!(reopened.state, h.app.state);
    }
    #[test]
    fn visible_brake_is_held_and_results_restart_without_losing_record() {
        let mut h = Harness::new();
        h.click("Start skiing  Enter");
        for _ in 0..180 {
            h.frame(vec![], DT);
        }
        assert!(h.app.state.run.speed > 10.);
        let pos = h.label("Hold to brake");
        h.frame(
            vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: Modifiers::NONE,
                },
            ],
            DT,
        );
        for _ in 0..120 {
            h.frame(vec![], DT);
        }
        assert!(h.app.state.run.speed < 0.01);
        h.frame(
            vec![Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::NONE,
            }],
            DT,
        );
        h.app.state.run.phase = Phase::Finished;
        h.app.state.run.position.y = world::FINISH;
        h.app.state.run.distance = world::FINISH;
        h.app.flush();
        h.frame(vec![], DT);
        h.click("New practice run  Enter");
        assert_eq!(h.app.state.run.phase, Phase::Ready);
        assert_eq!(h.app.state.completions, 1);
        assert_eq!(h.app.state.best_distance, world::FINISH);
    }

    #[test]
    fn focus_loss_and_overlay_dismissal_do_not_steer_or_advance() {
        let mut h = Harness::new();
        h.key(Key::Enter);
        h.frame(vec![Harness::key_event(Key::D, true)], DT);
        h.focused = false;
        h.frame(vec![], DT);
        assert_eq!(h.app.state.run.phase, Phase::Paused);
        let paused = h.app.state.run.clone();
        h.focused = true;
        h.frame(vec![], DT);
        assert_eq!(h.app.state.run, paused);
        h.key(Key::Enter);
        let resumed = h.app.state.run.heading;
        h.frame(vec![], DT);
        assert!(
            h.app.state.run.heading <= resumed,
            "held D must not continue steering after resume"
        );
        h.frame(vec![Harness::key_event(Key::D, false)], DT);
        h.key(Key::F1);
        let saved = h.app.state.run.clone();
        h.frame(vec![Harness::key_event(Key::D, true)], DT);
        h.key(Key::Escape);
        assert_eq!(h.app.state.run, saved);
    }
    #[test]
    fn render_schedule_and_resize_leave_tick_simulation_equivalent() {
        let mut runs = vec![];
        for hz in [30, 60, 120] {
            let mut h = Harness::new();
            h.app.state.run.start();
            h.frame(vec![], DT);
            // Reset to an identical state after layout settles.
            h.app.state.run = crate::engine::Sim::default();
            h.app.state.run.start();
            h.app.accumulator = 0.;
            for frame in 0..hz * 5 {
                h.size = if frame % 2 == 0 {
                    Vec2::new(900., 710.)
                } else {
                    Vec2::new(1280., 850.)
                };
                h.frame(vec![], 1. / hz as f64);
            }
            assert_eq!(h.app.state.run.ticks, 300);
            runs.push(h.app.state.run.clone());
        }
        assert_eq!(runs[0], runs[1]);
        assert_eq!(runs[1], runs[2]);
    }
    #[test]
    fn backlog_pauses_without_skipping_ticks_and_invalid_save_stays_intact() {
        let mut h = Harness::new();
        h.key(Key::Enter);
        let ticks = h.app.state.run.ticks;
        h.frame(vec![], 0.5);
        assert_eq!(h.app.state.run.phase, Phase::Paused);
        assert_eq!(h.app.state.run.ticks, ticks);
        h.app.suspend();
        std::fs::write(&h.app.path, b"future-save").unwrap();
        h.app = App::open(h.app.path.clone()).unwrap();
        h.frame(vec![], DT);
        h.click("Play without saving");
        h.click("Start skiing  Enter");
        h.app.suspend();
        assert_eq!(std::fs::read(&h.app.path).unwrap(), b"future-save");
    }
}
