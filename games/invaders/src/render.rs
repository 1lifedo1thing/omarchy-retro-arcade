use crate::{
    bitmap,
    game::{H, W},
    App,
};
use eframe::egui::{self, Color32, Rect, Stroke, Vec2};
const INK: Color32 = Color32::from_rgb(228, 232, 223);
const AMBER: Color32 = Color32::from_rgb(255, 155, 54);
pub fn safe_accent(c: Color32) -> Color32 {
    // Keep hull inlays visibly separate from amber danger and from the dark field.
    let brightness = (c.r() as u32 * 3 + c.g() as u32 * 6 + c.b() as u32) / 10;
    if !(110..=225).contains(&brightness) || (c.r() > 200 && c.g() < 180) {
        Color32::from_rgb(179, 203, 146)
    } else {
        c
    }
}
pub fn board(app: &mut App, ctx: &egui::Context, blocked: bool) {
    let accent = safe_accent(app.theme.accent);
    app.art.prepare(ctx, accent);
    egui::CentralPanel::default().show(ctx, |ui| {
        let avail = ui.available_size();
        let scale = (avail.x / W).min(avail.y / (H + 90.)).max(0.1);
        let origin = ui.cursor().min + Vec2::new((avail.x - W * scale) / 2., 0.);
        let full = Rect::from_min_size(origin, Vec2::new(W, H + 90.) * scale);
        ui.allocate_rect(full, egui::Sense::hover());
        let p = ui.painter_at(full);
        p.rect_filled(full, 0., Color32::from_rgb(23, 28, 26));
        let header = |x: f32, y: f32| origin + Vec2::new(x, y) * scale;
        bitmap::text(&p, header(14., 10.), "OMARCHY INVADERS", 3. * scale, INK);
        bitmap::text(&p, header(655., 18.), "ORBIT", 2. * scale, accent);
        bitmap::text(&p, header(14., 49.), "SCORE", 1.5 * scale, accent);
        bitmap::text(
            &p,
            header(76., 45.),
            &format!("{:06}", app.s.game.score),
            3. * scale,
            INK,
        );
        bitmap::text(&p, header(222., 49.), "BEST", 1.5 * scale, accent);
        bitmap::text(
            &p,
            header(273., 47.),
            &format!("{:06}", app.s.high),
            2.5 * scale,
            INK,
        );
        bitmap::text(
            &p,
            header(465., 49.),
            &format!("WAVE {:02}", app.s.game.wave),
            2. * scale,
            INK,
        );
        for n in 0..app.s.game.lives {
            app.art
                .draw(&p, header(694. + n as f32 * 32., 56.), 0, 32. * scale, INK);
        }
        p.line_segment(
            [header(0., 82.), header(W, 82.)],
            Stroke::new(scale, accent),
        );
        let origin = header(0., 90.);
        let point = |x: f32, y: f32| origin + Vec2::new(x.round(), y.round()) * scale;
        let r = Rect::from_min_size(origin, Vec2::new(W, H) * scale);
        let p = p.with_clip_rect(r);
        app.art.backdrop(&p, r);
        for i in 0..36 {
            let x = ((i * 173 + 37) % 800) as f32;
            let y = ((i * 97 + 19) % 700) as f32;
            p.rect_filled(
                Rect::from_min_size(point(x, y), Vec2::splat(2. * scale)),
                0.,
                Color32::from_rgb(54, 66, 54),
            );
        }
        // Effects below projectiles so even busy impacts cannot hide a live threat.
        for b in &app.s.game.bursts {
            let cell = match (b.age / 0.09) as usize {
                0 => 16,
                1 => 17,
                2 => 18,
                _ => 19,
            };
            app.art.draw(
                &p,
                point(b.p.x, b.p.y),
                cell,
                if b.player { 80. } else { 48. } * scale,
                INK,
            );
        }
        for a in &app.s.game.aliens {
            let base = if a.kind == 0 {
                3
            } else if a.kind < 3 {
                6
            } else {
                9
            };
            let frame = app.s.game.march_frame as usize % 2;
            app.art.draw(
                &p,
                point(a.p.x, a.p.y),
                base + frame,
                if app.s.game.orbit { 32. } else { 44. } * scale,
                Color32::WHITE,
            );
        }
        for b in &app.s.game.shields {
            // Solid flat pixels exactly cover surviving physical erosion cells.
            let colour = if b.y < 572. {
                INK
            } else if b.y < 584. {
                accent
            } else {
                Color32::from_rgb(100, 122, 81)
            };
            p.rect_filled(
                Rect::from_center_size(point(b.x, b.y), Vec2::splat(6. * scale)),
                0.,
                colour,
            );
        }
        if let Some(u) = app.s.game.ufo {
            app.art.draw(
                &p,
                point(u.x, u.y),
                12 + (app.s.game.tick / 20 % 2) as usize,
                64. * scale,
                Color32::WHITE,
            );
        }
        if app.s.game.grace <= 0. || (app.s.game.tick / 12).is_multiple_of(2) {
            let frame = if app.s.game.shots.iter().any(|s| s.y > 604.) {
                2
            } else {
                (app.s.game.tick / 14 % 2) as usize
            };
            app.art.draw(
                &p,
                point(app.s.game.ship, 652.),
                frame,
                64. * scale,
                Color32::WHITE,
            );
        }
        for s in &app.s.game.shots {
            p.rect_filled(
                Rect::from_center_size(point(s.x, s.y), Vec2::new(3., 14.) * scale),
                0.,
                INK,
            );
        }
        for b in &app.s.game.bombs {
            for n in 0..5 {
                let dx = if (n + app.s.game.tick as usize / 10).is_multiple_of(2) {
                    -2.
                } else {
                    2.
                };
                p.rect_filled(
                    Rect::from_min_size(
                        point(b.x + dx, b.y - 8. + n as f32 * 3.),
                        Vec2::splat(3. * scale),
                    ),
                    0.,
                    AMBER,
                );
            }
        }
        p.line_segment(
            [point(16., 685.), point(784., 685.)],
            Stroke::new(scale, accent),
        );
        if app.s.game.transition > 0. {
            bitmap::text(
                &p,
                point(294., 350.),
                &format!("WAVE {:02}", app.s.game.wave),
                4. * scale,
                INK,
            );
        }
        if (blocked || app.s.game.over)
            && !app.settings
            && !app.help
            && !app.confirm
            && app.error.is_none()
        {
            p.rect_filled(r, 0., Color32::from_black_alpha(195));
            let title = if app.s.game.over {
                "GAME OVER"
            } else {
                "PAUSED"
            };
            bitmap::text(
                &p,
                point(400. - title.len() as f32 * 15., 290.),
                title,
                5. * scale,
                INK,
            );
            let sub = if app.s.game.over {
                format!("{:06} POINTS", app.s.game.score)
            } else {
                "P TO RESUME".into()
            };
            bitmap::text(
                &p,
                point(400. - sub.len() as f32 * 6., 352.),
                &sub,
                2. * scale,
                accent,
            );
            let card = Rect::from_center_size(point(400., 410.), Vec2::new(250., 60.) * scale);
            ui.scope_builder(egui::UiBuilder::new().max_rect(card), |ui| {
                ui.vertical_centered(|ui| {
                    if app.s.game.over {
                        if ui.button("PLAY AGAIN  [Enter]").clicked() {
                            app.restart();
                        }
                    } else if ui.button("RESUME  [P]").clicked() {
                        app.paused = false;
                    }
                });
            });
        }
    });
}
pub fn about_icon(ui: &mut egui::Ui, art: &crate::art::Art) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(80.), egui::Sense::hover());
    art.draw(ui.painter(), r.center(), 0, 80., Color32::WHITE);
}
