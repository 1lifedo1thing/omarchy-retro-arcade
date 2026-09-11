use crate::game::Game;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Saved {
    pub game: Game,
    pub high: u32,
    pub sound: bool,
    pub follow: bool,
    pub version: u32,
}
pub struct Store {
    pub path: PathBuf,
    _lock: File,
}
pub fn read_bounded(path: &Path, max: u64) -> io::Result<Vec<u8>> {
    let mut b = vec![];
    File::open(path)?.take(max + 1).read_to_end(&mut b)?;
    if b.len() as u64 > max {
        return Err(io::Error::other("File too large"));
    }
    Ok(b)
}
impl Store {
    pub fn open() -> io::Result<Self> {
        let root = std::env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))
            .ok_or_else(|| io::Error::other("No state directory available"))?
            .join("omarchy-invaders");
        fs::create_dir_all(&root)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(root.join("session.lock"))?;
        lock.try_lock_exclusive()
            .map_err(|_| io::Error::other("Omarchy Invaders is already running"))?;
        Ok(Self {
            path: root.join("session.json"),
            _lock: lock,
        })
    }
    pub fn load(&self) -> io::Result<Option<Saved>> {
        match read_bounded(&self.path, 256 * 1024) {
            Ok(b) => match serde_json::from_slice::<Saved>(&b) {
                Ok(s) if s.version == 1 && s.game.valid() => Ok(Some(s)),
                _ => Err(io::Error::other(
                    "Saved session could not be read; original file has been left untouched",
                )),
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
    pub fn save(&self, s: &Saved) -> io::Result<()> {
        let parent = self.path.parent().unwrap();
        let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
        tmp.write_all(&serde_json::to_vec(s)?)?;
        tmp.as_file().sync_all()?;
        tmp.persist(&self.path).map_err(|e| e.error)?;
        File::open(parent)?.sync_all()
    }
    pub fn archive(&self) -> io::Result<()> {
        if !self.path.exists() {
            return Ok(());
        }
        let folder = self.path.parent().unwrap().join("archives");
        fs::create_dir_all(&folder)?;
        let mut out = tempfile::Builder::new()
            .prefix("run-")
            .suffix(".json")
            .tempfile_in(folder)?;
        out.write_all(&read_bounded(&self.path, 256 * 1024)?)?;
        out.as_file().sync_all()?;
        out.keep().map_err(|e| e.error)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn atomic_save_and_archive() {
        let d = tempfile::tempdir().unwrap();
        let s = Store {
            path: d.path().join("session.json"),
            _lock: File::create(d.path().join("lock")).unwrap(),
        };
        let saved = Saved {
            version: 1,
            follow: true,
            ..Default::default()
        };
        s.save(&saved).unwrap();
        assert!(s.load().unwrap().unwrap().game.valid());
        s.archive().unwrap();
        assert_eq!(fs::read_dir(d.path().join("archives")).unwrap().count(), 1);
    }
    #[test]
    fn corrupt_preserved() {
        let d = tempfile::tempdir().unwrap();
        let s = Store {
            path: d.path().join("session.json"),
            _lock: File::create(d.path().join("lock")).unwrap(),
        };
        fs::write(&s.path, b"bad").unwrap();
        assert!(s.load().is_err());
        assert_eq!(fs::read(&s.path).unwrap(), b"bad");
    }
}
