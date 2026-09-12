use super::engine::*;
fn tick(s: &mut Sim, n: u32, input: u8) {
    for _ in 0..n {
        s.tick(input);
    }
}
#[test]
fn bags_contain_seven_and_are_reproducible() {
    let mut a = Sim::new(1, Mode::Marathon, 10, 2);
    let mut b = a.clone();
    let mut ks = vec![];
    for _ in 0..140 {
        ks.push(a.active.kind);
        a.board = [[0; 10]; 24];
        b.board = [[0; 10]; 24];
        a.tick(HARD);
        a.tick(0);
        b.tick(HARD);
        b.tick(0);
        assert_eq!(a, b);
    }
    for chunk in ks.chunks(7) {
        let mut c = chunk.to_vec();
        c.sort();
        assert_eq!(c, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
#[test]
fn wall_floor_and_stack_kicks() {
    let mut s = Sim::new(2, Mode::Marathon, 10, 2);
    s.active = Piece {
        kind: 1,
        rotation: 1,
        x: -2,
        y: 10,
    };
    assert!(s.fits(s.active));
    assert!(s.rotate(true));
    assert!(s.fits(s.active));
    s.active = Piece {
        kind: 3,
        rotation: 0,
        x: 3,
        y: 22,
    };
    assert!(s.rotate(true));
    assert!(s.fits(s.active));
    s.board[21][3] = 4;
    s.active = Piece {
        kind: 3,
        rotation: 0,
        x: 3,
        y: 19,
    };
    assert!(s.rotate(false));
    assert!(s.fits(s.active));
}
#[test]
fn blocked_rotation_does_not_mutate() {
    let mut s = Sim::new(3, Mode::Marathon, 10, 2);
    s.active = Piece {
        kind: 1,
        rotation: 0,
        x: 3,
        y: 10,
    };
    s.board = [[7; 10]; 24];
    for (x, y) in s.active.cells() {
        s.board[y as usize][x as usize] = 0;
    }
    let p = s.active;
    assert!(!s.rotate(true));
    assert_eq!(p, s.active);
}
#[test]
fn hold_once_per_piece_and_resets_on_lock() {
    let mut s = Sim::new(4, Mode::Marathon, 10, 2);
    let first = s.active.kind;
    s.tick(HOLD);
    assert_eq!(s.held, Some(first));
    let second = s.active.kind;
    s.tick(0);
    s.tick(HOLD);
    assert_eq!(s.active.kind, second);
    s.tick(HARD);
    s.tick(0);
    s.tick(HOLD);
    assert_eq!(s.active.kind, first);
}
#[test]
fn hard_drop_lands_at_ghost_and_scores_distance() {
    let mut s = Sim::new(1, Mode::Marathon, 10, 2);
    let g = s.ghost();
    let expected = (g.y - s.active.y) as u64 * 2;
    s.tick(HARD);
    assert_eq!(s.score, expected);
    for (x, y) in g.cells() {
        assert_eq!(s.board[y as usize][x as usize], g.kind);
    }
    assert_eq!(s.locks, 1);
}
#[test]
fn lock_reset_cap_prevents_stalling() {
    let mut s = Sim::new(1, Mode::Marathon, 10, 2);
    s.active = s.ghost();
    for i in 0..150 {
        if s.locks > 0 {
            break;
        }
        s.tick(if i % 2 == 0 { LEFT } else { RIGHT });
    }
    assert!(s.locks > 0);
}
#[test]
fn four_lines_clear_together_and_combo_uses_previous_level() {
    let mut s = Sim::new(1, Mode::Marathon, 10, 2);
    s.lines = 9;
    for round in 0..2 {
        for y in 20..24 {
            s.board[y] = [2; 10];
            s.board[y][5] = 0;
        }
        s.active = Piece {
            kind: 1,
            rotation: 1,
            x: 3,
            y: 20,
        };
        s.tick(HARD);
        assert_eq!(s.last_clear, 4);
        s.tick(0);
        assert_eq!(s.lines, 13 + round * 4);
    }
    assert_eq!(s.score, 800 + 1700);
    assert_eq!(s.combo, 2);
}
#[test]
fn sprint_completes_at_forty_and_clock_stops() {
    let mut s = Sim::new(2, Mode::Sprint, 10, 2);
    s.lines = 39;
    s.board[23] = [1; 10];
    for x in 3..7 {
        s.board[23][x] = 0;
    }
    s.active = Piece {
        kind: 1,
        rotation: 0,
        x: 3,
        y: 22,
    };
    s.tick(HARD);
    assert_eq!(s.outcome, Outcome::Complete);
    assert_eq!(s.ticks, 1);
    tick(&mut s, 60, SOFT);
    assert_eq!(s.ticks, 1);
}
#[test]
fn hidden_cells_after_lock_top_out() {
    let mut s = Sim::new(5, Mode::Marathon, 10, 2);
    s.board[4] = [1; 10];
    s.board[4][0] = 0;
    s.tick(HARD);
    assert_eq!(s.outcome, Outcome::TopOut);
}
#[test]
fn exact_save_restoration_including_repeat_and_bag() {
    let mut a = Sim::new(543, Mode::Marathon, 8, 1);
    tick(&mut a, 9, LEFT | SOFT);
    a.tick(HOLD);
    let bytes = serde_json::to_vec(&a).unwrap();
    let mut b: Sim = serde_json::from_slice(&bytes).unwrap();
    assert!(b.valid());
    for input in [0, CW, SOFT, RIGHT, 0, HARD, 0, HOLD, CCW, 0, HARD] {
        a.tick(input);
        b.tick(input);
        assert_eq!(a, b);
    }
}
#[test]
fn opposing_directions_cancel_and_repeat_is_tick_based() {
    let mut s = Sim::new(1, Mode::Marathon, 4, 2);
    let x = s.active.x;
    s.tick(LEFT | RIGHT);
    assert_eq!(s.active.x, x);
    s.tick(LEFT);
    assert_eq!(s.active.x, x - 1);
    tick(&mut s, 3, LEFT);
    assert_eq!(s.active.x, x - 1);
    s.tick(LEFT);
    assert_eq!(s.active.x, x - 2);
}
#[test]
fn replay_validates_terminal_run_and_rejects_tampering() {
    let (seed, r) = terminal_replay();
    let s = replay(seed, &r).unwrap();
    assert_eq!(s.score, r.score);
    let mut bad = r.clone();
    bad.score += 1;
    assert!(replay(seed, &bad).is_err());
    bad = r.clone();
    bad.events.push(InputEvent {
        tick: 0,
        input: 128,
    });
    assert!(replay(seed, &bad).is_err());
    bad = r.clone();
    bad.rules = "stack-v2".into();
    assert!(replay(seed, &bad).is_err());
    bad = r.clone();
    bad.ticks += 1;
    assert!(replay(seed, &bad).is_err());
}
fn terminal_replay() -> (u64, Replay) {
    let seed = 9;
    let mut s = Sim::new(seed, Mode::Marathon, 10, 2);
    let mut events = vec![];
    while s.outcome == Outcome::Playing {
        let input = if s.ticks.is_multiple_of(2) { HARD } else { 0 };
        events.push(InputEvent {
            tick: s.ticks,
            input,
        });
        s.tick(input);
    }
    (
        seed,
        Replay {
            rules: RULES.into(),
            mode: s.mode,
            das: s.das,
            arr: s.arr,
            ticks: s.ticks,
            events,
            pauses: vec![PauseEvent {
                tick: 0,
                duration_ms: 5000,
            }],
            score: s.score,
            lines: s.lines,
        },
    )
}
