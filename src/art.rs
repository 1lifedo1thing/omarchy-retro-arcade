//! Replaceable presentation only: no hitboxes, scores or formation slots live here.
use eframe::egui::{self, Color32, Pos2, Rect, TextureHandle, Vec2};
use std::path::PathBuf;
const BUILTIN: &[u8] = include_bytes!("../assets/orbit/atlas.png");
pub struct Art {
    texture: Option<TextureHandle>,
    background: Option<TextureHandle>,
    bytes: Vec<u8>,
    accent: Color32,
    pub message: String,
}
impl Default for Art {
    fn default() -> Self {
        Self {
            texture: None,
            background: None,
            bytes: BUILTIN.to_vec(),
            accent: Color32::TRANSPARENT,
            message: String::new(),
        }
    }
}
pub fn path() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/share")
        })
        .join("omarchy-invaders/orbit/atlas.png")
}
fn decode(bytes: &[u8]) -> Result<image::RgbaImage, String> {
    if bytes.len() > 1024 * 1024 {
        return Err("Atlas exceeds 1 MB".into());
    }
    // Check dimensions before allocating a decoded image.
    let reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| e.to_string())?;
    let dimensions = reader.into_dimensions().map_err(|e| e.to_string())?;
    if dimensions != (192, 128) {
        return Err("Use a 192 × 128 PNG with 32 × 32 cells".into());
    }
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .map_err(|e| e.to_string())?
        .to_rgba8();
    if !img.pixels().any(|p| p[3] == 0) {
        return Err("Atlas must have transparent background pixels".into());
    }
    Ok(img)
}
impl Art {
    pub fn editable_copy(&mut self) {
        let p = path();
        let result = (|| -> std::io::Result<()> {
            std::fs::create_dir_all(p.parent().unwrap())?;
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&p)?;
            f.write_all(BUILTIN)
        })();
        self.message = match result {
            Ok(()) => "Editable copy created. Edit the PNG, then Reload artwork.".into(),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                "Your existing artwork was preserved.".into()
            }
            Err(e) => e.to_string(),
        };
    }
    pub fn reload(&mut self) -> bool {
        match crate::storage::read_bounded(&path(), 1024 * 1024)
            .map_err(|e| e.to_string())
            .and_then(|b| {
                decode(&b)?;
                Ok(b)
            }) {
            Ok(bytes) => {
                self.bytes = bytes;
                self.texture = None;
                self.message = "Artwork reloaded. Run and scores preserved.".into();
                true
            }
            Err(e) => {
                self.message = format!("Kept current artwork: {e}");
                false
            }
        }
    }
    pub fn builtin(&mut self) {
        self.bytes = BUILTIN.to_vec();
        self.texture = None;
        self.message = "Built-in Orbit artwork restored.".into();
    }
    pub fn prepare(&mut self, ctx: &egui::Context, accent: Color32) {
        if self.background.is_none() {
            let img = image::load_from_memory(include_bytes!("../assets/orbit/background.png"))
                .expect("embedded background")
                .to_rgba8();
            self.background = Some(ctx.load_texture(
                "orbit-background",
                egui::ColorImage::from_rgba_unmultiplied([200, 175], img.as_raw()),
                egui::TextureOptions::NEAREST,
            ));
        }
        if self.texture.is_some() && self.accent == accent {
            return;
        }
        let mut img = decode(&self.bytes).expect("validated atlas");
        for p in img.pixels_mut() {
            if p[0] == 179 && p[1] == 203 && p[2] == 146 {
                p[0] = accent.r();
                p[1] = accent.g();
                p[2] = accent.b();
            }
        }
        self.texture = Some(ctx.load_texture(
            "orbit-atlas",
            egui::ColorImage::from_rgba_unmultiplied([192, 128], img.as_raw()),
            egui::TextureOptions::NEAREST,
        ));
        self.accent = accent;
    }
    pub fn backdrop(&self, p: &egui::Painter, r: Rect) {
        if let Some(t) = &self.background {
            p.image(
                t.id(),
                r,
                Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(1., 1.)),
                Color32::WHITE,
            );
        }
    }
    pub fn draw(&self, p: &egui::Painter, center: Pos2, cell: usize, size: f32, tint: Color32) {
        let Some(t) = &self.texture else {
            return;
        };
        let x = (cell % 6) as f32 / 6.;
        let y = (cell / 6) as f32 / 4.;
        p.image(
            t.id(),
            Rect::from_center_size(center, Vec2::splat(size)),
            Rect::from_min_max(egui::pos2(x, y), egui::pos2(x + 1. / 6., y + 1. / 4.)),
            tint,
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_builtin_and_rejects_bad_replacements() {
        assert!(decode(BUILTIN).is_ok());
        assert!(decode(b"not png").is_err());
        assert!(decode(&vec![0; 1024 * 1024 + 1]).is_err());
    }
}
