//! Filter focus-boundary input before egui computes widget clicks and shortcuts.
use eframe::egui::{self, Event, Key, PointerButton};
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct FocusInput {
    keys: HashSet<Key>,
    buttons: [bool; 5],
}
const BUTTONS: [PointerButton; 5] = [
    PointerButton::Primary,
    PointerButton::Secondary,
    PointerButton::Middle,
    PointerButton::Extra1,
    PointerButton::Extra2,
];

impl FocusInput {
    pub(crate) fn filter(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        if !raw.focused {
            ctx.stop_dragging();
            // Remember controls held before blur, including ones whose release is
            // delivered only after focus returns. They must not rearm on repeats.
            ctx.input_mut(|input| {
                self.keys.extend(input.keys_down.iter().copied());
                for button in BUTTONS {
                    self.buttons[button as usize] |= input.pointer.button_down(button);
                }
                input.keys_down.clear();
                input.pointer = Default::default();
            });
            for event in &raw.events {
                match event {
                    Event::Key { key, pressed, .. } => {
                        if *pressed {
                            self.keys.insert(*key);
                        } else {
                            self.keys.remove(key);
                        }
                    }
                    Event::PointerButton {
                        button, pressed, ..
                    } => {
                        self.buttons[*button as usize] = *pressed;
                    }
                    _ => {}
                }
            }
            // Keep backend notifications and capture replies. Raw viewport, time,
            // files and focus metadata remain intact. No interactive event reaches
            // either host shortcuts or game widgets while the window is inactive.
            raw.events.retain(|event| {
                matches!(event, Event::WindowFocused(_) | Event::Screenshot { .. })
            });
            raw.modifiers = egui::Modifiers::NONE;
        } else {
            raw.events.retain(|event| match event {
                Event::Key { key, pressed, .. } if self.keys.contains(key) => {
                    if !pressed {
                        self.keys.remove(key);
                    }
                    false
                }
                Event::PointerButton {
                    button, pressed, ..
                } if self.buttons[*button as usize] => {
                    if !pressed {
                        self.buttons[*button as usize] = false;
                    }
                    false
                }
                _ => true,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(key: Key, pressed: bool, repeat: bool) -> Event {
        Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat,
            modifiers: egui::Modifiers::NONE,
        }
    }
    fn pointer(pressed: bool) -> Event {
        Event::PointerButton {
            pos: egui::pos2(30., 30.),
            button: PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }
    fn frame(
        gate: &mut FocusInput,
        ctx: &egui::Context,
        focused: bool,
        events: Vec<Event>,
        mut check: impl FnMut(&egui::Context),
    ) {
        let mut raw = egui::RawInput {
            focused,
            events,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(300., 200.),
            )),
            ..Default::default()
        };
        gate.filter(ctx, &mut raw);
        let _ = ctx.run(raw, |ctx| check(ctx));
    }

    #[test]
    fn blur_frame_cannot_undo_pause_or_activate_a_shortcut() {
        let ctx = egui::Context::default();
        let mut gate = FocusInput::default();
        frame(
            &mut gate,
            &ctx,
            false,
            vec![key(Key::P, true, false), key(Key::Escape, true, false)],
            |ctx| {
                // Same ordering as the affected games: pause first, then shortcuts.
                let mut paused = !ctx.input(|i| i.focused);
                if ctx.input(|i| i.key_pressed(Key::P) || i.key_pressed(Key::Escape)) {
                    paused = !paused;
                }
                assert!(paused);
            },
        );
    }

    #[test]
    fn held_controls_require_release_then_a_fresh_press_after_focus_returns() {
        let ctx = egui::Context::default();
        let mut gate = FocusInput::default();
        frame(
            &mut gate,
            &ctx,
            true,
            vec![key(Key::ArrowLeft, true, false), pointer(true)],
            |ctx| {
                assert!(ctx.input(|i| i.key_down(Key::ArrowLeft) && i.pointer.primary_down()));
            },
        );
        frame(&mut gate, &ctx, false, vec![], |_| {});
        for events in [
            vec![],
            vec![key(Key::ArrowLeft, true, true), pointer(true)],
            vec![key(Key::ArrowLeft, false, false), pointer(false)],
        ] {
            frame(&mut gate, &ctx, true, events, |ctx| {
                assert!(ctx.input(|i| !i.key_down(Key::ArrowLeft)
                    && !i.pointer.primary_down()
                    && !i.pointer.any_click()));
            });
        }
        frame(
            &mut gate,
            &ctx,
            true,
            vec![key(Key::ArrowLeft, true, false), pointer(true)],
            |ctx| {
                assert!(ctx.input(|i| i.key_pressed(Key::ArrowLeft) && i.pointer.primary_down()));
            },
        );
    }

    #[test]
    fn a_button_pressed_before_blur_cannot_click_on_focus_return() {
        let ctx = egui::Context::default();
        let mut gate = FocusInput::default();
        let mut clicks = 0;
        let mut button = egui::Rect::NOTHING;
        let mut draw = |ctx: &egui::Context| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ui.button("Resume");
                button = response.rect;
                clicks += usize::from(response.clicked());
            });
        };
        frame(&mut gate, &ctx, true, vec![], &mut draw);
        let pos = button.center();
        let event = |pressed| Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        let mut draw = |ctx: &egui::Context| {
            egui::CentralPanel::default().show(ctx, |ui| {
                clicks += usize::from(ui.button("Resume").clicked());
            });
        };
        frame(
            &mut gate,
            &ctx,
            true,
            vec![Event::PointerMoved(pos), event(true)],
            &mut draw,
        );
        frame(&mut gate, &ctx, false, vec![], &mut draw);
        frame(
            &mut gate,
            &ctx,
            true,
            vec![Event::PointerMoved(pos), event(false)],
            &mut draw,
        );
        assert_eq!(clicks, 0);
        let mut draw = |ctx: &egui::Context| {
            egui::CentralPanel::default().show(ctx, |ui| {
                clicks += usize::from(ui.button("Resume").clicked());
            });
        };
        frame(
            &mut gate,
            &ctx,
            true,
            vec![Event::PointerMoved(pos), event(true)],
            &mut draw,
        );
        frame(&mut gate, &ctx, true, vec![event(false)], &mut draw);
        assert_eq!(clicks, 1);
    }

    #[test]
    fn inactive_text_scroll_and_pointer_events_are_removed_but_capture_survives() {
        let ctx = egui::Context::default();
        let mut gate = FocusInput::default();
        let mut raw = egui::RawInput {
            focused: false,
            time: Some(42.),
            events: vec![
                Event::Paste("name".into()),
                Event::Text("x".into()),
                pointer(true),
                Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0., 10.),
                    modifiers: egui::Modifiers::NONE,
                },
                Event::WindowFocused(false),
                Event::Screenshot {
                    viewport_id: egui::ViewportId::ROOT,
                    user_data: Default::default(),
                    image: std::sync::Arc::new(egui::ColorImage::new([1, 1], egui::Color32::BLACK)),
                },
            ],
            ..Default::default()
        };
        gate.filter(&ctx, &mut raw);
        assert_eq!(raw.time, Some(42.));
        assert_eq!(raw.events.len(), 2);
        assert!(matches!(raw.events[1], Event::Screenshot { .. }));
    }
}
