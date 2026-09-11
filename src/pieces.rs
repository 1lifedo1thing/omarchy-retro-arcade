//! Chisel artwork: preserve all facets and recolour only the isolated inlay.
use eframe::egui::{Color32, Context, ImageSource};
use shakmaty::{Color, Role};
use std::sync::Arc;
const ART: [(&str, &str); 12] = [
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
pub struct Pieces {
    accent: Option<Color32>,
    bytes: [Arc<[u8]>; 12],
}
impl Default for Pieces {
    fn default() -> Self {
        Self {
            accent: None,
            bytes: std::array::from_fn(|_| Arc::from([])),
        }
    }
}
impl Pieces {
    pub fn refresh(&mut self, ctx: &Context, accent: Color32) {
        if self.accent == Some(accent) {
            return;
        }
        let colour = format!("#{:02x}{:02x}{:02x}", accent.r(), accent.g(), accent.b());
        for (i, (uri, svg)) in ART.iter().enumerate() {
            ctx.forget_image(uri);
            self.bytes[i] = Arc::from(svg.replace("#b3cb92", &colour).into_bytes());
        }
        self.accent = Some(accent);
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
            uri: ART[i].0.into(),
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
        for accent in [
            Color32::from_rgb(39, 109, 207),
            Color32::from_rgb(225, 90, 110),
        ] {
            pieces.refresh(&ctx, accent);
            let token = format!("#{:02x}{:02x}{:02x}", accent.r(), accent.g(), accent.b());
            for (i, (_, original)) in ART.iter().enumerate() {
                assert_eq!(original.matches("#b3cb92").count(), 1);
                let actual = std::str::from_utf8(&pieces.bytes[i]).unwrap();
                assert_eq!(actual.replace(&token, "#b3cb92"), *original);
                assert!(actual.contains("viewBox=\"0 0 128 128\""));
            }
        }
    }
}
