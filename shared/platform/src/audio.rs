//! Session-owned short-cue playback. File I/O and child management run on one worker.
use std::{
    io::Write,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, SyncSender},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

type Cue = Box<dyn FnOnce() -> Vec<u8> + Send>;

const TIMEOUT: Duration = Duration::from_secs(2);
const POLL: Duration = Duration::from_millis(10);

/// One worker and at most one cue per game. Dropping the owner cancels and joins it.
pub struct Playback {
    sender: Option<SyncSender<Cue>>,
    busy: Arc<AtomicBool>,
    cancelled: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl Default for Playback {
    fn default() -> Self {
        Self::with_command("paplay".into())
    }
}

impl Playback {
    fn with_command(program: std::ffi::OsString) -> Self {
        let (sender, receiver) = mpsc::sync_channel(1);
        let busy = Arc::new(AtomicBool::new(false));
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_busy = busy.clone();
        let worker_cancelled = cancelled.clone();
        let worker = thread::Builder::new()
            .name("arcade-audio".into())
            .spawn(move || run(receiver, worker_busy, worker_cancelled, program))
            .ok();
        Self {
            sender: Some(sender),
            busy,
            cancelled,
            worker,
        }
    }

    pub fn available() -> bool {
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .any(|p| p.join("paplay").is_file())
    }

    /// Synthesize only when idle; overlapping cues are discarded, never queued.
    pub fn play(&self, wave: impl FnOnce() -> Vec<u8> + Send + 'static) {
        if self.busy.swap(true, Ordering::AcqRel) {
            return;
        }
        if self
            .sender
            .as_ref()
            .is_none_or(|sender| sender.try_send(Box::new(wave)).is_err())
        {
            self.busy.store(false, Ordering::Release);
        }
    }
}

impl Drop for Playback {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Release);
        // Disconnect wakes an idle worker immediately. Unpark wakes active playback.
        self.sender.take();
        if let Some(worker) = self.worker.take() {
            worker.thread().unpark();
            let _ = worker.join();
        }
    }
}

fn run(
    receiver: Receiver<Cue>,
    busy: Arc<AtomicBool>,
    cancelled: Arc<AtomicBool>,
    program: std::ffi::OsString,
) {
    while let Ok(wave) = receiver.recv() {
        if cancelled.load(Ordering::Acquire) {
            break;
        }
        play_file(&wave(), &cancelled, &program);
        busy.store(false, Ordering::Release);
    }
}

fn play_file(wave: &[u8], cancelled: &AtomicBool, program: &std::ffi::OsStr) {
    let Ok(mut file) = tempfile::NamedTempFile::new() else {
        return;
    };
    if file.write_all(wave).is_err() || cancelled.load(Ordering::Acquire) {
        return;
    }
    let Ok(mut child) = Command::new(program)
        .arg(file.path())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    else {
        return;
    };
    let end = Instant::now() + TIMEOUT;
    while !cancelled.load(Ordering::Acquire) && Instant::now() < end {
        match child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) => thread::park_timeout(POLL),
            Err(_) => break,
        }
    }
    // Never kill by name: only the child this worker owns, then reap before file removal.
    let _ = child.kill();
    let _ = child.wait();
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::{fs, os::unix::fs::PermissionsExt, path::Path};

    fn wait_until(mut condition: impl FnMut() -> bool) {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !condition() {
            assert!(
                Instant::now() < deadline,
                "audio worker did not reach expected state"
            );
            thread::sleep(Duration::from_millis(5));
        }
    }

    fn stalled_player(dir: &Path) -> Playback {
        let script = dir.join("player");
        // $0 is the absolute script path; avoid global PATH changes in parallel tests.
        fs::write(
            &script,
            "#!/bin/sh\nprintf '%s\\n%s\\n' \"$$\" \"$1\" > \"$0.started\"\nexec sleep 30\n",
        )
        .unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        Playback::with_command(script.into_os_string())
    }

    fn started(dir: &Path) -> (String, String) {
        let marker = dir.join("player.started");
        wait_until(|| fs::read_to_string(&marker).is_ok_and(|s| s.lines().count() == 2));
        let text = fs::read_to_string(marker).unwrap();
        let mut lines = text.lines();
        (lines.next().unwrap().into(), lines.next().unwrap().into())
    }

    #[test]
    fn dropping_active_owner_reaps_child_removes_file_and_joins_worker() {
        let dir = tempfile::tempdir().unwrap();
        let playback = stalled_player(dir.path());
        playback.play(|| b"test wave".to_vec());
        let (pid, file) = started(dir.path());
        assert!(Path::new(&file).exists());
        playback.play(|| panic!("busy playback must not synthesize or queue another cue"));
        let start = Instant::now();
        drop(playback);
        assert!(
            start.elapsed() < Duration::from_secs(1),
            "drop waited for playback timeout"
        );
        assert!(
            !Path::new(&format!("/proc/{pid}")).exists(),
            "child remains alive or unreaped"
        );
        assert!(!Path::new(&file).exists());
    }

    #[test]
    fn timeout_reaps_stalled_player_and_accepts_next_cue() {
        let dir = tempfile::tempdir().unwrap();
        let playback = stalled_player(dir.path());
        playback.play(|| vec![1]);
        let (pid, file) = started(dir.path());
        wait_until(|| !playback.busy.load(Ordering::Acquire));
        assert!(!Path::new(&format!("/proc/{pid}")).exists());
        assert!(!Path::new(&file).exists());
        fs::remove_file(dir.path().join("player.started")).unwrap();
        playback.play(|| vec![2]);
        let (next_pid, next_file) = started(dir.path());
        drop(playback);
        assert!(!Path::new(&format!("/proc/{next_pid}")).exists());
        assert!(!Path::new(&next_file).exists());
    }

    #[test]
    fn missing_player_recovers_and_idle_drop_wakes_worker() {
        let dir = tempfile::tempdir().unwrap();
        let playback = Playback::with_command(dir.path().join("absent").into_os_string());
        for _ in 0..2 {
            playback.play(|| vec![0]);
            wait_until(|| !playback.busy.load(Ordering::Acquire));
        }
        drop(playback);
    }

    #[test]
    fn close_immediately_after_enqueue_cancels_pending_work() {
        let dir = tempfile::tempdir().unwrap();
        for _ in 0..20 {
            let playback = stalled_player(dir.path());
            playback.play(|| vec![0]);
            drop(playback);
            let marker = dir.path().join("player.started");
            if let Ok(text) = fs::read_to_string(&marker) {
                let mut lines = text.lines();
                let pid = lines.next().unwrap();
                let file = lines.next().unwrap();
                assert!(!Path::new(&format!("/proc/{pid}")).exists());
                assert!(!Path::new(file).exists());
                fs::remove_file(marker).unwrap();
            }
        }
    }
}
