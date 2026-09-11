use crate::{
    game::{H, W},
    App,
};
use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};
const SHIP: &[&str] = &[
    ".......#.......",
    "......###......",
    "......###......",
    "..#...###...#..",
    ".###.#####.###.",
    "###############",
    "###############",
    "##...#####...##",
];
const ALIENS: [&[&str]; 3] = [
    &[
        "....###....",
        "..#######..",
        ".##.#.#.##.",
        "###########",
        "##.#####.##",
        "...##.##...",
        "..##...##..",
        ".##.....##.",
    ],
    &[
        "..##...##..",
        "...##.##...",
        ".#########.",
        "###.###.###",
        "###########",
        "..##...##..",
        ".##.#.#.##.",
        "##.......##",
    ],
    &[
        "...#####...",
        ".#########.",
        "###..#..###",
        "###########",
        "..#######..",
        ".###...###.",
        "##..#.#..##",
        "..##...##..",
    ],
];
const UFO: &[&str] = &[
    "....#######....",
    "..###########..",
    ".##.#..#..#.##.",
    "###############",
    "..###..#..###..",
];
pub fn board(app: &mut App, ctx: &egui::Context, blocked: bool) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(8.);
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new("INVADERS")
                    .strong()
                    .color(app.theme.accent),
            );
            ui.label(egui::RichText::new("/ ORBITAL DEFENCE").small().weak());
        });
        ui.add_space(8.);
        ui.horizontal(|ui| {
            ui.monospace(format!(
                "SCORE  {:06}    BEST  {:06}",
                app.s.game.score, app.s.high
            ));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.monospace(format!(
                    "WAVE {:02}    LIVES {}",
                    app.s.game.wave, app.s.game.lives
                ));
            });
        });
        ui.add_space(12.);
        let avail = ui.available_size();
        let scale = (avail.x / W).min(avail.y / H).max(0.1);
        let size = Vec2::new(W, H) * scale;
        let origin = ui.cursor().min + Vec2::new((avail.x - size.x) / 2., 0.);
        let r = Rect::from_min_size(origin, size);
        ui.allocate_rect(r, egui::Sense::hover());
        let p = ui.painter_at(r);
        p.rect_filled(r, 4., Color32::from_rgb(10, 16, 20));
        let point = |x: f32, y: f32| origin + Vec2::new(x, y) * scale;
        let accent = if app.theme.accent.r() as u32
            + app.theme.accent.g() as u32
            + (app.theme.accent.b() as u32)
            < 240
        {
            Color32::from_rgb(179, 203, 146)
        } else {
            app.theme.accent
        };
        let ink = Color32::from_rgb(226, 236, 224);
        let amber = Color32::from_rgb(239, 182, 91);
        for i in 0..85 {
            let x = ((i * 173 + 37) % 800) as f32;
            let y = ((i * 97 + 19) % 700) as f32;
            let c = if i % 4 == 0 {
                Color32::from_rgb(51, 68, 72)
            } else {
                Color32::from_rgb(28, 40, 45)
            };
            p.circle_filled(point(x, y), scale.max(0.5), c);
        }
        p.line_segment(
            [point(22., 685.), point(778., 685.)],
            Stroke::new(scale, accent.gamma_multiply(0.5)),
        );
        for a in &app.s.game.aliens {
            let sprite = ALIENS[if a.kind == 0 {
                0
            } else if a.kind < 3 {
                1
            } else {
                2
            }];
            let c = if a.kind == 0 {
                amber
            } else if a.kind < 3 {
                accent
            } else {
                ink
            };
            pixel(
                &p,
                point(a.p.x, a.p.y),
                sprite,
                3. * scale,
                c,
                app.s.game.tick / 40 % 2 == 1,
            );
        }
        for b in &app.s.game.shields {
            p.rect_filled(
                Rect::from_center_size(point(b.x, b.y), Vec2::splat(5.5 * scale)),
                0.,
                accent.gamma_multiply(0.8),
            );
        }
        for b in &app.s.game.shots {
            p.rect_filled(
                Rect::from_center_size(point(b.x, b.y), Vec2::new(3., 14.) * scale),
                1.,
                ink,
            );
        }
        for b in &app.s.game.bombs {
            let c = point(b.x, b.y);
            p.line_segment(
                [c - Vec2::new(3., 7.) * scale, c + Vec2::new(3., 0.) * scale],
                Stroke::new(2. * scale, amber),
            );
            p.line_segment(
                [
                    c + Vec2::new(3., 0.) * scale,
                    c + Vec2::new(-3., 7.) * scale,
                ],
                Stroke::new(2. * scale, amber),
            );
        }
        if let Some(u) = app.s.game.ufo {
            pixel(&p, point(u.x, u.y), UFO, 3. * scale, amber, false);
        }
        if app.s.game.grace <= 0. || (app.s.game.tick / 12).is_multiple_of(2) {
            pixel(
                &p,
                point(app.s.game.ship, 652.),
                SHIP,
                3. * scale,
                accent,
                false,
            );
        }
        if app.s.game.transition > 0. {
            p.text(
                point(400., 360.),
                egui::Align2::CENTER_CENTER,
                format!("WAVE {:02}", app.s.game.wave),
                egui::FontId::monospace(32. * scale),
                ink,
            );
        }
        if (blocked && !app.settings && !app.help && !app.confirm && app.error.is_none())
            || (app.s.game.over
                && !app.settings
                && !app.help
                && !app.confirm
                && app.error.is_none())
        {
            p.rect_filled(r, 0., Color32::from_black_alpha(170));
            let card = Rect::from_center_size(r.center(), Vec2::new(330., 150.) * scale);
            ui.scope_builder(egui::UiBuilder::new().max_rect(card), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(if app.s.game.over {
                        "SECTOR LOST"
                    } else {
                        "PAUSED"
                    });
                    ui.label(if app.s.game.over {
                        format!("{} points · wave {}", app.s.game.score, app.s.game.wave)
                    } else {
                        "Ready when you are.".into()
                    });
                    ui.add_space(14.);
                    if app.s.game.over {
                        if ui.button("PLAY AGAIN").clicked() {
                            app.restart();
                        }
                    } else if ui.button("RESUME").clicked() {
                        app.paused = false;
                    }
                });
            });
        }
    });
}
fn pixel(p: &egui::Painter, center: Pos2, rows: &[&str], size: f32, c: Color32, alternate: bool) {
    let width = rows[0].len() as f32;
    for (y, row) in rows.iter().enumerate() {
        for (x, b) in row.bytes().enumerate() {
            if b == b'#' {
                let dx = if alternate && y >= rows.len() - 2 {
                    if x < rows[0].len() / 2 {
                        1.
                    } else {
                        -1.
                    }
                } else {
                    0.
                };
                let pos = center
                    + Vec2::new(
                        x as f32 + dx - width / 2.,
                        y as f32 - rows.len() as f32 / 2.,
                    ) * size;
                p.rect_filled(Rect::from_min_size(pos, Vec2::splat(size)), 0., c);
            }
        }
    }
}
pub fn about_icon(ui: &mut egui::Ui) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(64.), egui::Sense::hover());
    let p = ui.painter_at(r);
    let c = Color32::from_rgb(179, 203, 146);
    p.rect_filled(r, 12., Color32::from_rgb(23, 28, 26));
    pixel(&p, r.min + Vec2::new(32., 27.5), ALIENS[0], 3., c, false);
    for y in [49.5, 54.] {
        for x in [45., 49.5] {
            p.rect_filled(
                Rect::from_min_size(r.min + Vec2::new(x, y), Vec2::splat(3.)),
                0.,
                c,
            );
        }
    }
}
