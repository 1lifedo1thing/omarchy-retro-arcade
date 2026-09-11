use crate::{
    engine::*,
    replay::{self, Action, Event, Replay},
    timing::{Clock, Gate},
};
use std::{collections::VecDeque, time::Duration};
fn step(s: &mut Sim) {
    for _ in 0..s.speed.period() {
        s.advance();
    }
}
#[test]
fn initial_rules_and_exact_fixed_speeds() {
    for speed in Speed::ALL {
        let mut s = Sim::new(0, speed);
        assert!(s.valid());
        assert_eq!(s.body, VecDeque::from([253, 252, 251, 250]));
        for _ in 1..speed.period() {
            s.advance();
            assert_eq!(s.body[0], 253);
        }
        s.advance();
        assert_eq!(s.body[0], 254);
        assert_eq!(s.phase, 0);
    }
}
#[test]
fn all_directions_and_every_wall() {
    for (d, body, next) in [
        (Direction::Right, [253, 252, 251, 250], 254),
        (Direction::Left, [253, 254, 255, 256], 252),
        (Direction::Up, [253, 277, 301, 325], 229),
        (Direction::Down, [253, 229, 205, 181], 277),
    ] {
        let mut s = Sim::new(6, Speed::Normal);
        s.direction = d;
        s.body = body.into();
        s.food = Some(0);
        step(&mut s);
        assert_eq!(s.body[0], next);
        while s.outcome == Outcome::Playing {
            step(&mut s);
        }
        assert_eq!(s.outcome, Outcome::Collision);
        let ended = s.clone();
        step(&mut s);
        assert_eq!(s, ended);
    }
}
#[test]
fn body_collision_and_departing_tail() {
    let mut s = Sim::new(4, Speed::Normal);
    s.body = [25, 26, 50, 49].into();
    s.direction = Direction::Left;
    s.food = Some(0);
    assert!(s.turn(Direction::Down, false));
    step(&mut s);
    assert_eq!(s.body[0], 49);
    assert_eq!(s.outcome, Outcome::Playing);
    s.body = [25, 26, 50, 49, 73, 72].into();
    s.direction = Direction::Left;
    s.score = 20;
    s.food = Some(0);
    s.buffered.clear();
    s.turn(Direction::Down, false);
    step(&mut s);
    assert_eq!(s.outcome, Outcome::Collision);
}
#[test]
fn growth_is_one_cell_and_ten_points() {
    let mut s = Sim::new(12, Speed::Normal);
    s.food = Some(254);
    step(&mut s);
    assert_eq!(s.score, 10);
    assert_eq!(s.body.len(), 5);
    assert_eq!(s.body.back(), Some(&250));
    assert_ne!(s.food, Some(254));
    assert!(s.valid());
}
#[test]
fn two_turn_buffer_rejects_duplicates_reversals_repeats_and_overflow() {
    let mut s = Sim::new(6, Speed::Normal);
    assert!(!s.turn(Direction::Left, false));
    assert!(!s.turn(Direction::Right, false));
    assert!(!s.turn(Direction::Up, true));
    assert!(s.turn(Direction::Up, false));
    assert!(!s.turn(Direction::Down, false));
    assert!(!s.turn(Direction::Up, false));
    assert!(s.turn(Direction::Left, false));
    assert!(!s.turn(Direction::Down, false));
    step(&mut s);
    assert_eq!(s.body[0], 229);
    assert_eq!(s.direction, Direction::Up);
    step(&mut s);
    assert_eq!(s.body[0], 228);
    assert_eq!(s.direction, Direction::Left);
}
#[test]
fn food_is_deterministic_and_never_occupied() {
    for seed in 0..1000 {
        let a = Sim::new(seed, Speed::Normal);
        let b = Sim::new(seed, Speed::Normal);
        assert_eq!(a, b);
        assert!(!a.body.contains(&a.food.unwrap()));
    }
    // Fixed RNG/cell-order vector catches accidental replay-rule changes.
    assert_eq!(Sim::new(0, Speed::Normal).rng, 0x9e3779b97f4a7c15);
}
fn nearly_full() -> Sim {
    let cells: Vec<u16> = (0..HEIGHT)
        .flat_map(|y| {
            (0..WIDTH).map(move |x| y * WIDTH + if y % 2 == 0 { x } else { WIDTH - 1 - x })
        })
        .collect();
    let mut s = Sim::new(0, Speed::Fast);
    s.body = cells[..479].iter().rev().copied().collect();
    s.direction = Direction::Left;
    s.score = 4750;
    s.food = Some(cells[479]);
    s
}
#[test]
fn last_empty_cell_and_full_board_win_do_not_draw_rng() {
    let mut s = nearly_full();
    assert!(s.valid());
    s.spawn_food();
    assert_eq!(s.food, Some(456));
    let rng = s.rng;
    step(&mut s);
    assert_eq!(s.outcome, Outcome::Won);
    assert_eq!(s.body.len(), 480);
    assert_eq!(s.score, 4760);
    assert_eq!(s.food, None);
    assert_eq!(s.rng, rng);
    assert!(s.valid());
}
#[test]
fn integer_clock_does_not_depend_on_render_partition() {
    fn run(fps: u64) -> Sim {
        let mut c = Clock::default();
        let mut s = Sim::new(19, Speed::Normal);
        let mut ns = 0;
        for frame in 1..=fps * 8 {
            let end = frame * 1_000_000_000 / fps;
            let ticks = c.ticks(Duration::from_nanos(end - ns));
            ns = end;
            for _ in 0..ticks {
                let direction = match s.tick {
                    24 | 168 | 312 | 456 => Some(Direction::Down),
                    60 | 204 | 348 => Some(Direction::Left),
                    96 | 240 | 384 => Some(Direction::Up),
                    132 | 276 | 420 => Some(Direction::Right),
                    _ => None,
                };
                if let Some(d) = direction {
                    s.turn(d, false);
                }
                s.advance();
            }
        }
        s
    }
    let expected = run(60);
    assert_eq!(expected.tick, 480);
    assert_eq!(expected.outcome, Outcome::Playing);
    for fps in [15, 30, 59, 120, 144, 240] {
        assert_eq!(run(fps), expected);
    }
}
#[test]
fn pause_and_cancellable_countdown_never_advance_clock() {
    let mut gate = Gate::Running;
    let mut s = Sim::new(0, Speed::Normal);
    s.advance();
    let before = s.clone();
    gate.pause();
    assert_eq!(gate, Gate::Paused);
    assert!(!gate.elapse(Duration::from_secs(100)));
    gate.resume();
    assert!(!gate.elapse(Duration::from_millis(2999)));
    gate.pause();
    assert_eq!(gate, Gate::Paused);
    gate.resume();
    assert!(gate.elapse(Duration::from_secs(3)));
    assert_eq!(gate, Gate::Running);
    assert_eq!(s, before);
}
#[test]
fn json_restores_every_simulation_field_and_phase() {
    let mut s = Sim::new(77, Speed::Fast);
    s.advance();
    s.turn(Direction::Up, false);
    s.turn(Direction::Left, false);
    let mut restored: Sim = serde_json::from_slice(&serde_json::to_vec(&s).unwrap()).unwrap();
    assert!(restored.valid());
    assert_eq!(restored, s);
    for _ in 0..40 {
        s.advance();
        restored.advance();
        assert_eq!(s, restored);
    }
}
fn completed_replay() -> (Replay, Sim) {
    let mut s = Sim::new(7, Speed::Normal);
    let mut r = Replay::new(7, s.speed);
    for _ in 0..7 {
        s.advance();
    }
    s.turn(Direction::Up, false);
    r.record(s.tick, Action::Turn(Direction::Up));
    r.record(s.tick, Action::Pause);
    s.buffered.clear();
    r.record(s.tick, Action::Resume);
    r.record(s.tick, Action::Turn(Direction::Down));
    s.turn(Direction::Down, false);
    while s.outcome == Outcome::Playing {
        s.advance();
    }
    r.end_tick = s.tick;
    (r, s)
}
#[test]
fn replay_equivalence_and_ticket_separation() {
    let (r, s) = completed_replay();
    let b = serde_json::to_vec(&r).unwrap();
    let v = replay::validate(&b, 7, Speed::Normal, RULES).unwrap();
    assert_eq!(v.score, s.score);
    assert_eq!(v.outcome, s.outcome);
    assert_eq!(v.ticks, s.tick);
    assert_eq!(v.board, "snake-v1/Normal");
    assert!(replay::validate(&b, 8, Speed::Normal, RULES).is_err());
    assert!(replay::validate(&b, 7, Speed::Fast, RULES).is_err());
    assert!(replay::validate(&b, 7, Speed::Normal, "snake-v2").is_err());
}
#[test]
fn invalid_replays_and_client_scores_are_rejected() {
    let (r, _) = completed_replay();
    let valid =
        |r: &Replay| replay::validate(&serde_json::to_vec(r).unwrap(), 7, Speed::Normal, RULES);
    let mut x = r.clone();
    x.end_tick -= 1;
    assert!(valid(&x).is_err());
    x.end_tick += 2;
    assert!(valid(&x).is_err());
    x = r.clone();
    x.events.push(Event {
        tick: 0,
        action: Action::Pause,
    });
    assert!(valid(&x).is_err());
    x = r.clone();
    x.events[2].tick += 1;
    assert!(valid(&x).is_err());
    x = r.clone();
    x.events[0].action = Action::Turn(Direction::Left);
    assert!(valid(&x).is_err());
    x = r.clone();
    x.end_tick = replay::MAX_TICKS + 1;
    assert!(valid(&x).is_err());
    let mut value = serde_json::to_value(r).unwrap();
    value["score"] = 999999.into();
    assert!(replay::validate(
        &serde_json::to_vec(&value).unwrap(),
        7,
        Speed::Normal,
        RULES
    )
    .is_err());
    assert!(replay::validate(&vec![b' '; replay::MAX_BYTES + 1], 7, Speed::Normal, RULES).is_err());
}
#[test]
fn invalid_saved_boards_are_rejected() {
    let s = Sim::new(0, Speed::Normal);
    let mut invalid = s.clone();
    invalid.body[0] = 480;
    assert!(!invalid.valid());
    let mut x = s.clone();
    x.body[1] = x.body[0];
    assert!(!x.valid());
    x = s.clone();
    x.food = Some(x.body[0]);
    assert!(!x.valid());
    x = s.clone();
    x.phase = 6;
    assert!(!x.valid());
    x = s.clone();
    x.score = 100;
    assert!(!x.valid());
    x = s.clone();
    x.buffered = [Direction::Left].into();
    assert!(!x.valid());
    x = s;
    x.body[1] = 0;
    assert!(!x.valid());
}
#[cfg(feature = "ui")]
#[test]
fn records_are_independent_and_survive_a_corrupt_run() {
    use crate::storage::*;
    let dir = tempfile::tempdir().unwrap();
    let mut r = Records::default();
    for (speed, score) in [(Speed::Slow, 20), (Speed::Normal, 50), (Speed::Fast, 10)] {
        let mut s = Sim::new(1, speed);
        s.score = score;
        r.record(&s);
    }
    assert_eq!(r.best, [20, 50, 10]);
    let mut s = Sim::new(0, Speed::Slow);
    s.advance();
    s.turn(Direction::Up, false);
    s.turn(Direction::Left, false);
    r.preferences.audio = true;
    save(dir.path(), &mut r, Some(&s)).unwrap();
    let (loaded, restored, error) = load(dir.path());
    assert!(error.is_none());
    assert_eq!(restored, Some(s));
    assert_eq!(loaded.best, r.best);
    assert!(loaded.preferences.audio);
    std::fs::write(dir.path().join("session.json"), b"broken").unwrap();
    let (loaded, restored, error) = load(dir.path());
    assert!(restored.is_none());
    assert!(error.unwrap().contains("could not be read or resumed"));
    assert_eq!(loaded.best, r.best);
    assert_eq!(
        std::fs::read_dir(dir.path().join("archive"))
            .unwrap()
            .count(),
        1
    );
}
#[cfg(feature = "ui")]
#[test]
fn corrupt_records_do_not_destroy_a_valid_session() {
    use crate::storage::*;
    let dir = tempfile::tempdir().unwrap();
    let s = Sim::new(0, Speed::Normal);
    save(dir.path(), &mut Records::default(), Some(&s)).unwrap();
    std::fs::write(dir.path().join("records.json"), b"garbage").unwrap();
    let (_, run, error) = load(dir.path());
    assert!(error.is_some());
    assert_eq!(run, Some(s));
}

#[test]
fn full_board_replay_is_calculated_by_the_same_engine() {
    // A Hamiltonian route tests hundreds of growth events and a real replayed win.
    let mut cycle: Vec<u16> = (0..HEIGHT)
        .flat_map(|y| (1..WIDTH).map(move |x| y * WIDTH + if y % 2 == 0 { x } else { WIDTH - x }))
        .collect();
    cycle.extend((0..HEIGHT).rev().map(|y| y * WIDTH));
    let mut s = Sim::new(42, Speed::Fast);
    let mut r = Replay::new(42, Speed::Fast);
    while s.outcome == Outcome::Playing {
        let index = cycle.iter().position(|c| *c == s.body[0]).unwrap();
        let next = cycle[(index + 1) % CELLS];
        let head = s.body[0];
        let d = if next == head + 1 {
            Direction::Right
        } else if head == next + 1 {
            Direction::Left
        } else if next > head {
            Direction::Down
        } else {
            Direction::Up
        };
        if d != s.direction {
            assert!(s.turn(d, false));
            assert!(r.record(s.tick, Action::Turn(d)));
        }
        step(&mut s);
    }
    assert_eq!(s.outcome, Outcome::Won);
    r.end_tick = s.tick;
    let result =
        replay::validate(&serde_json::to_vec(&r).unwrap(), 42, Speed::Fast, RULES).unwrap();
    assert_eq!(result.score, 4760);
    assert_eq!(result.outcome, Outcome::Won);
}
