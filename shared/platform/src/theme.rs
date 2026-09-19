use ecolor::Color32;
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
        let home = PathBuf::from(env::var_os("HOME").unwrap_or_default());
        let state = env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".local/state"));
        let config = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"));
        Self::load_from(&[
            state.join("omarchy/current/theme/colors.toml"),
            home.join(".local/state/omarchy/current/theme/colors.toml"),
            config.join("omarchy/current/theme/colors.toml"),
        ])
        .unwrap_or_default()
    }
    pub fn load_from(paths: &[PathBuf]) -> Option<Self> {
        paths.iter().find_map(|path| {
            let bytes = crate::storage::read_bounded(path, 65536).ok()?;
            let text = String::from_utf8(bytes).ok()?;
            let table = text.parse::<toml::Table>().ok()?;
            for key in ["background", "foreground"] {
                parse_color(table.get(key)?.as_str()?)?;
            }
            Some(Self::parse(&text))
        })
    }
    pub fn light(&self) -> bool {
        self.background.r() as u32 + self.background.g() as u32 + self.background.b() as u32 > 420
    }
    pub fn accent_text(&self) -> Color32 {
        if self.accent.r() as u32 * 299
            + self.accent.g() as u32 * 587
            + self.accent.b() as u32 * 114
            > 140000
        {
            Color32::BLACK
        } else {
            Color32::WHITE
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_primary_palette_falls_back_and_replacement_is_reloaded() {
        let dir = tempfile::tempdir().unwrap();
        let primary = dir.path().join("current.toml");
        let fallback = dir.path().join("fallback.toml");
        std::fs::write(&primary, "background = 'invalid'").unwrap();
        std::fs::write(
            &fallback,
            "background = '#ffffff'\nforeground = '#111111'\naccent = '#abcdef'",
        )
        .unwrap();
        let paths = [primary.clone(), fallback];
        let light = Theme::load_from(&paths).unwrap();
        assert!(light.light());
        assert_eq!(light.accent, parse_color("#abcdef").unwrap());
        crate::storage::atomic_write(
            &primary,
            b"background = '#000000'\nforeground = '#eeeeee'\naccent = '#123456'",
        )
        .unwrap();
        let dark = Theme::load_from(&paths).unwrap();
        assert!(!dark.light());
        assert_eq!(dark.accent, parse_color("#123456").unwrap());
    }
}
