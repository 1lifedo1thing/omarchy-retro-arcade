//! Stack adapter for the shared leaderboard client. Network work never runs in update().
use crate::engine::*;
use arcade_leaderboard::{background, Board, Client, Identity, Submission, Ticket};
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{io::Write, path::PathBuf, sync::mpsc::Receiver, time::Instant};
#[derive(Clone, Serialize, Deserialize)]
struct Pending {
    submission: Submission,
    expires: u64,
}
#[derive(Default, Serialize, Deserialize)]
struct State {
    #[serde(default)]
    endpoint: String,
    identity: Option<Identity>,
    alias: String,
    consent: bool,
    pending: Vec<Pending>,
}
enum Reply {
    Ticket(Identity, Ticket, Mode, u32, u32),
    Board(Board, Mode),
    Shared(String),
    Deleted,
}
struct Recording {
    ticket: Ticket,
    events: Vec<InputEvent>,
    pauses: Vec<PauseEvent>,
    previous: u8,
    pause: Option<(u64, Instant)>,
}
pub struct Online {
    url: Option<String>,
    state: State,
    file: PathBuf,
    pub enabled: bool,
    worker: Option<Receiver<Result<Reply, String>>>,
    pub message: String,
    current: Option<Recording>,
    shared_current: bool,
    board: Option<Board>,
    show: bool,
    share: bool,
    delete: bool,
    mode: Mode,
}
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
impl Online {
    pub fn new() -> Self {
        let file = std::env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/state")
            })
            .join("omarchy-retro-arcade/leaderboard.json");
        let state = std::fs::read(&file)
            .ok()
            .filter(|b| b.len() < 20_000_000)
            .and_then(|b| serde_json::from_slice::<State>(&b).ok())
            .unwrap_or_default();
        let mut url = std::env::var("ARCADE_LEADERBOARD_URL")
            .ok()
            .filter(|u| !u.is_empty());
        let mut message = String::new();
        if !state.endpoint.is_empty() && url.as_ref().is_some_and(|u| u != &state.endpoint) {
            url = None;
            message="The configured service differs from your saved identity. Restore the original endpoint to use its leaderboard.".into();
        }
        Self {
            url,
            state,
            file,
            enabled: false,
            worker: None,
            message,
            current: None,
            shared_current: false,
            board: None,
            show: false,
            share: false,
            delete: false,
            mode: Mode::Marathon,
        }
    }
    fn save(&mut self) -> bool {
        self.state.pending.retain(|p| p.expires > now());
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let dir = self.file.parent().unwrap();
            std::fs::create_dir_all(dir)?;
            let mut f = tempfile::NamedTempFile::new_in(dir)?;
            f.write_all(&serde_json::to_vec(&self.state)?)?;
            f.as_file().sync_all()?;
            f.persist(&self.file)?;
            std::fs::File::open(dir)?.sync_all()?;
            Ok(())
        })();
        if result.is_err() {
            self.message =
                "Could not save leaderboard identity or retry data. Submission stopped.".into();
            false
        } else {
            true
        }
    }
    pub fn has_modal(&self) -> bool {
        self.share || self.show || self.delete
    }
    pub fn discard(&mut self) {
        self.shared_current = false;
        self.current = None;
        self.share = false;
    }
    pub fn prepare(&mut self, mode: Mode, das: u32, arr: u32) -> bool {
        self.discard();
        if !self.enabled || self.url.is_none() {
            return false;
        }
        if self.worker.is_some() {
            self.message =
                "Wait for the current leaderboard request, or turn off community runs.".into();
            return true;
        }
        let url = self.url.clone().unwrap();
        let identity = self.state.identity.clone();
        self.message = "Preparing a community run… You can turn this off and play offline.".into();
        self.worker = Some(background(move || {
            let client = Client::new(&url)?;
            let id = match identity {
                Some(i) => i,
                None => client.call("POST", "/identity", None, None)?,
            };
            let ticket = client.call(
                "POST",
                "/tickets",
                Some(&id.credential),
                Some(&json!({"mode":format!("{mode:?}"),"rules":RULES})),
            )?;
            Ok(Reply::Ticket(id, ticket, mode, das, arr))
        }));
        true
    }
    pub fn poll(&mut self) -> Option<Sim> {
        let reply = self.worker.as_ref().and_then(|r| match r.try_recv() {
            Ok(v) => Some(v),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                Some(Err("Leaderboard worker stopped".into()))
            }
            Err(_) => None,
        });
        if let Some(reply) = reply {
            self.worker = None;
            match reply {
                Err(e) => {
                    self.message = e;
                }
                Ok(Reply::Ticket(id, t, mode, das, arr)) => {
                    self.state.endpoint = self.url.clone().unwrap_or_default();
                    self.state.identity = Some(id);
                    if !self.save() {
                        return None;
                    }
                    if !self.enabled {
                        return None;
                    }
                    if t.rules != RULES
                        || t.mode != format!("{mode:?}")
                        || t.expires <= now()
                        || t.ticket.len() != 64
                    {
                        self.message = "Invalid run ticket; start an offline run.".into();
                        return None;
                    }
                    let seed = t.seed;
                    self.current = Some(Recording {
                        ticket: t,
                        events: vec![],
                        pauses: vec![],
                        previous: 0,
                        pause: None,
                    });
                    self.message = "Community run ready. Sharing is optional when it ends.".into();
                    return Some(Sim::new(seed, mode, das, arr));
                }
                Ok(Reply::Board(b, mode)) => {
                    if mode == self.mode {
                        self.board = Some(b);
                        self.message.clear();
                    } else {
                        self.fetch();
                    }
                }
                Ok(Reply::Shared(ticket)) => {
                    let matched = self
                        .current
                        .as_ref()
                        .is_some_and(|r| r.ticket.ticket == ticket);
                    self.state.pending.retain(|p| p.submission.ticket != ticket);
                    self.save();
                    self.message = "Score accepted. Refresh the board to see your best.".into();
                    if matched {
                        self.current = None;
                        self.shared_current = true;
                    }
                    self.share = false;
                }
                Ok(Reply::Deleted) => {
                    self.state.pending.clear();
                    self.current = None;
                    self.board = None;
                    self.save();
                    self.message =
                        "Your submissions were removed and outstanding tickets revoked.".into();
                }
            }
        }
        None
    }
    pub fn tick(&mut self, s: &Sim, input: u8) {
        if let Some(r) = &mut self.current {
            if s.ticks >= 216_000 || r.events.len() >= 100_000 || r.ticket.expires <= now() {
                self.current = None;
                self.message =
                    "This run now counts locally only (community run limit reached).".into();
                return;
            }
            if input != r.previous {
                r.events.push(InputEvent {
                    tick: s.ticks,
                    input,
                });
                r.previous = input;
            }
        }
    }
    pub fn pause(&mut self, tick: u64, paused: bool) {
        if let Some(r) = &mut self.current {
            if paused {
                if r.pause.is_none() {
                    r.pause = Some((tick, Instant::now()));
                }
            } else if let Some((tick, start)) = r.pause.take() {
                r.pauses.push(PauseEvent {
                    tick,
                    duration_ms: start.elapsed().as_millis().min(86_400_000) as u64,
                });
                if r.pauses.len() > 1000 {
                    self.current = None;
                    self.message = "This run counts locally only (pause limit reached).".into();
                }
            }
        }
    }
    fn fetch(&mut self) {
        if self.worker.is_some() {
            return;
        }
        let Some(url) = self.url.clone() else {
            return;
        };
        let token = self.state.identity.as_ref().map(|i| i.credential.clone());
        let mode = self.mode;
        self.worker = Some(background(move || {
            Ok(Reply::Board(
                Client::new(&url)?.call(
                    "GET",
                    &format!("/boards/{RULES}/{mode:?}"),
                    token.as_deref(),
                    None,
                )?,
                mode,
            ))
        }));
    }
    fn submit(&mut self) {
        if self.worker.is_some() {
            return;
        }
        self.state.pending.retain(|p| p.expires > now());
        let Some(p) = self.state.pending.first().cloned() else {
            self.message = "No unexpired submissions to retry.".into();
            return;
        };
        let Some(id) = self.state.identity.clone() else {
            return;
        };
        let Some(url) = self.url.clone() else {
            return;
        };
        self.worker = Some(background(move || {
            let _: serde_json::Value = Client::new(&url)?.call(
                "POST",
                "/submissions",
                Some(&id.credential),
                Some(&serde_json::to_value(&p.submission).unwrap()),
            )?;
            Ok(Reply::Shared(p.submission.ticket))
        }));
    }
    pub fn menu(&mut self, ui: &mut egui::Ui) {
        if self.url.is_none() {
            if !self.message.is_empty() {
                ui.label(&self.message);
            }
            ui.label(
                egui::RichText::new("Community leaderboards are awaiting service deployment.")
                    .small()
                    .weak(),
            );
            return;
        }
        ui.checkbox(
            &mut self.enabled,
            "Prepare new runs for optional community sharing",
        );
        ui.horizontal(|ui| {
            if ui.button("Community boards").clicked() {
                self.show = true;
                self.fetch();
            }
            if !self.state.pending.is_empty() && ui.button("Retry shared score").clicked() {
                self.submit();
            }
        });
        if !self.message.is_empty() {
            ui.label(&self.message);
        }
    }
    pub fn result(&mut self, ui: &mut egui::Ui, s: &Sim) {
        if self.current.is_some()
            && s.outcome != Outcome::Playing
            && (s.mode == Mode::Marathon || s.outcome == Outcome::Complete)
        {
            if ui
                .add_enabled(self.worker.is_none(), egui::Button::new("Share score"))
                .clicked()
            {
                self.share = true;
            }
        } else if self.shared_current {
            ui.label("Shared with the community leaderboard.");
        } else {
            ui.label("Local result. Saved or offline runs are not globally ranked.");
        }
        if !self.message.is_empty() {
            ui.label(&self.message);
        }
        if !self.state.pending.is_empty() && ui.button("Retry submission").clicked() {
            self.submit();
        }
    }
    pub fn windows(&mut self, ctx: &egui::Context, s: Option<&Sim>) {
        if self.share {
            egui::Window::new("Share this result?")
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label("Public: your chosen alias, result, mode, submission date and pseudonymous player ID. Aliases are not unique or verified.");
                    ui.label("These are community leaderboards. Replay validation checks the rules, not whether a human played unaided.");
                    ui.label("Keep your local identity credential safe. Losing it means losing control of this identity and its submissions.");
                    ui.label("Alias (3–24 letters, digits, spaces, _ or -)");
                    ui.text_edit_singleline(&mut self.state.alias);
                    ui.checkbox(&mut self.state.consent,"I understand what becomes public");
                    if ui.add_enabled(self.state.consent && self.worker.is_none(),egui::Button::new("Share publicly")).clicked() {
                        if let (Some(r),Some(s))=(&self.current,s) {
                            let replay=Replay {
                                rules:RULES.into(),mode:s.mode,das:s.das,arr:s.arr,ticks:s.ticks,
                                events:r.events.clone(),pauses:r.pauses.clone(),score:s.score,lines:s.lines,
                            };
                            let pending=Pending {
                                submission:Submission {
                                    ticket:r.ticket.ticket.clone(),alias:self.state.alias.clone(),
                                    replay:serde_json::to_value(replay).unwrap(),
                                },
                                expires:r.ticket.expires,
                            };
                            let existing=self.state.pending.iter().position(|p|p.submission.ticket==pending.submission.ticket);
                            if existing.is_none() && self.state.pending.len()>=8 {
                                self.message="Retry or discard pending submissions first.".into();
                                return;
                            }
                            if let Some(index)=existing {self.state.pending.remove(index);}
                            self.state.pending.insert(0,pending);
                            if self.save() {self.submit();self.share=false;}
                        }
                    }
                    if ui.button("Keep local").clicked() {self.share=false;}
                });
        }
        if self.show {
            let mut open = true;
            egui::Window::new("Community leaderboards")
                .open(&mut open)
                .default_width(520.)
                .show(ctx, |ui| {
                    let previous_mode = self.mode;
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.mode, Mode::Marathon, "Marathon");
                        ui.selectable_value(&mut self.mode, Mode::Sprint, "Sprint");
                        if ui.button("Refresh").clicked() {
                            self.fetch();
                        }
                    });
                    if previous_mode != self.mode {
                        self.board = None;
                        self.fetch();
                    }
                    ui.label(format!(
                        "Rules {RULES} · only each player's best · equal results share rank"
                    ));
                    if let Some(b) = &self.board {
                        egui::ScrollArea::vertical()
                            .max_height(350.)
                            .show(ui, |ui| {
                                for r in &b.rows {
                                    let own = self
                                        .state
                                        .identity
                                        .as_ref()
                                        .is_some_and(|id| id.id == r.identity);
                                    let result = if self.mode == Mode::Sprint {
                                        format!("{:.2}s", r.result as f64 / 60.)
                                    } else {
                                        r.result.to_string()
                                    };
                                    let text = format!(
                                        "#{}  {}{}  {}  ·  {} UTC",
                                        r.rank,
                                        r.alias,
                                        if own { " (you)" } else { "" },
                                        result,
                                        chrono::DateTime::from_timestamp(r.submitted as i64, 0)
                                            .map(|d| d.format("%Y-%m-%d").to_string())
                                            .unwrap_or_else(|| "Unknown date".into())
                                    );
                                    ui.label(if own {
                                        egui::RichText::new(text).strong()
                                    } else {
                                        egui::RichText::new(text)
                                    });
                                }
                                if let Some(r) = &b.own {
                                    ui.label(format!("Your best: #{} · {}", r.rank, r.result));
                                }
                            });
                    }
                    ui.label(&self.message);
                    if !self.state.pending.is_empty()
                        && self.worker.is_none()
                        && ui.button("Discard pending uploads").clicked()
                    {
                        self.state.pending.clear();
                        self.save();
                    }
                    if ui
                        .add_enabled(
                            self.state.identity.is_some() && self.worker.is_none(),
                            egui::Button::new("Remove my submissions"),
                        )
                        .clicked()
                    {
                        self.delete = true;
                    }
                });
            self.show = open;
        }
        if self.delete {
            egui::Window::new("Remove all your submissions?").collapsible(false).show(ctx,|ui|{ui.label("This removes your public scores and revokes your current run tickets. Local records stay on this computer.");if ui.button("Remove submissions").clicked(){if let (Some(url),Some(id))=(self.url.clone(),self.state.identity.clone()){self.worker=Some(background(move||{let _:serde_json::Value=Client::new(&url)?.call("DELETE","/scores",Some(&id.credential),None)?;Ok(Reply::Deleted)}));self.delete=false;}}
if ui.button("Cancel").clicked(){self.delete=false;}});
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn offline_and_restored_runs_never_have_a_ticket() {
        let mut online = Online::new();
        online.url = None;
        online.enabled = true;
        assert!(!online.prepare(Mode::Marathon, 10, 2));
        assert!(online.current.is_none());
    }
    #[test]
    fn failed_submission_keeps_retry_and_local_credential() {
        let temp = tempfile::tempdir().unwrap();
        let mut online = Online::new();
        online.file = temp.path().join("identity.json");
        online.url = Some("http://127.0.0.1:1".into());
        online.state.identity = Some(Identity {
            id: "test".into(),
            credential: "a".repeat(64),
        });
        online.state.pending.push(Pending {
            submission: Submission {
                ticket: "b".repeat(64),
                alias: "Player".into(),
                replay: json!({}),
            },
            expires: now() + 300,
        });
        assert!(online.save());
        online.submit();
        for _ in 0..200 {
            online.poll();
            if online.worker.is_none() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(online.worker.is_none());
        assert_eq!(online.state.pending.len(), 1);
        assert!(online.message.contains("Connection failed"));
        let restored: State =
            serde_json::from_slice(&std::fs::read(&online.file).unwrap()).unwrap();
        assert_eq!(restored.pending.len(), 1);
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(&online.file)
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    #[test]
    fn delayed_retry_does_not_replace_the_current_run() {
        let temp = tempfile::tempdir().unwrap();
        let mut o = Online::new();
        o.file = temp.path().join("state.json");
        o.current = Some(Recording {
            ticket: Ticket {
                ticket: "current".into(),
                seed: 1,
                expires: now() + 300,
                rules: RULES.into(),
                mode: "Marathon".into(),
            },
            events: vec![],
            pauses: vec![],
            previous: 0,
            pause: None,
        });
        let (tx, rx) = std::sync::mpsc::channel();
        o.worker = Some(rx);
        tx.send(Ok(Reply::Shared("previous".into()))).unwrap();
        o.poll();
        assert_eq!(o.current.as_ref().unwrap().ticket.ticket, "current");
        assert!(!o.shared_current);
        let (tx, rx) = std::sync::mpsc::channel();
        o.worker = Some(rx);
        tx.send(Ok(Reply::Shared("current".into()))).unwrap();
        o.poll();
        assert!(o.current.is_none());
        assert!(o.shared_current);
    }
    #[test]
    fn expired_retries_are_removed() {
        let temp = tempfile::tempdir().unwrap();
        let mut o = Online::new();
        o.file = temp.path().join("state.json");
        o.state.pending.push(Pending {
            submission: Submission {
                ticket: "x".into(),
                alias: "Player".into(),
                replay: json!({}),
            },
            expires: now() - 1,
        });
        o.save();
        assert!(o.state.pending.is_empty());
    }
}
