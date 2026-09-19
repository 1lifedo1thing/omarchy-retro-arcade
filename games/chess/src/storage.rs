// Compatibility exports for existing Chess callers. Game saves stay below.
use crate::game::{fen, Difficulty, Game, Mode, MAX_PGN};
pub use arcade_platform::storage::{atomic_write, read_bounded, stamp};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use shakmaty::{fen::Fen, uci::UciMove, CastlingMode};
use std::{
    env,
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub version: u32,
    pub initial_fen: String,
    pub moves: Vec<String>,
    pub mode: Mode,
    pub human: bool,
    pub difficulty: Difficulty,
    pub result: String,
    pub ending: String,
    pub flipped: bool,
    pub guides: bool,
    #[serde(default)]
    pub headers: std::collections::BTreeMap<String, String>,
}
impl Session {
    pub fn capture(game: &Game, flipped: bool, guides: bool) -> Self {
        Self {
            version: 2,
            initial_fen: fen(&game.positions[0]),
            moves: game
                .moves
                .iter()
                .map(|m| UciMove::from_move(m, CastlingMode::Standard).to_string())
                .collect(),
            mode: game.mode,
            human: game.human_white,
            difficulty: game.difficulty,
            result: game.result.clone(),
            ending: game.ending.clone(),
            flipped,
            guides,
            headers: game.headers.clone(),
        }
    }
    pub fn game(self) -> Result<(Game, bool, bool), String> {
        if self.version != 2 {
            return Err("Unknown saved-session version.".into());
        }
        let initial = self
            .initial_fen
            .parse::<Fen>()
            .map_err(|e| e.to_string())?
            .into_position(CastlingMode::Standard)
            .map_err(|e| e.to_string())?;
        let mut game = Game::new(initial);
        for text in self.moves {
            let m = game.parse_move(&text)?;
            game.play(m)?;
        }
        if !["*", "1-0", "0-1", "1/2-1/2"].contains(&self.result.as_str()) {
            return Err("Invalid saved result.".into());
        }
        if let Some((actual, _)) = game.automatic_result() {
            if self.result != "*" && self.result != actual {
                return Err("Saved result contradicts position.".into());
            }
        }
        game.mode = self.mode;
        game.human_white = self.human;
        game.difficulty = self.difficulty;
        game.result = self.result;
        game.ending = self.ending;
        game.headers = self.headers;
        Ok((game, self.flipped, self.guides))
    }
}
#[derive(Deserialize)]
struct Legacy {
    version: u32,
    pgn: String,
    mode: Mode,
    human: bool,
    difficulty: Difficulty,
    ending: String,
    flipped: bool,
    guides: bool,
}

pub fn state_dir() -> PathBuf {
    env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".local/state")
        })
        .join("omarchy-chess")
}
pub fn save(dir: &Path, game: &Game, flipped: bool, guides: bool) -> Result<(), String> {
    let data = serde_json::to_vec_pretty(&Session::capture(game, flipped, guides))
        .map_err(|e| e.to_string())?;
    atomic_write(&dir.join("session.json"), &data)
}
pub fn load(dir: &Path) -> Result<(Game, bool, bool), String> {
    let path = dir.join("session.json");
    let bytes = read_bounded(&path, MAX_PGN * 2)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if value.get("version").and_then(|v| v.as_u64()) == Some(1) {
        let old: Legacy = serde_json::from_value(value).map_err(|e| e.to_string())?;
        if old.version != 1 {
            return Err("Unknown legacy version".into());
        }
        let mut game = Game::from_pgn(&old.pgn)?;
        game.mode = old.mode;
        game.human_white = old.human;
        game.difficulty = old.difficulty;
        game.ending = old.ending;
        // Preserve the original Python save before the first Rust write.
        atomic_write(
            &dir.join("archive")
                .join(format!("{}-python-session.json", stamp())),
            &bytes,
        )?;
        Ok((game, old.flipped, old.guides))
    } else {
        serde_json::from_value::<Session>(value)
            .map_err(|e| e.to_string())?
            .game()
    }
}
pub fn archive(dir: &Path, game: &Game, recovery: bool) -> Result<(), String> {
    if recovery && dir.join("session.json").exists() {
        atomic_write(
            &dir.join("archive")
                .join(format!("{}-recovery.json", stamp())),
            &read_bounded(&dir.join("session.json"), MAX_PGN * 2)?,
        )?;
    }
    if !game.moves.is_empty() || game.finished() {
        atomic_write(
            &dir.join("archive").join(format!("{}.pgn", stamp())),
            game.pgn().as_bytes(),
        )?;
    }
    Ok(())
}
pub struct SessionLock {
    _file: File,
}
impl SessionLock {
    pub fn acquire(dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        // Do not open concurrently with the previous Qt application: it uses a different lock protocol.
        if dir.join("session.lock").exists() {
            return Err("Close the Python preview first. If it crashed, remove its stale session.lock after confirming it is not running.".into());
        }
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(dir.join("rust-session.lock"))
            .map_err(|e| e.to_string())?;
        file.try_lock_exclusive()
            .map_err(|_| "Another Omarchy Chess process is using this saved game.".to_string())?;
        Ok(Self { _file: file })
    }
}
