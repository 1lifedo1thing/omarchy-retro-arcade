//! Preserve both artwork sets and recolour only their isolated accents.
use crate::preferences::PieceStyle;
use eframe::egui::{Color32, Context, ImageSource};
use shakmaty::{Color, Role};
use std::sync::Arc;
const CHISEL: [(&str, &str); 12] = [
    (
        "bytes://chisel-wp.svg",
        include_str!("../assets/pieces/wp.svg"),
    ),
    (
        "bytes://chisel-wn.svg",
        include_str!("../assets/pieces/wn.svg"),
    ),
    (
        "bytes://chisel-wb.svg",
        include_str!("../assets/pieces/wb.svg"),
    ),
    (
        "bytes://chisel-wr.svg",
        include_str!("../assets/pieces/wr.svg"),
    ),
    (
        "bytes://chisel-wq.svg",
        include_str!("../assets/pieces/wq.svg"),
    ),
    (
        "bytes://chisel-wk.svg",
        include_str!("../assets/pieces/wk.svg"),
    ),
    (
        "bytes://chisel-bp.svg",
        include_str!("../assets/pieces/bp.svg"),
    ),
    (
        "bytes://chisel-bn.svg",
        include_str!("../assets/pieces/bn.svg"),
    ),
    (
        "bytes://chisel-bb.svg",
        include_str!("../assets/pieces/bb.svg"),
    ),
    (
        "bytes://chisel-br.svg",
        include_str!("../assets/pieces/br.svg"),
    ),
    (
        "bytes://chisel-bq.svg",
        include_str!("../assets/pieces/bq.svg"),
    ),
    (
        "bytes://chisel-bk.svg",
        include_str!("../assets/pieces/bk.svg"),
    ),
];
const AFTER_HOURS: [(&str, &str); 12] = [
    (
        "bytes://after-hours-wp.svg",
        include_str!("../assets/pieces/after-hours/wp.svg"),
    ),
    (
        "bytes://after-hours-wn.svg",
        include_str!("../assets/pieces/after-hours/wn.svg"),
    ),
    (
        "bytes://after-hours-wb.svg",
        include_str!("../assets/pieces/after-hours/wb.svg"),
    ),
    (
        "bytes://after-hours-wr.svg",
        include_str!("../assets/pieces/after-hours/wr.svg"),
    ),
    (
        "bytes://after-hours-wq.svg",
        include_str!("../assets/pieces/after-hours/wq.svg"),
    ),
    (
        "bytes://after-hours-wk.svg",
        include_str!("../assets/pieces/after-hours/wk.svg"),
    ),
    (
        "bytes://after-hours-bp.svg",
        include_str!("../assets/pieces/after-hours/bp.svg"),
    ),
    (
        "bytes://after-hours-bn.svg",
        include_str!("../assets/pieces/after-hours/bn.svg"),
    ),
    (
        "bytes://after-hours-bb.svg",
        include_str!("../assets/pieces/after-hours/bb.svg"),
    ),
    (
        "bytes://after-hours-br.svg",
        include_str!("../assets/pieces/after-hours/br.svg"),
    ),
    (
        "bytes://after-hours-bq.svg",
        include_str!("../assets/pieces/after-hours/bq.svg"),
    ),
    (
        "bytes://after-hours-bk.svg",
        include_str!("../assets/pieces/after-hours/bk.svg"),
    ),
];
fn art(style: PieceStyle) -> &'static [(&'static str, &'static str); 12] {
    match style {
        PieceStyle::AfterHours => &AFTER_HOURS,
        PieceStyle::Chisel => &CHISEL,
    }
}
pub struct Pieces {
    accent: Option<Color32>,
    style: PieceStyle,
    bytes: [Arc<[u8]>; 12],
}
impl Default for Pieces {
    fn default() -> Self {
        Self {
            accent: None,
            style: PieceStyle::default(),
            bytes: std::array::from_fn(|_| Arc::from([])),
        }
    }
}
impl Pieces {
    pub fn refresh(&mut self, ctx: &Context, accent: Color32, style: PieceStyle) {
        if self.accent == Some(accent) && self.style == style {
            return;
        }
        let colour = format!("#{:02x}{:02x}{:02x}", accent.r(), accent.g(), accent.b());
        for (i, (uri, svg)) in art(style).iter().enumerate() {
            ctx.forget_image(uri);
            self.bytes[i] = Arc::from(svg.replace("#b3cb92", &colour).into_bytes());
        }
        self.accent = Some(accent);
        self.style = style;
    }
    pub fn image(&self, color: Color, role: Role) -> ImageSource<'static> {
        let i = if color == Color::Black { 6 } else { 0 }
            + match role {
                Role::Pawn => 0,
                Role::Knight => 1,
                Role::Bishop => 2,
                Role::Rook => 3,
                Role::Queen => 4,
                Role::King => 5,
            };
        ImageSource::Bytes {
            uri: art(self.style)[i].0.into(),
            bytes: self.bytes[i].clone().into(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recolouring_preserves_geometry_and_refreshes_all_pieces() {
        let ctx = Context::default();
        let mut pieces = Pieces::default();
        for style in [
            PieceStyle::AfterHours,
            PieceStyle::Chisel,
            PieceStyle::AfterHours,
        ] {
            for accent in [
                Color32::from_rgb(39, 109, 207),
                Color32::from_rgb(225, 90, 110),
            ] {
                pieces.refresh(&ctx, accent, style);
                let token = format!("#{:02x}{:02x}{:02x}", accent.r(), accent.g(), accent.b());
                for (i, (_, original)) in art(style).iter().enumerate() {
                    assert_eq!(original.matches("#b3cb92").count(), 1);
                    let actual = std::str::from_utf8(&pieces.bytes[i]).unwrap();
                    assert_eq!(actual.replace(&token, "#b3cb92"), *original);
                    assert!(actual.contains("viewBox=\"0 0 128 128\""));
                }
            }
        }
    }
}
