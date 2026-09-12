use crate::{
    engine::{Phase, Point, Sim},
    world::{self, Kind, Obstacle},
};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Shape, Stroke, Vec2};
pub const VIEW_WIDTH: f64 = 96.;
pub const VIEW_HEIGHT: f64 = 72.;
pub fn field(area: Rect) -> Rect {
    let w = area.width().min(area.height() * 4. / 3.);
    Rect::from_center_size(area.center(), Vec2::new(w, w * 0.75))
}
pub fn point(field: Rect, sim: &Sim, p: Point) -> Pos2 {
    Pos2::new(
        field.center().x + (p.x / VIEW_WIDTH) as f32 * field.width(),
        field.top()
            + field.height() * 0.24
            + ((p.y - sim.position.y) / VIEW_HEIGHT) as f32 * field.height(),
    )
}
pub fn draw(
    ui: &egui::Ui,
    field: Rect,
    sim: &Sim,
    obstacles: &[Obstacle],
    accent: Color32,
    reduced: bool,
    tracks: &[(Point, Point)],
) {
    arcade_presentation::bezel(ui.painter(), field, accent);
    let p = ui.painter().with_clip_rect(field);
    let light = !ui.visuals().dark_mode;
    let snow = if light {
        Color32::from_rgb(239, 244, 242)
    } else {
        Color32::from_rgb(201, 220, 220)
    };
    let ink = Color32::from_rgb(40, 65, 70);
    let scale = field.width() / VIEW_WIDTH as f32;
    p.rect_filled(field, 0., snow);
    let at = |x, y| point(field, sim, Point { x, y });
    // A fixed world view reveals the same hazards at every window size.
    let start = ((sim.position.y - 20.) / 10.).floor() as i32;
    for row in start..start + 12 {
        let y = row as f64 * 10.;
        for side in [-1., 1.] {
            let a = at(side * world::HALF_WIDTH, y);
            p.line_segment(
                [a, a + Vec2::new(0., 5. * scale)],
                Stroke::new(scale * 0.22, Color32::from_rgb(136, 166, 173)),
            );
            p.line_segment(
                [
                    a + Vec2::new(-1.5 * scale, 0.),
                    a + Vec2::new(1.5 * scale, 0.),
                ],
                Stroke::new(scale * 0.13, ink),
            );
        }
    }
    if !reduced {
        for (a, b) in tracks {
            for offset in [-0.35, 0.35] {
                p.line_segment(
                    [at(a.x + offset, a.y), at(b.x + offset, b.y)],
                    Stroke::new((scale * 0.12).max(0.7), Color32::from_black_alpha(20)),
                );
            }
        }
    }
    for o in obstacles {
        let pos = point(field, sim, o.at);
        if !field.expand(10. * scale).contains(pos) {
            continue;
        }
        p.add(Shape::ellipse_filled(
            pos + Vec2::new(1.1, 0.8) * scale,
            Vec2::new(2.4, 0.8) * scale,
            Color32::from_black_alpha(28),
        ));
        match o.kind {
            Kind::Tree => {
                p.line_segment(
                    [pos, pos - Vec2::new(0., 2.) * scale],
                    Stroke::new(0.65 * scale, Color32::from_rgb(88, 76, 58)),
                );
                for (dy, w) in [(1.2, 2.5), (2.7, 2.), (4., 1.4)] {
                    let q = pos - Vec2::new(0., dy) * scale;
                    p.add(Shape::convex_polygon(
                        vec![
                            q + Vec2::new(-w, 0.) * scale,
                            q + Vec2::new(0., -3.) * scale,
                            q + Vec2::new(w, 0.) * scale,
                        ],
                        Color32::from_rgb(34, 83, 76),
                        Stroke::NONE,
                    ));
                    p.line_segment(
                        [
                            q + Vec2::new(-w * 0.7, -0.45) * scale,
                            q + Vec2::new(0., -2.7) * scale,
                        ],
                        Stroke::new(0.35 * scale, Color32::from_rgb(192, 215, 209)),
                    );
                }
            }
            Kind::Rock => {
                p.add(Shape::convex_polygon(
                    vec![
                        pos + Vec2::new(-1.5, 0.) * scale,
                        pos + Vec2::new(-0.9, -1.4) * scale,
                        pos + Vec2::new(0.7, -1.7) * scale,
                        pos + Vec2::new(1.5, -0.2) * scale,
                        pos + Vec2::new(0.5, 0.7) * scale,
                    ],
                    Color32::from_rgb(107, 127, 139),
                    Stroke::new(0.15 * scale, ink),
                ));
                p.line_segment(
                    [
                        pos + Vec2::new(-0.9, -1.3) * scale,
                        pos + Vec2::new(0.7, -1.6) * scale,
                    ],
                    Stroke::new(0.5 * scale, Color32::WHITE),
                );
            }
            Kind::Ramp => {
                let r = Rect::from_center_size(pos, Vec2::new(4.2, 3.3) * scale);
                p.rect_filled(r, 0., Color32::from_rgb(51, 109, 125));
                for y in [-0.8, 0., 0.8] {
                    p.line_segment(
                        [
                            pos + Vec2::new(-1.5, y) * scale,
                            pos + Vec2::new(1.5, y) * scale,
                        ],
                        Stroke::new(0.28 * scale, Color32::WHITE),
                    );
                }
                p.line_segment(
                    [r.left_bottom(), r.right_bottom()],
                    Stroke::new(0.5 * scale, Color32::from_rgb(231, 170, 71)),
                );
            }
        }
    }
    let finish = at(0., world::FINISH);
    if field.expand(8. * scale).contains(finish) {
        for n in -16..16 {
            let x = n as f32 * 2.5 * scale;
            let r = Rect::from_min_size(
                Pos2::new(finish.x + x, finish.y),
                Vec2::new(2.5, 1.2) * scale,
            );
            p.rect_filled(r, 0., if n % 2 == 0 { ink } else { Color32::WHITE });
        }
        p.text(
            finish - Vec2::new(0., 4. * scale),
            Align2::CENTER_CENTER,
            "PRACTICE FINISH",
            FontId::monospace(1.7 * scale),
            ink,
        );
    }
    let ground = point(field, sim, sim.position);
    p.add(Shape::ellipse_filled(
        ground + Vec2::new(0., 0.7) * scale,
        Vec2::new(1.6, 0.65) * scale,
        Color32::from_black_alpha(65),
    ));
    let skier = ground - Vec2::new(0., sim.height() as f32 * scale * 1.3);
    if sim.protection > 0 || sim.tumble > 0 {
        p.circle_stroke(
            ground,
            2.6 * scale,
            Stroke::new(0.24 * scale, Color32::from_rgb(168, 101, 24)),
        );
    }
    let angle = if sim.tumble > 0 {
        1.4
    } else {
        -(sim.heading as f32)
    };
    let rotate = |v: Vec2| {
        Vec2::new(
            v.x * angle.cos() - v.y * angle.sin(),
            v.x * angle.sin() + v.y * angle.cos(),
        )
    };
    for dx in [-0.55, 0.55] {
        p.line_segment(
            [
                skier + rotate(Vec2::new(dx, -1.4)) * scale,
                skier + rotate(Vec2::new(dx, 1.6)) * scale,
            ],
            Stroke::new(0.27 * scale, ink),
        );
    }
    let jacket = Color32::from_rgb(206, 89, 49);
    p.line_segment(
        [
            skier - rotate(Vec2::new(0., 1.)) * scale,
            skier + rotate(Vec2::new(0., 0.35)) * scale,
        ],
        Stroke::new(1.35 * scale, jacket),
    );
    p.circle_filled(
        skier - rotate(Vec2::new(0., 1.3)) * scale,
        0.65 * scale,
        Color32::from_rgb(248, 224, 174),
    );
    p.line_segment(
        [
            skier + rotate(Vec2::new(-1.15, -0.1)) * scale,
            skier + rotate(Vec2::new(1.15, -0.1)) * scale,
        ],
        Stroke::new(0.28 * scale, jacket),
    );
    if sim.phase == Phase::Ready {
        p.text(
            ground + Vec2::new(0., 5. * scale),
            Align2::CENTER_TOP,
            "YOUR FIRST TRACKS",
            FontId::monospace(2. * scale),
            ink,
        );
    }
    p.text(
        field.left_bottom() + Vec2::new(12., -10.),
        Align2::LEFT_BOTTOM,
        "PRACTICE / 1,200 m",
        FontId::monospace(12.),
        ink,
    );
}
