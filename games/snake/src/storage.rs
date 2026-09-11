use crate::engine::*;
use omarchy_chess::storage::{atomic_write, read_bounded, stamp};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub speed: Speed,
    pub keys: [String; 4],
    pub audio: bool,
    pub reduced_motion: bool,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            speed: Speed::Normal,
            keys: ["ArrowUp", "ArrowRight", "ArrowDown", "ArrowLeft"].map(str::to_owned),
            audio: false,
            reduced_motion: false,
        }
    }
}
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Records {
    pub version: u32,
    pub best: [u32; 3],
    pub preferences: Preferences,
}
impl Records {
    pub fn record(&mut self, sim: &Sim) {
        let best = &mut self.best[sim.speed.index()];
        *best = (*best).max(sim.score);
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Session {
    version: u32,
    sim: Sim,
}
pub fn state_dir() -> PathBuf {
    std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/state")
        })
        .join("omarchy-snake")
}
fn reject(dir: &Path, filename: &str) -> String {
    let path = dir.join(filename);
    let archive = dir.join("archive");
    let kept = std::fs::create_dir_all(&archive)
        .and_then(|_| std::fs::rename(&path, archive.join(format!("{}-{filename}", stamp()))))
        .is_ok();
    format!(
        "Snake {filename} could not be read or resumed. {}",
        if kept {
            "The original was kept in Snake's recovery archive."
        } else {
            "The original could not be archived; check file permissions."
        }
    )
}
pub fn load(dir: &Path) -> (Records, Option<Sim>, Option<String>) {
    let mut error = None;
    let records = if dir.join("records.json").exists() {
        match read_bounded(&dir.join("records.json"), 8192)
            .ok()
            .and_then(|b| serde_json::from_slice::<Records>(&b).ok())
            .filter(|r| r.version == 1 && r.best.iter().all(|s| *s <= 4760 && s % FOOD_POINTS == 0))
        {
            Some(r) => r,
            None => {
                error = Some(reject(dir, "records.json"));
                Records::default()
            }
        }
    } else {
        Records::default()
    };
    let sim = if dir.join("session.json").exists() {
        match read_bounded(&dir.join("session.json"), 16384)
            .ok()
            .and_then(|b| serde_json::from_slice::<Session>(&b).ok())
            .filter(|s| s.version == 1 && s.sim.valid() && s.sim.outcome == Outcome::Playing)
        {
            Some(s) => Some(s.sim),
            None => {
                error = Some(reject(dir, "session.json"));
                None
            }
        }
    } else {
        None
    };
    (records, sim, error)
}
pub fn save(dir: &Path, records: &mut Records, sim: Option<&Sim>) -> Result<(), String> {
    records.version = 1;
    atomic_write(
        &dir.join("records.json"),
        &serde_json::to_vec_pretty(records).map_err(|e| e.to_string())?,
    )?;
    if let Some(sim) = sim.filter(|s| s.outcome == Outcome::Playing) {
        atomic_write(
            &dir.join("session.json"),
            &serde_json::to_vec_pretty(&Session {
                version: 1,
                sim: sim.clone(),
            })
            .map_err(|e| e.to_string())?,
        )
    } else {
        match std::fs::remove_file(dir.join("session.json")) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
