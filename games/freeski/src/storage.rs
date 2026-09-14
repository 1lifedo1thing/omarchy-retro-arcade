//! One private atomic document; records/preferences are independent of the attempt.
use crate::{
    chase::{Chase, ChasePhase},
    course::{self, CourseProgress, COURSE_COUNT},
    endless,
    engine::{Mode, Phase, Sim, MAX_ENDLESS_DISTANCE, RULES_VERSION},
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
    #[serde(default)]
    pub mode: Mode,
    #[serde(default)]
    pub seed: u64,
    #[serde(default = "default_generator_version")]
    pub generator_version: u32,
    pub run: Sim,
    pub best_distance: f64,
    #[serde(default)]
    pub free_best_distance: f64,
    pub completions: u64,
    pub result_recorded: bool,
    pub reduced_effects: bool,
    #[serde(default)]
    pub chase_enabled: bool,
    #[serde(default)]
    pub chase: Chase,
    #[serde(default)]
    pub chase_best_distance: f64,
    #[serde(default)]
    pub course_index: u8,
    #[serde(default)]
    pub slalom: CourseProgress,
    #[serde(default)]
    pub slalom_best: [Option<u64>; 5],
    #[serde(default = "default_unlocked_courses")]
    pub unlocked_courses: u8,
    #[serde(default)]
    pub muted: bool,
}
impl Default for Save {
    fn default() -> Self {
        Self {
            version: 3,
            rules_version: RULES_VERSION,
            course_version: world::COURSE_VERSION,
            mode: Mode::Practice,
            seed: 0,
            generator_version: endless::GENERATOR_VERSION,
            run: Sim::default(),
            best_distance: 0.,
            free_best_distance: 0.,
            completions: 0,
            result_recorded: false,
            reduced_effects: false,
            chase_enabled: false,
            chase: Chase::default(),
            chase_best_distance: 0.,
            course_index: 0,
            slalom: CourseProgress::default(),
            slalom_best: [None; 5],
            unlocked_courses: 1,
            muted: false,
        }
    }
}
impl Save {
    pub fn obstacles(&self) -> Vec<Obstacle> {
        match self.mode {
            Mode::Practice => world::practice(),
            Mode::FreeSki => endless::obstacles(self.seed, self.run.position.y),
            Mode::Slalom => course::course(self.course_index)
                .map(|course| course.obstacles)
                .unwrap_or_default(),
        }
    }
    pub fn best(&self) -> f64 {
        match self.mode {
            Mode::Practice => self.best_distance,
            Mode::FreeSki if self.chase_enabled => self.chase_best_distance.max(self.run.distance),
            Mode::FreeSki => self.free_best_distance.max(self.run.distance),
            Mode::Slalom => {
                self.slalom_best[self.course_index.min(4) as usize].map_or(0., |ticks| ticks as f64)
            }
        }
    }
    pub fn valid(&self, obstacles: &[Obstacle]) -> bool {
        let selected_course = course::course(self.course_index);
        let course_obstacles = selected_course
            .as_ref()
            .map(|course| course.obstacles.as_slice())
            .unwrap_or(&[]);
        let active = match self.mode {
            Mode::Practice => obstacles,
            // Endless terrain is canonical from seed and position, but simulation
            // validity itself only needs numeric bounds. Avoid generation until
            // after those bounds are known to be safe.
            Mode::FreeSki => &[],
            Mode::Slalom => course_obstacles,
        };
        self.version == 3
            && self.rules_version == RULES_VERSION
            && self.course_version == world::COURSE_VERSION
            && self.generator_version == endless::GENERATOR_VERSION
            && self.run.valid_mode(active, self.mode)
            && self.run.last_ramp.is_none_or(|id| match self.mode {
                Mode::Practice => active
                    .iter()
                    .any(|obstacle| obstacle.id == id && obstacle.kind == world::Kind::Ramp),
                Mode::FreeSki => endless::is_ramp(self.seed, id),
                Mode::Slalom => active
                    .iter()
                    .any(|obstacle| obstacle.id == id && obstacle.kind == world::Kind::Ramp),
            })
            && self.best_distance.is_finite()
            && (0. ..=world::FINISH).contains(&self.best_distance)
            && self.free_best_distance.is_finite()
            && (0. ..=MAX_ENDLESS_DISTANCE).contains(&self.free_best_distance)
            && self.chase_best_distance.is_finite()
            && (0. ..=MAX_ENDLESS_DISTANCE).contains(&self.chase_best_distance)
            && self.course_index < COURSE_COUNT
            && (1..=COURSE_COUNT).contains(&self.unlocked_courses)
            && self.course_index < self.unlocked_courses
            && self.slalom_best.iter().flatten().all(|ticks| *ticks > 0)
            && self.slalom_best.iter().flatten().all(|ticks| {
                *ticks
                    <= crate::engine::HZ as u64 * 60 * 60 * 24 * 365
                        + course::MISSED_GATE_PENALTY_TICKS * 64
            })
            && (0..self.unlocked_courses.saturating_sub(1) as usize)
                .all(|index| self.slalom_best[index].is_some())
            && (self.unlocked_courses as usize..self.slalom_best.len())
                .all(|index| self.slalom_best[index].is_none())
            && selected_course
                .as_ref()
                .is_some_and(|course| self.slalom.valid(course))
            && match self.mode {
                Mode::FreeSki if self.chase_enabled => {
                    self.chase.valid()
                        && match self.chase.phase {
                            ChasePhase::Dormant => {
                                self.run.distance < crate::chase::TRIGGER_DISTANCE
                            }
                            ChasePhase::Warning | ChasePhase::Active => {
                                self.run.distance >= crate::chase::TRIGGER_DISTANCE
                            }
                        }
                        && (self.chase.phase != ChasePhase::Active
                            || (self.chase.position.y - self.run.position.y).abs()
                                <= endless::CHUNK_LENGTH * 4.)
                }
                _ => !self.chase_enabled && self.chase == Chase::default(),
            }
            && (self.run.phase != Phase::Caught
                || (self.mode == Mode::FreeSki
                    && self.chase_enabled
                    && self.chase.phase == ChasePhase::Active))
            && (self.mode != Mode::Slalom
                || selected_course.as_ref().is_some_and(|course| {
                    let resolved_by_distance = course
                        .gates
                        .iter()
                        .take_while(|gate| gate.y <= self.run.distance)
                        .count();
                    self.run.position.y <= course.length
                        && self.run.distance <= course.length
                        && self.slalom.next_gate == resolved_by_distance
                        && (self.run.phase != Phase::Finished
                            || (self.run.position.y == course.length
                                && self.slalom.all_resolved(course)))
                }))
            && (!self.result_recorded
                || (self.run.ended()
                    && match self.mode {
                        Mode::Practice => self.best_distance >= self.run.distance,
                        Mode::FreeSki if self.chase_enabled => {
                            self.chase_best_distance >= self.run.distance
                        }
                        Mode::FreeSki => self.free_best_distance >= self.run.distance,
                        Mode::Slalom => {
                            self.run.phase != Phase::Finished
                                || self.slalom_best[self.course_index as usize].is_some_and(
                                    |ticks| ticks <= self.slalom.final_ticks(self.run.ticks),
                                )
                        }
                    }))
            && (self.completions == 0 || self.best_distance == world::FINISH)
    }
    pub fn record_result(&mut self) {
        if self.run.ended() && !self.result_recorded {
            match self.mode {
                Mode::Practice => {
                    self.best_distance = self.best_distance.max(self.run.distance);
                    if self.run.phase == Phase::Finished {
                        self.completions = self.completions.saturating_add(1);
                    }
                }
                Mode::FreeSki => {
                    if self.chase_enabled {
                        self.chase_best_distance = self.chase_best_distance.max(self.run.distance);
                    } else {
                        self.free_best_distance = self.free_best_distance.max(self.run.distance);
                    }
                }
                Mode::Slalom => {
                    if self.run.phase == Phase::Finished {
                        let ticks = self.slalom.final_ticks(self.run.ticks);
                        let slot = &mut self.slalom_best[self.course_index as usize];
                        *slot = Some(slot.map_or(ticks, |old| old.min(ticks)));
                        self.unlocked_courses = self
                            .unlocked_courses
                            .max((self.course_index + 2).min(COURSE_COUNT));
                    }
                }
            }
            self.result_recorded = true;
        }
    }
    pub fn restart(&mut self) {
        self.bank_free_distance();
        self.run = Sim::default();
        self.chase = Chase::default();
        self.slalom = CourseProgress::default();
        self.result_recorded = false;
    }
    pub fn select_mode(&mut self, mode: Mode, seed: u64) {
        self.bank_free_distance();
        self.mode = mode;
        self.seed = seed;
        self.generator_version = endless::GENERATOR_VERSION;
        self.run = Sim::default();
        self.chase_enabled = false;
        self.chase = Chase::default();
        self.slalom = CourseProgress::default();
        self.result_recorded = false;
    }

    pub fn select_chase(&mut self, enabled: bool, seed: u64) {
        self.select_mode(Mode::FreeSki, seed);
        self.chase_enabled = enabled;
    }

    pub fn set_chase_enabled(&mut self, enabled: bool) -> bool {
        if self.mode != Mode::FreeSki || self.run.phase != Phase::Ready {
            return false;
        }
        self.chase_enabled = enabled;
        self.chase = Chase::default();
        true
    }

    pub fn select_course(&mut self, index: u8) -> bool {
        if index >= self.unlocked_courses || index >= COURSE_COUNT {
            return false;
        }
        self.select_mode(Mode::Slalom, 0);
        self.course_index = index;
        true
    }

    pub fn slalom_total_ticks(&self) -> Option<u64> {
        (self.mode == Mode::Slalom && self.run.phase == Phase::Finished)
            .then(|| self.slalom.final_ticks(self.run.ticks))
    }

    pub fn slalom_medal(&self) -> Option<course::Medal> {
        if self.mode != Mode::Slalom || self.run.phase != Phase::Finished {
            return None;
        }
        let course = course::course(self.course_index)?;
        course::medal(&course, self.run.ticks, &self.slalom)
    }

    fn bank_free_distance(&mut self) {
        if self.mode == Mode::FreeSki {
            if self.chase_enabled {
                self.chase_best_distance = self.chase_best_distance.max(self.run.distance);
            } else {
                self.free_best_distance = self.free_best_distance.max(self.run.distance);
            }
        }
    }
}
fn default_generator_version() -> u32 {
    1
}
fn default_unlocked_courses() -> u8 {
    1
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
    let legacy_version = save.version;
    let legacy_schema = legacy_version == 1;
    let legacy_envelope = matches!(legacy_version, 1 | 2);
    let legacy_rules = save.rules_version == 1;
    if legacy_schema
        && (save.mode != Mode::Practice
            || save.seed != 0
            || save.generator_version != 1
            || save.free_best_distance != 0.)
    {
        return Err("Invalid schema-1 FreeSki save. Original retained.".into());
    }
    if legacy_rules {
        // Rules 2 widens the old numeric bounds without changing state shape.
        // Validate the old limits before accepting this explicit migration.
        if save.run.speed > 22. || save.run.heading.abs() > 1.35 {
            return Err("Invalid rules-1 FreeSki save. Original retained.".into());
        }
        save.rules_version = RULES_VERSION;
    }
    if legacy_envelope {
        save.version = 3;
    }
    if !save.valid(obstacles) {
        return Err("Incompatible or invalid FreeSki save. Original retained.".into());
    }
    if legacy_envelope || legacy_rules {
        // Preserve the original inode before a later atomic save replaces it.
        // Reopening without a write reuses the matching backup.
        let mut retained = false;
        for n in 1..=10000 {
            let label = if legacy_rules {
                "rules-1"
            } else if legacy_version == 1 {
                "schema-1"
            } else {
                "schema-2"
            };
            let backup = path.with_extension(format!("{label}-{n}.json"));
            match fs::hard_link(path, &backup) {
                Ok(()) => {
                    retained = true;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    if fs::read(&backup).ok().as_deref() == Some(bytes.as_slice()) {
                        retained = true;
                        break;
                    }
                }
                Err(e) => return Err(format!("Could not retain legacy save: {e}")),
            }
        }
        if !retained {
            return Err("No free migration backup filename; original retained.".into());
        }
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
