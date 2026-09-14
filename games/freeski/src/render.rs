use crate::{
    engine::{Mode, Phase, Point, Sim},
    world::{self, Kind, Obstacle},
};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Shape, Stroke, Vec2};
pub const VIEW_WIDTH: f64 = 96.;
pub const VIEW_HEIGHT: f64 = 96.;
pub fn field(area: Rect) -> Rect {
    let w = area.width().min(area.height());
    Rect::from_center_size(area.center(), Vec2::splat(w))
}
pub fn point(field: Rect, sim: &Sim, p: Point) -> Pos2 {
    Pos2::new(
        field.center().x + (p.x / VIEW_WIDTH) as f32 * field.width(),
        field.top()
            + field.height() * 0.12
            + ((p.y - sim.position.y) / VIEW_HEIGHT) as f32 * field.height(),
    )
}
pub fn draw(
    ui: &egui::Ui,
    field: Rect,
    state: &crate::storage::Save,
    obstacles: &[Obstacle],
    accent: Color32,
    reduced: bool,
    tracks: &[(Point, Point)],
) {
    let sim = &state.run;
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
            Kind::Pole => {}
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
    if state.mode == Mode::Slalom {
        if let Some(course) = crate::course::course(state.course_index) {
            for (index, gate) in course.gates.iter().enumerate() {
                let centre = at(gate.x, gate.y);
                if !field.expand(8. * scale).contains(centre) {
                    continue;
                }
                let color = if index < state.slalom.next_gate {
                    Color32::from_rgb(125, 145, 145)
                } else if index == state.slalom.next_gate {
                    Color32::from_rgb(25, 95, 180)
                } else {
                    Color32::from_rgb(180, 55, 50)
                };
                for side in [-1., 1.] {
                    let base = at(gate.x + side * gate.half_width, gate.y);
                    let top = base - Vec2::new(0., 4.) * scale;
                    p.circle_stroke(base, (0.5 * scale).max(2.), Stroke::new(scale * 0.2, color));
                    p.line_segment([base, top], Stroke::new(scale * 0.3, color));
                    p.add(Shape::convex_polygon(
                        vec![
                            top,
                            top + Vec2::new(-side as f32 * 2.2, 0.5) * scale,
                            top + Vec2::new(0., 1.7) * scale,
                        ],
                        color,
                        Stroke::NONE,
                    ));
                }
                if index >= state.slalom.next_gate {
                    p.text(
                        centre - Vec2::new(0., 2.5) * scale,
                        Align2::CENTER_CENTER,
                        format!(
                            "{}{}",
                            index + 1,
                            if index == state.slalom.next_gate {
                                "  NEXT"
                            } else {
                                ""
                            }
                        ),
                        FontId::monospace((1.5 * scale).max(10.)),
                        color,
                    );
                }
            }
        }
    }
    let finish_distance = if state.mode == Mode::Slalom {
        crate::course::course(state.course_index).map_or(world::FINISH, |c| c.length)
    } else {
        world::FINISH
    };
    let finish = at(0., finish_distance);
    if state.mode != Mode::FreeSki && field.expand(8. * scale).contains(finish) {
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
            if state.mode == Mode::Slalom {
                "SLALOM FINISH"
            } else {
                "PRACTICE FINISH"
            },
            FontId::monospace(1.7 * scale),
            ink,
        );
    }
    if state.chase_enabled && state.chase.phase == crate::chase::ChasePhase::Active {
        let creature = point(field, sim, state.chase.position);
        let separation = (state.chase.position.x - sim.position.x)
            .hypot(state.chase.position.y - sim.position.y);
        if !field.shrink(4. * scale).contains(creature) {
            let marker = Pos2::new(
                creature.x.clamp(field.left() + 65., field.right() - 65.),
                creature.y.clamp(field.top() + 18., field.bottom() - 18.),
            );
            p.rect_filled(
                Rect::from_center_size(marker, Vec2::new(122., 25.)),
                4.,
                Color32::from_rgb(65, 47, 72),
            );
            p.text(
                marker,
                Align2::CENTER_CENTER,
                format!("CREATURE {:.0} m", separation),
                FontId::monospace(12.),
                Color32::WHITE,
            );
        } else {
            // An original horned snow runner, with its feet at the collision point.
            let fur = Color32::from_rgb(112, 72, 95);
            let gold = Color32::from_rgb(240, 181, 86);
            p.add(Shape::ellipse_filled(
                creature + Vec2::new(0., 0.5) * scale,
                Vec2::new(2.7, 0.9) * scale,
                Color32::from_black_alpha(50),
            ));
            let stride = if reduced {
                0.
            } else {
                (sim.ticks as f32 * 0.4).sin() * 0.5
            };
            for side in [-1., 1.] {
                let foot = creature + Vec2::new(side, side * stride) * scale;
                p.line_segment(
                    [foot, creature + Vec2::new(side * 0.7, -1.7) * scale],
                    Stroke::new(0.65 * scale, fur),
                );
            }
            p.add(Shape::convex_polygon(
                vec![
                    creature + Vec2::new(-2., -1.) * scale,
                    creature + Vec2::new(-1.7, -4.) * scale,
                    creature + Vec2::new(0., -4.7) * scale,
                    creature + Vec2::new(1.7, -4.) * scale,
                    creature + Vec2::new(2., -1.) * scale,
                ],
                fur,
                Stroke::new(0.2 * scale, ink),
            ));
            for side in [-1., 1.] {
                p.add(Shape::convex_polygon(
                    vec![
                        creature + Vec2::new(side * 1.2, -3.6) * scale,
                        creature + Vec2::new(side * 2.2, -5.3) * scale,
                        creature + Vec2::new(side * 0.3, -4.4) * scale,
                    ],
                    gold,
                    Stroke::NONE,
                ));
                p.circle_filled(
                    creature + Vec2::new(side * 0.55, -3.) * scale,
                    0.22 * scale,
                    Color32::WHITE,
                );
            }
            p.line_segment(
                [
                    creature + Vec2::new(-0.45, -2.2) * scale,
                    creature + Vec2::new(0.45, -2.2) * scale,
                ],
                Stroke::new(0.2 * scale, gold),
            );
        }
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
    // Yaw belongs to the skis/feet. Keep the standing body vertical in the
    // oblique view so a traverse reads as turning, rather than falling over.
    let body = |v: Vec2| if sim.tumble > 0 { rotate(v) } else { v };
    let jacket = Color32::from_rgb(206, 89, 49);
    let hip = skier - body(Vec2::new(0., 0.65)) * scale;
    for dx in [-0.55, 0.55] {
        let boot = skier + rotate(Vec2::new(dx, 0.)) * scale;
        p.line_segment([hip, boot], Stroke::new(0.38 * scale, ink));
    }
    let shoulder = skier - body(Vec2::new(0., 1.65)) * scale;
    p.line_segment([hip, shoulder], Stroke::new(1.25 * scale, jacket));
    let head = skier - body(Vec2::new(0., 2.25)) * scale;
    p.circle_filled(head, 0.65 * scale, Color32::from_rgb(248, 224, 174));
    // Goggles shift toward the direction of travel, showing left/right profile.
    let facing = sim.heading.sin() as f32;
    p.line_segment(
        [
            head + Vec2::new(facing * 0.35 - 0.3, 0.15) * scale,
            head + Vec2::new(facing * 0.35 + 0.3, 0.15) * scale,
        ],
        Stroke::new(0.22 * scale, ink),
    );
    for side in [-1., 1.] {
        let hand = shoulder + body(Vec2::new(side * 0.95, 0.65)) * scale;
        p.line_segment([shoulder, hand], Stroke::new(0.3 * scale, jacket));
        p.line_segment(
            [hand, hand + rotate(Vec2::new(0., -1.5)) * scale],
            Stroke::new(0.12 * scale, ink),
        );
    }
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
        if state.mode == Mode::Practice {
            "PRACTICE / 1,200 m"
        } else if state.mode == Mode::Slalom {
            "SLALOM / FIND YOUR LINE"
        } else {
            "FREE SKI / KEEP GOING"
        },
        FontId::monospace(12.),
        ink,
    );
}
