//! Stack rules v1. All gameplay runs at 60 ticks/second, without wall-clock time.
use serde::{Deserialize, Serialize};
pub const RULES: &str = "stack-v1";
pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 24;
pub const HIDDEN: usize = 4;
pub const LEFT: u8 = 1;
pub const RIGHT: u8 = 2;
pub const SOFT: u8 = 4;
pub const HARD: u8 = 8;
pub const CW: u8 = 16;
pub const CCW: u8 = 32;
pub const HOLD: u8 = 64;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Marathon,
    Sprint,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    #[default]
    Playing,
    TopOut,
    Complete,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub kind: u8,
    pub rotation: u8,
    pub x: i8,
    pub y: i8,
}
impl Piece {
    pub fn new(kind: u8) -> Self {
        Self {
            kind,
            rotation: 0,
            x: 3,
            y: 2,
        }
    }
    pub fn cells(self) -> [(i8, i8); 4] {
        let mut cells = match self.kind {
            1 => [(0, 1), (1, 1), (2, 1), (3, 1)],
            2 => [(1, 0), (2, 0), (1, 1), (2, 1)],
            3 => [(1, 0), (0, 1), (1, 1), (2, 1)],
            4 => [(1, 0), (2, 0), (0, 1), (1, 1)],
            5 => [(0, 0), (1, 0), (1, 1), (2, 1)],
            6 => [(0, 0), (0, 1), (1, 1), (2, 1)],
            _ => [(2, 0), (0, 1), (1, 1), (2, 1)],
        };
        if self.kind != 2 {
            for _ in 0..self.rotation {
                for (x, y) in &mut cells {
                    (*x, *y) = (if self.kind == 1 { 3 } else { 2 } - *y, *x);
                }
            }
        }
        cells.map(|(x, y)| (x + self.x, y + self.y))
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sim {
    pub board: [[u8; WIDTH]; HEIGHT],
    pub active: Piece,
    pub queue: Vec<u8>,
    pub held: Option<u8>,
    pub hold_used: bool,
    pub rng: u64,
    pub mode: Mode,
    pub outcome: Outcome,
    pub score: u64,
    pub lines: u32,
    pub ticks: u64,
    pub combo: u32,
    pub locks: u32,
    pub gravity: u32,
    pub lock_ticks: u32,
    pub resets: u8,
    pub touched: bool,
    pub previous: u8,
    pub direction: i8,
    pub repeat_ticks: u32,
    pub das: u32,
    pub arr: u32,
    pub last_clear: u8,
}
impl Sim {
    pub fn new(seed: u64, mode: Mode, das: u32, arr: u32) -> Self {
        let mut s = Self {
            board: [[0; WIDTH]; HEIGHT],
            active: Piece::new(1),
            queue: vec![],
            held: None,
            hold_used: false,
            rng: seed,
            mode,
            outcome: Outcome::Playing,
            score: 0,
            lines: 0,
            ticks: 0,
            combo: 0,
            locks: 0,
            gravity: 0,
            lock_ticks: 0,
            resets: 0,
            touched: false,
            previous: 0,
            direction: 0,
            repeat_ticks: 0,
            das: das.clamp(4, 24),
            arr: arr.clamp(1, 12),
            last_clear: 0,
        };
        s.refill();
        let k = s.queue.remove(0);
        s.spawn(k);
        s
    }
    fn random(&mut self) -> u64 {
        self.rng = self.rng.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn refill(&mut self) {
        while self.queue.len() < 6 {
            let mut b = [1, 2, 3, 4, 5, 6, 7];
            for i in (1..7).rev() {
                let j = (self.random() % (i + 1) as u64) as usize;
                b.swap(i, j);
            }
            self.queue.extend(b);
        }
    }
    pub fn level(&self) -> u32 {
        self.lines / 10 + 1
    }
    pub fn fits(&self, p: Piece) -> bool {
        p.cells().iter().all(|&(x, y)| {
            x >= 0
                && x < WIDTH as i8
                && y >= 0
                && y < HEIGHT as i8
                && self.board[y as usize][x as usize] == 0
        })
    }
    pub fn grounded(&self) -> bool {
        !self.fits(Piece {
            y: self.active.y + 1,
            ..self.active
        })
    }
    pub fn ghost(&self) -> Piece {
        let mut p = self.active;
        while self.fits(Piece { y: p.y + 1, ..p }) {
            p.y += 1;
        }
        p
    }
    fn spawn(&mut self, k: u8) {
        self.active = Piece::new(k);
        self.gravity = 0;
        self.lock_ticks = 0;
        self.resets = 0;
        self.touched = false;
        self.refill();
        if !self.fits(self.active) {
            self.outcome = Outcome::TopOut;
        }
    }
    fn adjust(&mut self, p: Piece) -> bool {
        if !self.fits(p) {
            return false;
        }
        let ground = self.grounded();
        self.active = p;
        if (ground || self.grounded()) && self.resets < 15 {
            self.lock_ticks = 0;
            self.resets += 1;
        }
        true
    }
    pub fn rotate(&mut self, clockwise: bool) -> bool {
        if self.active.kind == 2 {
            return false;
        }
        let r = (self.active.rotation + if clockwise { 1 } else { 3 }) % 4;
        // Ordered, symmetric kick candidates in screen coordinates. Same list for both directions.
        for (dx, dy) in [
            (0, 0),
            (-1, 0),
            (1, 0),
            (-2, 0),
            (2, 0),
            (0, -1),
            (-1, -1),
            (1, -1),
            (0, -2),
        ] {
            if self.adjust(Piece {
                rotation: r,
                x: self.active.x + dx,
                y: self.active.y + dy,
                ..self.active
            }) {
                return true;
            }
        }
        false
    }
    fn lock(&mut self) {
        for (x, y) in self.active.cells() {
            self.board[y as usize][x as usize] = self.active.kind;
        }
        self.locks += 1;
        let mut next = [[0; WIDTH]; HEIGHT];
        let mut target = HEIGHT;
        let mut cleared = 0;
        for row in self.board.iter().rev() {
            if row.iter().all(|&v| v != 0) {
                cleared += 1;
            } else {
                target -= 1;
                next[target] = *row;
            }
        }
        self.board = next;
        self.last_clear = cleared;
        if cleared > 0 {
            self.score += ([0, 100, 300, 500, 800][cleared as usize] + 50 * self.combo as u64)
                * self.level() as u64;
            self.combo += 1;
        } else {
            self.combo = 0;
        }
        self.lines += cleared as u32;
        self.hold_used = false;
        if self.mode == Mode::Sprint && self.lines >= 40 {
            self.outcome = Outcome::Complete;
            return;
        }
        if self.board[..HIDDEN].iter().flatten().any(|&v| v != 0) {
            self.outcome = Outcome::TopOut;
            return;
        }
        let k = self.queue.remove(0);
        self.spawn(k);
    }
    pub fn tick(&mut self, input: u8) {
        if self.outcome != Outcome::Playing {
            return;
        }
        self.ticks += 1;
        self.last_clear = 0;
        let edge = input & !self.previous;
        self.previous = input;
        if edge & HOLD != 0 && !self.hold_used {
            let k = self.active.kind;
            let next = self.held.replace(k).unwrap_or_else(|| self.queue.remove(0));
            self.spawn(next);
            self.hold_used = true;
            if self.outcome != Outcome::Playing {
                return;
            }
        }
        if edge & CW != 0 {
            self.rotate(true);
        } else if edge & CCW != 0 {
            self.rotate(false);
        }
        let dir = match (input & LEFT != 0, input & RIGHT != 0) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        if dir != self.direction {
            self.direction = dir;
            self.repeat_ticks = 0;
            if dir != 0 {
                self.adjust(Piece {
                    x: self.active.x + dir,
                    ..self.active
                });
            }
        } else if dir != 0 {
            self.repeat_ticks += 1;
            if self.repeat_ticks >= self.das
                && (self.repeat_ticks - self.das).is_multiple_of(self.arr)
            {
                self.adjust(Piece {
                    x: self.active.x + dir,
                    ..self.active
                });
            }
        }
        if edge & HARD != 0 {
            let p = self.ghost();
            self.score += (p.y - self.active.y) as u64 * 2;
            self.active = p;
            self.lock();
            return;
        }
        self.gravity += 1;
        let interval = (60 / (1 + self.level().saturating_sub(1))).max(1);
        if self.gravity
            >= if input & SOFT != 0 {
                interval.min(2)
            } else {
                interval
            }
        {
            self.gravity = 0;
            let p = Piece {
                y: self.active.y + 1,
                ..self.active
            };
            if self.fits(p) {
                self.active = p;
                if input & SOFT != 0 {
                    self.score += 1;
                }
            }
        }
        if self.grounded() {
            self.touched = true;
        }
        if self.touched {
            self.lock_ticks += 1;
        }
        if self.grounded() && self.lock_ticks >= 30 {
            self.lock();
        }
    }
    pub fn valid(&self) -> bool {
        self.board.iter().flatten().all(|&x| x <= 7)
            && self.queue.len() >= 5
            && self.queue.len() <= 12
            && self.queue.iter().all(|&k| (1..=7).contains(&k))
            && self.held.is_none_or(|k| (1..=7).contains(&k))
            && (1..=7).contains(&self.active.kind)
            && self.active.rotation < 4
            && (-4..=10).contains(&self.active.x)
            && (0..24).contains(&self.active.y)
            && (4..=24).contains(&self.das)
            && (1..=12).contains(&self.arr)
            && self.resets <= 15
            && self.lines <= self.locks.saturating_mul(4)
            && self.last_clear <= 4
            && (self.outcome != Outcome::Playing || self.fits(self.active))
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputEvent {
    pub tick: u64,
    pub input: u8,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PauseEvent {
    pub tick: u64,
    pub duration_ms: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Replay {
    pub rules: String,
    pub mode: Mode,
    pub das: u32,
    pub arr: u32,
    pub ticks: u64,
    pub events: Vec<InputEvent>,
    pub pauses: Vec<PauseEvent>,
    pub score: u64,
    pub lines: u32,
}
pub fn replay(seed: u64, r: &Replay) -> Result<Sim, &'static str> {
    if r.rules != RULES {
        return Err("unsupported rules");
    }
    if r.ticks == 0
        || r.ticks > 216_000
        || r.events.len() > 100_000
        || r.pauses.len() > 1000
        || !(4..=24).contains(&r.das)
        || !(1..=12).contains(&r.arr)
    {
        return Err("replay bounds");
    }
    let mut last = None;
    for e in &r.events {
        if e.tick >= r.ticks || e.input > 127 || last.is_some_and(|t| e.tick <= t) {
            return Err("invalid inputs");
        }
        last = Some(e.tick);
    }
    let mut last_pause = 0;
    for p in &r.pauses {
        if p.tick > r.ticks || p.tick < last_pause || p.duration_ms > 86_400_000 {
            return Err("invalid pause");
        }
        last_pause = p.tick;
    }
    let mut s = Sim::new(seed, r.mode, r.das, r.arr);
    let mut idx = 0;
    let mut input = 0;
    for tick in 0..r.ticks {
        if s.outcome != Outcome::Playing {
            return Err("inputs after end");
        }
        if idx < r.events.len() && r.events[idx].tick == tick {
            input = r.events[idx].input;
            idx += 1;
        }
        s.tick(input);
    }
    if s.outcome == Outcome::Playing || (s.mode == Mode::Sprint && s.outcome != Outcome::Complete) {
        return Err("unfinished run");
    }
    if s.score != r.score || s.lines != r.lines {
        return Err("inconsistent result");
    }
    Ok(s)
}
