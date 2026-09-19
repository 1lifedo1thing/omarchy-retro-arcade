use eframe::egui::Key;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub version: u32,
    pub humans: usize,
    pub bots: usize,
    pub arena: usize,
    pub sound: bool,
    pub reduced_motion: bool,
    pub bindings: [[String; 5]; 2],
    pub matches: u32,
    pub wins: [u32; 4],
    pub seen_help: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            version: 1,
            humans: 1,
            bots: 3,
            arena: 0,
            sound: false,
            reduced_motion: false,
            bindings: [
                ["W", "S", "A", "D", "Space"].map(String::from),
                ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight", "Enter"].map(String::from),
            ],
            matches: 0,
            wins: [0; 4],
            seen_help: false,
        }
    }
}
impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1
            || !(1..=2).contains(&self.humans)
            || !(2..=4).contains(&(self.humans + self.bots))
            || self.arena >= 3
        {
            return Err("Unsupported Blast preferences.".into());
        }
        let mut used = Vec::new();
        for name in self.bindings.iter().flatten() {
            let key = Key::from_name(name).ok_or("Unknown control key")?;
            if matches!(key, Key::Escape | Key::Tab | Key::F1 | Key::F11) {
                return Err("Escape, Tab, F1 and F11 are reserved.".into());
            }
            if used.contains(&key) {
                return Err(format!(
                    "{} is already assigned. Choose a different key.",
                    key.name()
                ));
            }
            used.push(key);
        }
        Ok(())
    }
    pub fn load(dir: &Path) -> Result<Self, String> {
        let path = dir.join("blast.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let value: Self =
            serde_json::from_slice(&arcade_platform::storage::read_bounded(&path, 16384)?)
                .map_err(|e| e.to_string())?;
        value.validate()?;
        Ok(value)
    }
    pub fn save(&self, dir: &Path) -> Result<(), String> {
        self.validate()?;
        arcade_platform::storage::atomic_write(
            &dir.join("blast.json"),
            &serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?,
        )
    }
}
pub fn state_dir() -> PathBuf {
    std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/state")
        })
        .join("omarchy-retro-arcade")
}
