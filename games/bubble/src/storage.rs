use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
    pub version: u32,
    pub unlocked: usize,
    pub selected: usize,
    pub best: Vec<u32>,
    pub sound: bool,
    pub introduced: bool,
}
impl Default for Progress {
    fn default() -> Self {
        Self {
            version: 1,
            unlocked: 0,
            selected: 0,
            best: vec![0; 20],
            sound: true,
            introduced: false,
        }
    }
}
pub fn state_path() -> PathBuf {
    std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/state")
        })
        .join("omarchy-retro-arcade/bubble.json")
}
pub fn load(path: &Path, count: usize) -> Result<Progress, String> {
    let mut bytes = vec![];
    match fs::File::open(path) {
        Ok(file) => {
            file.take(65537)
                .read_to_end(&mut bytes)
                .map_err(|e| e.to_string())?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Progress {
                best: vec![0; count],
                ..Default::default()
            })
        }
        Err(e) => return Err(e.to_string()),
    };
    if bytes.len() > 65536 {
        return Err("Bubble save is too large".into());
    }
    let mut p: Progress = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if p.version != 1 || p.unlocked >= count || p.selected > p.unlocked || p.best.len() > count {
        return Err("Unsupported or invalid Bubble progress; original save retained".into());
    }
    p.best.resize(count, 0);
    Ok(p)
}
pub fn save(path: &Path, p: &Progress) -> Result<(), String> {
    let dir = path.parent().ok_or("Missing save directory")?;
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut file = tempfile::NamedTempFile::new_in(dir).map_err(|e| e.to_string())?;
    file.write_all(&serde_json::to_vec_pretty(p).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    file.as_file().sync_all().map_err(|e| e.to_string())?;
    file.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}
