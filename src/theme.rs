use eframe::egui::Color32;
use std::{env, path::PathBuf};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    pub background: Color32,
    pub foreground: Color32,
    pub accent: Color32,
}
impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(23, 28, 26),
            foreground: Color32::from_rgb(228, 232, 223),
            accent: Color32::from_rgb(179, 203, 146),
        }
    }
}
pub fn parse_color(s: &str) -> Option<Color32> {
    if s.len() != 7 || !s.starts_with('#') || !s[1..].bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let n = u32::from_str_radix(&s[1..], 16).ok()?;
    Some(Color32::from_rgb((n >> 16) as u8, (n >> 8) as u8, n as u8))
}
impl Theme {
    pub fn parse(text: &str) -> Self {
        let mut theme = Self::default();
        if let Ok(t) = text.parse::<toml::Table>() {
            for (key, target) in [
                ("background", &mut theme.background),
                ("foreground", &mut theme.foreground),
                ("accent", &mut theme.accent),
            ] {
                if let Some(c) = t.get(key).and_then(|v| v.as_str()).and_then(parse_color) {
                    *target = c;
                }
            }
        }
        theme
    }
    pub fn load() -> Self {
        let config = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".config")
            });
        let path = config.join("omarchy/current/theme/colors.toml");
        crate::storage::read_bounded(&path, 65536)
            .ok()
            .and_then(|b| String::from_utf8(b).ok())
            .map_or_else(Self::default, |s| Self::parse(&s))
    }
    pub fn square(&self, dark: bool) -> Color32 {
        let (scale, base) = if dark { (0.35, 45.) } else { (0.12, 195.) };
        Color32::from_rgb(
            (self.accent.r() as f32 * scale + base) as u8,
            (self.accent.g() as f32 * scale + base) as u8,
            (self.accent.b() as f32 * scale + base) as u8,
        )
    }
}
