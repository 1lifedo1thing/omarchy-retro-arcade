//! Original vector deck: readable horizontal indexes and mirrored court portraits.
use crate::{
    game::Card,
    theme::{blend, Palette, FACE, INK, RED},
};
use eframe::egui::{
    self, pos2, vec2, Align2, Color32, FontId, Painter, Rect, Shape, Stroke, StrokeKind,
    TextureHandle,
};

pub fn suit(p: &Painter, center: egui::Pos2, size: f32, s: u8, color: Color32) {
    let q = |x: f32, y: f32| center + vec2(x, y) * size;
    let poly = |points: &[(f32, f32)]| {
        p.add(Shape::convex_polygon(
            points.iter().map(|&(x, y)| q(x, y)).collect(),
            color,
            Stroke::NONE,
        ));
    };
    match s {
        1 => poly(&[(0., -0.6), (0.4, 0.), (0., 0.6), (-0.4, 0.)]),
        2 => {
            p.circle_filled(q(-0.22, -0.18), size * 0.29, color);
            p.circle_filled(q(0.22, -0.18), size * 0.29, color);
            poly(&[(-0.46, -0.1), (0.46, -0.1), (0., 0.55)]);
        }
        3 => {
            p.circle_filled(q(-0.2, 0.07), size * 0.27, color);
            p.circle_filled(q(0.2, 0.07), size * 0.27, color);
            poly(&[(-0.43, 0.12), (0., -0.6), (0.43, 0.12)]);
            poly(&[(-0.22, 0.55), (0., 0.03), (0.22, 0.55)]);
        }
        _ => {
            for (x, y) in [(0., -0.28), (-0.25, 0.12), (0.25, 0.12)] {
                p.circle_filled(q(x, y), size * 0.28, color);
            }
            poly(&[(-0.22, 0.58), (0., 0.0), (0.22, 0.58)]);
        }
    }
}
fn court(p: &Painter, r: Rect, c: Card) {
    let navy = Color32::from_rgb(36, 66, 82);
    let gold = Color32::from_rgb(179, 135, 48);
    let skin = Color32::from_rgb(239, 215, 175);
    let color = if c.red() { RED } else { navy };
    p.rect_filled(r, 1, blend(FACE, gold, 0.08));
    p.rect_stroke(r, 1, Stroke::new(1.0_f32, gold), StrokeKind::Inside);
    for invert in [false, true] {
        let q = |x: f32, y: f32| {
            if invert {
                pos2(r.right() - x * r.width(), r.bottom() - y * r.height())
            } else {
                pos2(r.left() + x * r.width(), r.top() + y * r.height())
            }
        };
        let poly = |pts: &[(f32, f32)], fill| {
            p.add(Shape::convex_polygon(
                pts.iter().map(|&(x, y)| q(x, y)).collect(),
                fill,
                Stroke::new(0.6_f32, INK),
            ));
        };
        // Mantle, sash and ornamental shoulders.
        poly(
            &[(0.12, 0.50), (0.24, 0.33), (0.69, 0.32), (0.88, 0.50)],
            color,
        );
        poly(
            &[(0.28, 0.34), (0.40, 0.33), (0.75, 0.50), (0.61, 0.50)],
            gold,
        );
        for x in [0.24, 0.34, 0.69, 0.78] {
            p.line_segment([q(x, 0.42), q(x - 0.03, 0.48)], Stroke::new(1.0_f32, gold));
        }
        // Hair and face in profile; each rank has its own headwear and emblem.
        poly(
            &[
                (0.36, 0.14),
                (0.58, 0.12),
                (0.68, 0.20),
                (0.62, 0.35),
                (0.34, 0.34),
            ],
            navy,
        );
        poly(
            &[
                (0.43, 0.17),
                (0.61, 0.17),
                (0.63, 0.22),
                (0.70, 0.25),
                (0.62, 0.26),
                (0.61, 0.31),
                (0.45, 0.32),
                (0.40, 0.27),
            ],
            skin,
        );
        p.circle_filled(q(0.59, 0.215), r.width() * 0.014, INK);
        p.line_segment([q(0.59, 0.28), q(0.63, 0.28)], Stroke::new(0.7_f32, INK));
        if c.rank() == 11 {
            poly(
                &[(0.31, 0.17), (0.39, 0.08), (0.59, 0.08), (0.69, 0.17)],
                color,
            );
            p.line_segment([q(0.39, 0.11), q(0.28, 0.03)], Stroke::new(2.0_f32, gold));
            p.line_segment([q(0.80, 0.43), q(0.80, 0.10)], Stroke::new(2.0_f32, navy));
            p.line_segment([q(0.73, 0.18), q(0.87, 0.18)], Stroke::new(1.5_f32, gold));
        } else {
            poly(
                &[
                    (0.35, 0.17),
                    (0.31, 0.06),
                    (0.42, 0.10),
                    (0.49, 0.03),
                    (0.57, 0.10),
                    (0.67, 0.06),
                    (0.63, 0.17),
                ],
                gold,
            );
            for x in [0.4, 0.49, 0.58] {
                p.circle_filled(q(x, 0.135), r.width() * 0.018, color);
            }
            if c.rank() == 12 {
                p.line_segment([q(0.80, 0.43), q(0.80, 0.25)], Stroke::new(1.2_f32, navy));
                for (x, y) in [(0.80, 0.21), (0.75, 0.25), (0.85, 0.25), (0.80, 0.29)] {
                    p.circle_filled(q(x, y), r.width() * 0.045, color);
                }
                p.circle_filled(q(0.80, 0.25), r.width() * 0.035, gold);
            } else {
                poly(
                    &[(0.45, 0.29), (0.62, 0.29), (0.57, 0.37), (0.47, 0.34)],
                    navy,
                );
                p.line_segment([q(0.81, 0.45), q(0.81, 0.12)], Stroke::new(2.0_f32, gold));
                p.circle_filled(q(0.81, 0.12), r.width() * 0.047, gold);
            }
        }
        suit(p, q(0.18, 0.22), r.width() * 0.16, c.suit(), color);
    }
    p.line_segment(
        [r.left_center(), r.right_center()],
        Stroke::new(1.0_f32, gold),
    );
}
pub struct DeckStyle<'a> {
    pub palette: &'a Palette,
    pub pattern: usize,
    pub holographic: bool,
    pub sheen: f32,
    pub logo: &'a TextureHandle,
    pub art: Option<&'a TextureHandle>,
}
pub fn paint(p: &Painter, r: Rect, c: Card, style: &DeckStyle<'_>, selected: bool) {
    p.rect_filled(r.translate(vec2(1., 2.)), 4, Color32::from_black_alpha(48));
    p.rect_filled(r, 4, FACE);
    p.rect_stroke(
        r,
        4,
        Stroke::new(
            if selected { 2.5_f32 } else { 0.8_f32 },
            if selected {
                style.palette.accent
            } else {
                blend(INK, FACE, 0.65)
            },
        ),
        StrokeKind::Inside,
    );
    if !c.1 {
        let inset = r.shrink(5.);
        p.rect_filled(inset, 2, style.palette.back);
        if let Some(art) = style.art {
            let size = art.size_vec2();
            let scale = (inset.width() / size.x).min(inset.height() / size.y);
            let target = Rect::from_center_size(inset.center(), size * scale);
            p.image(
                art.id(),
                target,
                Rect::from_min_max(pos2(0., 0.), pos2(1., 1.)),
                Color32::WHITE,
            );
        } else {
            let clip = p.with_clip_rect(inset);
            let step = (r.width() * 0.11).max(7.);
            if style.pattern < 2 {
                for row in 0..(inset.height() / step) as i32 + 1 {
                    for col in 0..(inset.width() / step) as i32 + 1 {
                        let center = inset.min + vec2(col as f32 * step, row as f32 * step);
                        if style.pattern == 0 {
                            clip.line_segment(
                                [center, center + vec2(step * 0.42, step * 0.42)],
                                Stroke::new(1.0_f32, style.palette.pattern),
                            );
                            clip.line_segment(
                                [
                                    center + vec2(step * 0.55, 0.),
                                    center + vec2(step * 0.95, step * 0.4),
                                ],
                                Stroke::new(1.0_f32, style.palette.pattern),
                            );
                        } else {
                            clip.add(Shape::closed_line(
                                vec![
                                    center + vec2(0., -step * 0.35),
                                    center + vec2(step * 0.3, 0.),
                                    center + vec2(0., step * 0.35),
                                    center + vec2(-step * 0.3, 0.),
                                ],
                                Stroke::new(0.8_f32, style.palette.pattern),
                            ));
                        }
                    }
                }
            } else {
                p.rect_stroke(
                    inset.shrink(5.),
                    1,
                    Stroke::new(1.0_f32, style.palette.pattern),
                    StrokeKind::Inside,
                );
            }
            let badge =
                Rect::from_center_size(r.center(), vec2(r.width() * 0.55, r.width() * 0.66));
            p.rect_filled(badge, 2, Color32::from_rgb(22, 31, 27));
            p.rect_stroke(
                badge,
                2,
                Stroke::new(1.0_f32, style.palette.pattern),
                StrokeKind::Inside,
            );
            let logo =
                Rect::from_center_size(badge.center(), vec2(r.width() * 0.36, r.width() * 0.36));
            p.image(
                style.logo.id(),
                logo,
                Rect::from_min_max(pos2(0., 0.), pos2(1., 1.)),
                Color32::WHITE,
            );
        }
        if style.holographic {
            let clip = p.with_clip_rect(inset);
            let x = inset.left() + style.sheen * inset.width() * 1.6 - inset.width() * 0.3;
            for (offset, color) in [
                (0., Color32::from_rgba_unmultiplied(110, 220, 240, 32)),
                (6., Color32::from_rgba_unmultiplied(230, 140, 210, 28)),
                (12., Color32::from_rgba_unmultiplied(230, 225, 160, 28)),
            ] {
                clip.line_segment(
                    [
                        pos2(x + offset, inset.top()),
                        pos2(x + offset - inset.width() * 0.5, inset.bottom()),
                    ],
                    Stroke::new(r.width() * 0.12, color),
                );
            }
        }
        return;
    }
    let ink = if c.red() { RED } else { INK };
    let font = FontId::monospace(r.width() * 0.175);
    // Horizontal indexes keep both rank and suit visible under a 0.26w overlap.
    p.text(
        r.min + vec2(r.width() * 0.065, r.width() * 0.04),
        Align2::LEFT_TOP,
        c.label(),
        font.clone(),
        ink,
    );
    let shift = if c.rank() == 10 { 0.34 } else { 0.235 };
    suit(
        p,
        r.min + vec2(r.width() * shift, r.width() * 0.135),
        r.width() * 0.14,
        c.suit(),
        ink,
    );
    p.text(
        r.max - vec2(r.width() * 0.065, r.width() * 0.04),
        Align2::RIGHT_BOTTOM,
        c.label(),
        font,
        ink,
    );
    suit(
        p,
        r.max - vec2(r.width() * shift, r.width() * 0.135),
        r.width() * 0.14,
        c.suit(),
        ink,
    );
    let body = Rect::from_min_max(
        r.min + vec2(r.width() * 0.17, r.width() * 0.30),
        r.max - vec2(r.width() * 0.17, r.width() * 0.30),
    );
    if c.rank() > 10 {
        court(p, body, c);
        return;
    }
    let points: Vec<(f32, f32)> = match c.rank() {
        1 => vec![(0.5, 0.5)],
        2 => vec![(0.5, 0.08), (0.5, 0.92)],
        3 => vec![(0.5, 0.08), (0.5, 0.5), (0.5, 0.92)],
        n => {
            let mut pts = vec![(0.2, 0.08), (0.8, 0.08), (0.2, 0.92), (0.8, 0.92)];
            if n == 5 {
                pts.push((0.5, 0.5));
            }
            if (6..=8).contains(&n) {
                pts.extend([(0.2, 0.5), (0.8, 0.5)]);
                if n >= 7 {
                    pts.push((0.5, 0.29));
                }
                if n == 8 {
                    pts.push((0.5, 0.71));
                }
            }
            if n >= 9 {
                pts.extend([(0.2, 0.36), (0.8, 0.36), (0.2, 0.64), (0.8, 0.64)]);
                if n == 9 {
                    pts.push((0.5, 0.5));
                } else {
                    pts.extend([(0.5, 0.22), (0.5, 0.78)]);
                }
            }
            pts
        }
    };
    for (x, y) in points {
        suit(
            p,
            pos2(
                body.left() + body.width() * x,
                body.top() + body.height() * y,
            ),
            r.width()
                * if c.rank() == 1 {
                    0.38
                } else if c.rank() >= 9 {
                    0.165
                } else {
                    0.18
                },
            c.suit(),
            ink,
        );
    }
}
