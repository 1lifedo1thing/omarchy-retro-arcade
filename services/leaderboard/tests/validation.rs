use arcade_leaderboard::{Identity, Submission, Ticket};
use arcade_leaderboard_service::{hash, now, Service};
use omarchy_stack::engine::*;
use serde_json::{json, Value};
use std::path::Path;
fn identity(service: &mut Service) -> Identity {
    serde_json::from_value(
        service
            .handle("POST", "/identity", None, Value::Null)
            .unwrap(),
    )
    .unwrap()
}
fn ticket(service: &mut Service, id: &Identity, mode: Mode) -> Ticket {
    serde_json::from_value(
        service
            .handle(
                "POST",
                "/tickets",
                Some(&id.credential),
                json!({"mode":format!("{mode:?}"),"rules":RULES}),
            )
            .unwrap(),
    )
    .unwrap()
}
fn record(s: &mut Sim, events: &mut Vec<InputEvent>, input: u8) {
    if events.last().map_or(0, |e| e.input) != input {
        events.push(InputEvent {
            tick: s.ticks,
            input,
        });
    }
    s.tick(input);
}
fn result(s: Sim, events: Vec<InputEvent>) -> Replay {
    Replay {
        rules: RULES.into(),
        mode: s.mode,
        das: s.das,
        arr: s.arr,
        ticks: s.ticks,
        events,
        pauses: vec![PauseEvent {
            tick: 0,
            duration_ms: 3000,
        }],
        score: s.score,
        lines: s.lines,
    }
}
fn marathon(seed: u64) -> Replay {
    let mut s = Sim::new(seed, Mode::Marathon, 10, 2);
    let mut events = vec![];
    while s.outcome == Outcome::Playing {
        let input = if s.ticks.is_multiple_of(2) { HARD } else { 0 };
        record(&mut s, &mut events, input);
    }
    result(s, events)
}
fn submission(t: &Ticket, r: &Replay) -> Value {
    serde_json::to_value(Submission {
        ticket: t.ticket.clone(),
        alias: "Player One".into(),
        replay: serde_json::to_value(r).unwrap(),
    })
    .unwrap()
}
#[test]
fn validates_deduplicates_ties_separates_versions_and_deletes() {
    let mut svc = Service::open(Path::new(":memory:")).unwrap();
    let a = identity(&mut svc);
    let b = identity(&mut svc);
    let t = ticket(&mut svc, &a, Mode::Marathon);
    let r = marathon(t.seed);
    let sub = submission(&t, &r);
    let accepted = svc
        .handle("POST", "/submissions", Some(&a.credential), sub.clone())
        .unwrap();
    assert_eq!(accepted["result"], r.score);
    assert_eq!(
        svc.handle("POST", "/submissions", Some(&a.credential), sub.clone())
            .unwrap(),
        accepted,
        "lost-response retries must be idempotent"
    );
    let mut changed = sub.clone();
    changed["alias"] = json!("Different");
    assert_eq!(
        svc.handle("POST", "/submissions", Some(&a.credential), changed)
            .unwrap_err()
            .0,
        409
    );
    assert!(svc
        .handle("POST", "/submissions", Some(&b.credential), sub)
        .is_err());
    let t2 = ticket(&mut svc, &b, Mode::Marathon);
    svc.db
        .execute(
            "UPDATE tickets SET seed=? WHERE id=?",
            [t.seed.to_string(), hash(&t2.ticket)],
        )
        .unwrap();
    svc.handle(
        "POST",
        "/submissions",
        Some(&b.credential),
        submission(&t2, &r),
    )
    .unwrap();
    svc.db
        .execute(
            "INSERT INTO scores VALUES (?,'Marathon','stack-v0','Old board',9999999,0)",
            [&a.id],
        )
        .unwrap();
    let board = svc
        .handle(
            "GET",
            "/boards/stack-v1/Marathon",
            Some(&a.credential),
            Value::Null,
        )
        .unwrap();
    assert_eq!(board["rows"].as_array().unwrap().len(), 2);
    assert_eq!(board["rows"][0]["rank"], 1);
    assert_eq!(board["rows"][1]["rank"], 1);
    assert_eq!(board["own"]["identity"], a.id);
    assert!(svc
        .handle("GET", "/boards/stack-v2/Marathon", None, Value::Null)
        .is_err());
    svc.handle("DELETE", "/scores", Some(&a.credential), Value::Null)
        .unwrap();
    let board = svc
        .handle("GET", "/boards/stack-v1/Marathon", None, Value::Null)
        .unwrap();
    assert_eq!(board["rows"].as_array().unwrap().len(), 1);
    assert_eq!(board["rows"][0]["identity"], b.id);
    svc.moderate(&b.id, true).unwrap();
    assert!(svc
        .handle(
            "POST",
            "/tickets",
            Some(&b.credential),
            json!({"mode":"Marathon","rules":RULES})
        )
        .is_err());
}
#[test]
fn rejects_tampering_expiry_aliases_and_impossible_events() {
    let mut svc = Service::open(Path::new(":memory:")).unwrap();
    let id = identity(&mut svc);
    let t = ticket(&mut svc, &id, Mode::Marathon);
    let r = marathon(t.seed);
    let original = submission(&t, &r);
    for field in ["score", "ticks", "lines"] {
        let mut bad = original.clone();
        bad["replay"][field] = json!(bad["replay"][field].as_u64().unwrap() + 1);
        assert!(svc
            .handle("POST", "/submissions", Some(&id.credential), bad)
            .is_err());
    }
    let mut bad = original.clone();
    bad["replay"]["events"][0]["input"] = json!(128);
    assert!(svc
        .handle("POST", "/submissions", Some(&id.credential), bad)
        .is_err());
    let mut bad = original.clone();
    bad["replay"]["rules"] = json!("stack-v2");
    assert!(svc
        .handle("POST", "/submissions", Some(&id.credential), bad)
        .is_err());
    let mut bad = original.clone();
    bad["alias"] = json!("admin");
    assert!(svc
        .handle("POST", "/submissions", Some(&id.credential), bad)
        .is_err());
    let mut bad = original.clone();
    bad["replay"]["events"] = json!([{"tick":0,"input":1},{"tick":0,"input":2}]);
    assert!(svc
        .handle("POST", "/submissions", Some(&id.credential), bad)
        .is_err());
    svc.db
        .execute(
            "UPDATE tickets SET expires=? WHERE id=?",
            rusqlite::params![now() as i64 - 1, hash(&t.ticket)],
        )
        .unwrap();
    assert_eq!(
        svc.handle("POST", "/submissions", Some(&id.credential), original)
            .unwrap_err()
            .0,
        410
    );
}
// A simple placement search plays real inputs through the production engine. It is a test
// fixture generator, not a bypass: no board cells, scores or timing are edited.
fn sprint(seed: u64) -> Replay {
    let mut sim = Sim::new(seed, Mode::Sprint, 4, 1);
    let mut events = vec![];
    for _ in 0..600 {
        if sim.outcome != Outcome::Playing {
            break;
        }
        let mut best: Option<(f64, Sim, Vec<u8>)> = None;
        for rot in 0..4 {
            for target in -2..10 {
                let mut s = sim.clone();
                let mut inputs = vec![];
                for _ in 0..rot {
                    for input in [0, CW] {
                        s.tick(input);
                        inputs.push(input);
                    }
                }
                for _ in 0..16 {
                    if s.active.x == target {
                        break;
                    }
                    let input = if s.active.x < target { RIGHT } else { LEFT };
                    let old = s.active.x;
                    for i in [0, input] {
                        s.tick(i);
                        inputs.push(i);
                    }
                    if s.active.x == old {
                        break;
                    }
                }
                for i in [0, HARD, 0] {
                    s.tick(i);
                    inputs.push(i);
                }
                if s.outcome == Outcome::TopOut {
                    continue;
                }
                let mut heights = [0; 10];
                let mut holes = 0;
                for (x, height) in heights.iter_mut().enumerate() {
                    let mut seen = false;
                    for y in 0..24 {
                        if s.board[y][x] != 0 {
                            if !seen {
                                *height = 24 - y;
                            }
                            seen = true;
                        } else if seen {
                            holes += 1;
                        }
                    }
                }
                let sum: usize = heights.iter().sum();
                let bump: usize = heights.windows(2).map(|w| w[0].abs_diff(w[1])).sum();
                let value = -(sum as f64) * 0.51 - (holes as f64) * 0.90 - (bump as f64) * 0.25
                    + ((s.lines - sim.lines) as f64) * 0.76;
                if best.as_ref().is_none_or(|b| value > b.0) {
                    best = Some((value, s, inputs));
                }
            }
        }
        let (_, next, inputs) = best.expect("sprint solver must have a legal placement");
        for input in inputs {
            if sim.outcome != Outcome::Playing {
                break;
            }
            record(&mut sim, &mut events, input);
        }
        assert_eq!(sim, next);
    }
    assert_eq!(
        sim.outcome,
        Outcome::Complete,
        "solver failed at {} lines",
        sim.lines
    );
    result(sim, events)
}
#[test]
fn sprint_replay_is_completed_and_ranked_by_active_ticks() {
    let mut svc = Service::open(Path::new(":memory:")).unwrap();
    let id = identity(&mut svc);
    let t = ticket(&mut svc, &id, Mode::Sprint);
    svc.db
        .execute("UPDATE tickets SET seed='42' WHERE id=?", [hash(&t.ticket)])
        .unwrap();
    let r = sprint(42);
    let accepted = svc
        .handle(
            "POST",
            "/submissions",
            Some(&id.credential),
            submission(&t, &r),
        )
        .unwrap();
    assert_eq!(accepted["result"], r.ticks);
    assert!(r.lines >= 40);
}
