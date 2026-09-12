//! Shared-service adapter. No identity, HTTP or ticket implementation belongs here.
use crate::engine::*;
use serde::{Deserialize, Serialize};
pub const MAX_TICKS: u64 = 60 * 60 * 2 * 60;
pub const MAX_EVENTS: usize = 100_000;
pub const MAX_BYTES: usize = 4 * 1024 * 1024;
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Turn(Direction),
    Pause,
    Resume,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub tick: u64,
    pub action: Action,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Replay {
    pub rules: String,
    pub speed: Speed,
    pub seed: u64,
    pub end_tick: u64,
    pub events: Vec<Event>,
}
impl Replay {
    pub fn new(seed: u64, speed: Speed) -> Self {
        Self {
            rules: RULES.into(),
            speed,
            seed,
            end_tick: 0,
            events: vec![],
        }
    }
    pub fn record(&mut self, tick: u64, action: Action) -> bool {
        if tick > MAX_TICKS || self.events.len() >= MAX_EVENTS {
            return false;
        }
        self.events.push(Event { tick, action });
        true
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct Validated {
    pub score: u32,
    pub outcome: Outcome,
    pub ticks: u64,
    pub board: String,
}
/// Ticket seed, speed and rules MUST be supplied by the shared service, not the client.
/// Caller enforces ticket expiry, wall-time plausibility, deduplication and identity.
pub fn validate(
    bytes: &[u8],
    ticket_seed: u64,
    ticket_speed: Speed,
    ticket_rules: &str,
) -> Result<Validated, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("replay too large");
    }
    let r: Replay = serde_json::from_slice(bytes).map_err(|_| "invalid replay")?;
    if r.rules != RULES
        || r.rules != ticket_rules
        || r.speed != ticket_speed
        || r.seed != ticket_seed
    {
        return Err("ticket mismatch");
    }
    if r.end_tick == 0 || r.end_tick > MAX_TICKS || r.events.len() > MAX_EVENTS {
        return Err("replay limit");
    }
    let mut sim = Sim::new(r.seed, r.speed);
    let mut paused = false;
    for event in r.events {
        if event.tick < sim.tick || event.tick >= r.end_tick {
            return Err("invalid event tick");
        }
        if paused && event.tick != sim.tick {
            return Err("time advanced while paused");
        }
        while sim.tick < event.tick && sim.outcome == Outcome::Playing {
            sim.advance();
        }
        if sim.outcome != Outcome::Playing {
            return Err("input after result");
        }
        match event.action {
            Action::Turn(d) if !paused && sim.turn(d, false) => (),
            Action::Pause if !paused => {
                sim.buffered.clear();
                paused = true;
            }
            Action::Resume if paused => {
                sim.buffered.clear();
                paused = false;
            }
            _ => return Err("invalid input or pause boundary"),
        }
    }
    if paused {
        return Err("incomplete paused replay");
    }
    while sim.tick < r.end_tick && sim.outcome == Outcome::Playing {
        sim.advance();
    }
    if sim.tick != r.end_tick || sim.outcome == Outcome::Playing {
        return Err("incomplete or trailing replay");
    }
    Ok(Validated {
        score: sim.score,
        outcome: sim.outcome,
        ticks: sim.tick,
        board: r.speed.board(),
    })
}
