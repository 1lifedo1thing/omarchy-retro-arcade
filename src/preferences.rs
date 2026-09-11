use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Preferences {
    pub version: u32,
    pub sound: bool,
    pub follow_omarchy: bool,
    pub engine_path: Option<PathBuf>,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            version: 1,
            sound: false,
            follow_omarchy: true,
            engine_path: None,
        }
    }
}
impl Preferences {
    pub fn load(dir: &Path) -> Result<Self, String> {
        let path = dir.join("settings.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let p: Self = serde_json::from_slice(&crate::storage::read_bounded(&path, 16384)?)
            .map_err(|e| e.to_string())?;
        if p.version != 1 {
            return Err("Unsupported settings version".into());
        }
        Ok(p)
    }
    pub fn save(&self, dir: &Path) -> Result<(), String> {
        crate::storage::atomic_write(
            &dir.join("settings.json"),
            &serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?,
        )
    }
}
