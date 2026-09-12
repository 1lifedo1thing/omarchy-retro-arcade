//! One private atomic document; records/preferences are independent of the attempt.
use crate::{
    engine::{Phase, Sim, RULES_VERSION},
    world::{self, Obstacle},
};
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "ui"))]
use std::io::Write;
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};
const LIMIT: u64 = 64 * 1024;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Save {
    pub version: u32,
    pub rules_version: u32,
    pub course_version: u32,
    pub run: Sim,
    pub best_distance: f64,
    pub completions: u64,
    pub result_recorded: bool,
    pub reduced_effects: bool,
}
impl Default for Save {
    fn default() -> Self {
        Self {
            version: 1,
            rules_version: RULES_VERSION,
            course_version: world::COURSE_VERSION,
            run: Sim::default(),
            best_distance: 0.,
            completions: 0,
            result_recorded: false,
            reduced_effects: false,
        }
    }
}
impl Save {
    pub fn valid(&self, obstacles: &[Obstacle]) -> bool {
        self.version == 1
            && self.rules_version == RULES_VERSION
            && self.course_version == world::COURSE_VERSION
            && self.run.valid(obstacles)
            && self.best_distance.is_finite()
            && (0. ..=world::FINISH).contains(&self.best_distance)
            && (!self.result_recorded
                || (self.run.ended() && self.best_distance >= self.run.distance))
            && (self.completions == 0 || self.best_distance == world::FINISH)
    }
    pub fn record_result(&mut self) {
        if self.run.ended() && !self.result_recorded {
            self.best_distance = self.best_distance.max(self.run.distance);
            if self.run.phase == Phase::Finished {
                self.completions = self.completions.saturating_add(1);
            }
            self.result_recorded = true;
        }
    }
    pub fn restart(&mut self) {
        self.run = Sim::default();
        self.result_recorded = false;
    }
}
pub fn path() -> Result<PathBuf, String> {
    std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))
        .map(|p| p.join("omarchy-retro-arcade/freeski.json"))
        .ok_or_else(|| "No state directory is available.".into())
}
pub fn load(path: &Path, obstacles: &[Obstacle]) -> Result<Save, String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Save::default()),
        Err(e) => return Err(e.to_string()),
    };
    let mut bytes = vec![];
    file.take(LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > LIMIT {
        return Err("FreeSki save exceeds 64 KiB. Original retained.".into());
    }
    let mut save: Save = serde_json::from_slice(&bytes)
        .map_err(|_| "Unreadable FreeSki save. Original retained.".to_string())?;
    if !save.valid(obstacles) {
        return Err("Incompatible or invalid FreeSki save. Original retained.".into());
    }
    save.run.pause();
    save.record_result();
    Ok(save)
}
pub fn write(path: &Path, state: &Save, obstacles: &[Obstacle]) -> Result<(), String> {
    if !state.valid(obstacles) {
        return Err("Refusing to write invalid FreeSki state.".into());
    }
    let bytes = serde_json::to_vec(state).map_err(|e| e.to_string())?;
    #[cfg(feature = "ui")]
    {
        omarchy_chess::storage::atomic_write(path, &bytes)
    }
    #[cfg(not(feature = "ui"))]
    {
        // Portable equivalent for the headless evidence runner. Both feature builds
        // exercise the same save round-trip, retention and permissions tests.
        let parent = path.parent().ok_or("Missing save directory")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        temp.write_all(&bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        temp.persist(path).map_err(|e| e.to_string())?;
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    }
}
/// Called only by the explicit archive/reset action; never overwrites an archive.
pub fn archive(path: &Path) -> Result<PathBuf, String> {
    for n in 1..=10000 {
        let backup = path.with_extension(format!("archive-{n}.json"));
        match fs::hard_link(path, &backup) {
            Ok(()) => {
                fs::remove_file(path).map_err(|e| e.to_string())?;
                return Ok(backup);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.to_string()),
        }
    }
    Err("No free archive filename; original retained.".into())
}
