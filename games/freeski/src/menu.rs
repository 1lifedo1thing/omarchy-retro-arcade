//! FreeSki menu components. Presentation only; callers own every action.
use eframe::egui::{self, Color32, FontId, RichText, Stroke, Vec2};
use omarchy_chess::theme::Theme;

pub fn show(ctx: &egui::Context, id: &str, theme: &Theme, content: impl FnOnce(&mut egui::Ui)) {
    let width = 480_f32.min((ctx.screen_rect().width() - 96.).max(240.));
    egui::Modal::new(egui::Id::new(id))
        .backdrop_color(Color32::from_black_alpha(if theme.light() {
            110
        } else {
            165
        }))
        .frame(
            egui::Frame::new()
                .fill(theme.background)
                .stroke(Stroke::new(1_f32, theme.foreground.gamma_multiply(0.20)))
                .corner_radius(16)
                .inner_margin(28)
                .shadow(egui::epaint::Shadow {
                    offset: [0, 14],
                    blur: 40,
                    spread: 0,
                    color: Color32::from_black_alpha(90),
                }),
        )
        .show(ctx, |ui| {
            style_controls(ui, theme);
            ui.set_width(width);
            ui.set_max_height((ctx.screen_rect().height() - 112.).max(220.));
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            ui.spacing_mut().item_spacing = Vec2::new(12., 8.);
            ui.spacing_mut().button_padding = Vec2::new(16., 12.);
            ui.spacing_mut().interact_size.y = 44.;
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Body, FontId::proportional(16.));
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Button, FontId::proportional(16.));
            egui::ScrollArea::vertical()
                .max_height((ctx.screen_rect().height() - 112.).max(220.))
                .auto_shrink([false, true])
                .show(ui, content);
        });
}
pub fn heading(ui: &mut egui::Ui, theme: &Theme, eyebrow: &str, title: &str, subtitle: &str) {
    ui.label(
        RichText::new(eyebrow)
            .monospace()
            .size(11.)
            .color(theme.accent),
    );
    ui.add_space(2.);
    ui.label(
        RichText::new(title)
            .size(28.)
            .strong()
            .color(theme.foreground.lerp_to_gamma(
                if theme.light() {
                    Color32::BLACK
                } else {
                    Color32::WHITE
                },
                0.65,
            )),
    );
    if !subtitle.is_empty() {
        ui.label(RichText::new(subtitle).size(15.).color(muted(ui)));
    }
    ui.add_space(8.);
}
pub fn muted(ui: &egui::Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(163, 174, 182)
    } else {
        Color32::from_rgb(87, 101, 106)
    }
}
pub fn section(ui: &mut egui::Ui, text: &str) {
    ui.add_space(4.);
    ui.label(RichText::new(text).monospace().size(11.).color(muted(ui)));
}
pub fn action(ui: &mut egui::Ui, theme: &Theme, label: &str, primary: bool) -> egui::Response {
    let (fill, text, stroke) = if primary {
        (theme.accent, theme.accent_text(), Stroke::NONE)
    } else {
        (
            theme.background.lerp_to_gamma(theme.foreground, 0.045),
            theme.foreground,
            Stroke::new(1_f32, theme.foreground.gamma_multiply(0.16)),
        )
    };
    ui.add_sized(
        [ui.available_width(), 44.],
        egui::Button::new(RichText::new(label).size(16.).color(text))
            .fill(fill)
            .stroke(stroke)
            .corner_radius(8),
    )
}
pub fn pair(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui, &mut egui::Ui)) {
    ui.columns(2, |columns| {
        let (left, right) = columns.split_at_mut(1);
        content(&mut left[0], &mut right[0]);
    });
}
pub fn stats(ui: &mut egui::Ui, values: &[(&str, String)]) {
    ui.columns(values.len(), |columns| {
        for (column, (label, value)) in columns.iter_mut().zip(values) {
            egui::Frame::new()
                .fill(surface(column))
                .corner_radius(8)
                .inner_margin(14)
                .show(column, |ui| {
                    ui.set_min_width((ui.available_width() - 1.).max(0.));
                    ui.label(RichText::new(*label).monospace().size(11.).color(muted(ui)));
                    ui.label(RichText::new(value).size(25.).strong());
                });
        }
    });
    ui.add_space(8.);
}
pub fn setting(ui: &mut egui::Ui, value: &mut bool, title: &str, description: &str) -> bool {
    let before = *value;
    egui::Frame::new()
        .fill(surface(ui))
        .corner_radius(8)
        .inner_margin(16)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.checkbox(value, RichText::new(title).size(16.));
            ui.label(RichText::new(description).size(14.).color(muted(ui)));
        });
    before != *value
}
pub fn control(ui: &mut egui::Ui, title: &str, keys: &str, detail: &str) {
    ui.scope(|ui| {
        ui.spacing_mut().interact_size.y = 22.;
        ui.spacing_mut().item_spacing.y = 4.;
        ui.horizontal(|ui| {
            ui.add_sized(
                [100., 22.],
                egui::Label::new(RichText::new(title).strong()).halign(egui::Align::Min),
            );
            ui.label(RichText::new(keys).monospace().size(14.));
        });
        ui.label(RichText::new(detail).size(14.).color(muted(ui)));
    });
    ui.add_space(8.);
}

pub fn style_controls(ui: &mut egui::Ui, theme: &Theme) {
    let visuals = ui.visuals_mut();
    visuals.selection.stroke.color = theme.accent_text();
    for (widget, tint) in [
        (&mut visuals.widgets.inactive, 0.045),
        (&mut visuals.widgets.hovered, 0.10),
        (&mut visuals.widgets.active, 0.15),
    ] {
        widget.weak_bg_fill = theme.background.lerp_to_gamma(theme.foreground, tint);
        widget.bg_fill = widget.weak_bg_fill;
        widget.bg_stroke = Stroke::new(1_f32, theme.foreground.gamma_multiply(0.16));
        widget.fg_stroke.color = theme.foreground;
        widget.corner_radius = 8.into();
    }
}

fn surface(ui: &egui::Ui) -> Color32 {
    ui.visuals()
        .window_fill()
        .lerp_to_gamma(ui.visuals().text_color(), 0.055)
}
