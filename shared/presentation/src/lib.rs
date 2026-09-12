//! Shared materials and controls. No game state, timing or input lives here.
use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

pub const IVORY: Color32 = Color32::from_rgb(235, 230, 210);
pub const BRASS: Color32 = Color32::from_rgb(174, 145, 89);
pub const INK: Color32 = Color32::from_rgb(12, 17, 16);
pub const AMBER: Color32 = Color32::from_rgb(238, 183, 98);

/// Apply the collection's geometry while retaining the active game's palette.
pub fn apply(ctx: &egui::Context) {
    ctx.style_mut(|s| {
        s.spacing.button_padding = Vec2::new(12., 7.);
        s.spacing.item_spacing = Vec2::new(10., 8.);
        s.spacing.interact_size.y = 30.;
        s.visuals.window_corner_radius = 2.into();
        s.visuals.menu_corner_radius = 2.into();
        s.visuals.window_stroke = Stroke::new(1_f32, BRASS.gamma_multiply(0.7));
        let light = !s.visuals.dark_mode;
        for (state, strength) in [
            (&mut s.visuals.widgets.inactive, 0.38),
            (&mut s.visuals.widgets.hovered, 0.75),
            (&mut s.visuals.widgets.active, 1.),
        ] {
            state.corner_radius = 2.into();
            state.bg_stroke = Stroke::new(
                1_f32,
                if light {
                    Color32::from_gray(140)
                } else {
                    BRASS.gamma_multiply(strength)
                },
            );
        }
        s.text_styles
            .insert(egui::TextStyle::Button, egui::FontId::proportional(13.));
    });
}

/// The bitmap is decoded once per context; all games share its GPU allocation.
pub fn backdrop(ui: &egui::Ui) {
    let rect = ui.max_rect();
    if ui.visuals().dark_mode {
        let id = egui::Id::new("arcade-cabinet-texture");
        let tex = ui.ctx().data_mut(|d| d.get_temp::<egui::TextureHandle>(id));
        let texture = tex.unwrap_or_else(|| {
            let img = image::load_from_memory(include_bytes!("../assets/cabinet.png"))
                .expect("bundled cabinet PNG")
                .into_rgba8();
            let size = [img.width() as usize, img.height() as usize];
            let texture = ui.ctx().load_texture(
                "Arcade cabinet",
                egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw()),
                egui::TextureOptions::LINEAR,
            );
            ui.ctx().data_mut(|d| d.insert_temp(id, texture.clone()));
            texture
        });
        // Crop the artwork centrally rather than stretching the machined circles.
        let target = rect.aspect_ratio();
        let source = 1.5;
        let uv = if target > source {
            let h = source / target;
            Rect::from_min_max(Pos2::new(0., (1. - h) / 2.), Pos2::new(1., (1. + h) / 2.))
        } else {
            let w = target / source;
            Rect::from_min_max(Pos2::new((1. - w) / 2., 0.), Pos2::new((1. + w) / 2., 1.))
        };
        ui.painter()
            .image(texture.id(), rect, uv, Color32::from_gray(135));
    } else {
        ui.painter().rect_filled(rect, 0., ui.visuals().panel_fill);
        grain(ui.painter(), rect, Color32::from_black_alpha(8), 5.);
        ui.painter().rect_stroke(
            rect.shrink(8.),
            0.,
            Stroke::new(1_f32, BRASS.gamma_multiply(0.4)),
            egui::StrokeKind::Inside,
        );
    }
}

pub fn grain(p: &egui::Painter, r: Rect, color: Color32, spacing: f32) {
    let lines = (r.height() / spacing).min(360.) as usize;
    for i in 0..lines {
        let y = r.top() + i as f32 * spacing;
        p.line_segment(
            [Pos2::new(r.left(), y), Pos2::new(r.right(), y)],
            Stroke::new(0.45_f32, color),
        );
    }
}

/// Recessed playing surface, with lit bevels outside the collision geometry.
pub fn bezel(p: &egui::Painter, r: Rect, accent: Color32) {
    p.rect_filled(
        r.expand(13.).translate(Vec2::new(0., 4.)),
        2.,
        Color32::from_black_alpha(100),
    );
    p.rect_filled(r.expand(10.), 2., Color32::from_rgb(47, 50, 43));
    p.rect_stroke(
        r.expand(10.),
        2.,
        Stroke::new(1_f32, BRASS.gamma_multiply(0.7)),
        egui::StrokeKind::Inside,
    );
    p.rect_filled(r.expand(7.), 1., Color32::from_rgb(21, 26, 23));
    p.rect_stroke(
        r.expand(5.),
        1.,
        Stroke::new(1_f32, accent.gamma_multiply(0.45)),
        egui::StrokeKind::Inside,
    );
    p.rect_filled(r.expand(2.), 0., Color32::from_rgb(5, 10, 10));
    p.line_segment(
        [
            r.left_top() - Vec2::new(5., 5.),
            r.right_top() + Vec2::new(5., -5.),
        ],
        Stroke::new(1_f32, Color32::from_white_alpha(60)),
    );
    for c in [
        r.left_top(),
        r.right_top(),
        r.left_bottom(),
        r.right_bottom(),
    ] {
        let dir = (c - r.center()).normalized();
        let c = c + dir * 9.;
        p.circle_filled(c, 2., BRASS.gamma_multiply(0.7));
        p.line_segment(
            [c - Vec2::new(1., 0.), c + Vec2::new(1., 0.)],
            Stroke::new(0.7_f32, INK),
        );
    }
}

pub fn tile(p: &egui::Painter, r: Rect, color: Color32, raised: bool) {
    let edge = (r.width() * 0.10).clamp(1., 5.);
    p.rect_filled(
        r.translate(Vec2::new(0., edge * 0.6)),
        1.,
        Color32::from_black_alpha(140),
    );
    p.rect_filled(r, 1., color.gamma_multiply(0.45));
    let face = r.shrink(edge);
    p.add(egui::Shape::convex_polygon(
        vec![
            r.left_top(),
            r.right_top(),
            face.right_top(),
            face.left_top(),
        ],
        color.lerp_to_gamma(IVORY, if raised { 0.5 } else { 0.25 }),
        Stroke::NONE,
    ));
    p.add(egui::Shape::convex_polygon(
        vec![
            r.left_top(),
            face.left_top(),
            face.left_bottom(),
            r.left_bottom(),
        ],
        color.lerp_to_gamma(IVORY, 0.18),
        Stroke::NONE,
    ));
    p.rect_filled(face, 0., color);
    p.rect_stroke(
        face,
        0.,
        Stroke::new(0.6_f32, Color32::from_white_alpha(45)),
        egui::StrokeKind::Inside,
    );
    p.line_segment(
        [
            face.left_top() + Vec2::new(2., 2.),
            face.right_top() + Vec2::new(-2., 2.),
        ],
        Stroke::new(0.8_f32, Color32::from_white_alpha(70)),
    );
}

pub fn glass_ball(p: &egui::Painter, c: Pos2, r: f32, color: Color32) {
    p.circle_filled(
        c + Vec2::new(r * 0.09, r * 0.18),
        r,
        Color32::from_black_alpha(110),
    );
    for i in 0..14 {
        let t = i as f32 / 14.;
        p.circle_filled(
            c + Vec2::new(-r * 0.12 * t, -r * 0.18 * t),
            r * (1. - t * 0.5),
            color.gamma_multiply(0.4 + t * 0.6),
        );
    }
    p.circle_stroke(
        c,
        r * 0.98,
        Stroke::new((r * 0.035).max(0.6), color.lerp_to_gamma(IVORY, 0.48)),
    );
    p.circle_filled(
        c + Vec2::new(-r * 0.28, -r * 0.42),
        r * 0.14,
        Color32::from_white_alpha(210),
    );
    p.circle_filled(
        c + Vec2::new(r * 0.40, r * 0.30),
        r * 0.08,
        color.lerp_to_gamma(IVORY, 0.4),
    );
}
