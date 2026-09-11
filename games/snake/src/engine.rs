use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub const RULES: &str = "snake-v1";
pub const WIDTH: u16 = 24;
pub const HEIGHT: u16 = 20;
pub const CELLS: usize = (WIDTH * HEIGHT) as usize;
pub const FOOD_POINTS: u32 = 10;
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Speed {
    Slow,
    #[default]
    Normal,
    Fast,
}
impl Speed {
    pub const ALL: [Self; 3] = [Self::Slow, Self::Normal, Self::Fast];
    pub fn period(self) -> u8 {
        match self {
            Self::Slow => 10,
            Self::Normal => 6,
            Self::Fast => 4,
        }
    }
    pub fn index(self) -> usize {
        match self {
            Self::Slow => 0,
            Self::Normal => 1,
            Self::Fast => 2,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Slow => "Slow",
            Self::Normal => "Normal",
            Self::Fast => "Fast",
        }
    }
    pub fn board(self) -> String {
        format!("{RULES}/{}", self.label())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn delta(self) -> (i16, i16) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
    pub fn opposite(self, other: Self) -> bool {
        let (x, y) = self.delta();
        let (a, b) = other.delta();
        x == -a && y == -b
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Outcome {
    Playing,
    Collision,
    Won,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sim {
    pub rules: String,
    /// Head first; cell = y * WIDTH + x.
    pub body: VecDeque<u16>,
    pub direction: Direction,
    pub buffered: VecDeque<Direction>,
    pub food: Option<u16>,
    pub rng: u64,
    pub score: u32,
    pub speed: Speed,
    pub tick: u64,
    pub phase: u8,
    pub outcome: Outcome,
}
impl Sim {
    pub fn new(seed: u64, speed: Speed) -> Self {
        let mut s = Self {
            rules: RULES.into(),
            body: [253, 252, 251, 250].into(),
            direction: Direction::Right,
            buffered: VecDeque::new(),
            food: None,
            rng: seed,
            score: 0,
            speed,
            tick: 0,
            phase: 0,
            outcome: Outcome::Playing,
        };
        s.spawn_food();
        s
    }
    /// SplitMix64. Rejection sampling removes modulo bias, including for seed zero.
    fn random(&mut self) -> u64 {
        self.rng = self.rng.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub(crate) fn spawn_food(&mut self) {
        let empty: Vec<_> = (0..WIDTH * HEIGHT)
            .filter(|c| !self.body.contains(c))
            .collect();
        if empty.is_empty() {
            self.food = None;
            self.outcome = Outcome::Won;
            return;
        }
        let n = empty.len() as u64;
        let threshold = n.wrapping_neg() % n;
        let r = loop {
            let r = self.random();
            if r >= threshold {
                break r;
            }
        };
        self.food = Some(empty[(r % n) as usize]);
    }
    pub fn turn(&mut self, direction: Direction, repeat: bool) -> bool {
        let previous = self.buffered.back().copied().unwrap_or(self.direction);
        if repeat
            || self.outcome != Outcome::Playing
            || self.buffered.len() == 2
            || previous == direction
            || previous.opposite(direction)
        {
            return false;
        }
        self.buffered.push_back(direction);
        true
    }
    pub fn advance(&mut self) {
        if self.outcome != Outcome::Playing {
            return;
        }
        self.tick += 1;
        self.phase += 1;
        if self.phase < self.speed.period() {
            return;
        }
        self.phase = 0;
        if let Some(direction) = self.buffered.pop_front() {
            self.direction = direction;
        }
        let head = self.body[0];
        let (dx, dy) = self.direction.delta();
        let x = (head % WIDTH) as i16 + dx;
        let y = (head / WIDTH) as i16 + dy;
        if !(0..WIDTH as i16).contains(&x) || !(0..HEIGHT as i16).contains(&y) {
            self.outcome = Outcome::Collision;
            return;
        }
        let next = y as u16 * WIDTH + x as u16;
        let growing = self.food == Some(next);
        let occupied = self
            .body
            .iter()
            .take(self.body.len() - usize::from(!growing))
            .any(|c| *c == next);
        if occupied {
            self.outcome = Outcome::Collision;
            return;
        }
        self.body.push_front(next);
        if growing {
            self.score += FOOD_POINTS;
            self.spawn_food();
        } else {
            self.body.pop_back();
        }
    }
    pub fn valid(&self) -> bool {
        if self.rules != RULES
            || !(4..=CELLS).contains(&self.body.len())
            || self.phase >= self.speed.period()
            || self.phase as u64 != self.tick % self.speed.period() as u64
            || self.buffered.len() > 2
            || self.score != (self.body.len() as u32 - 4) * FOOD_POINTS
        {
            return false;
        }
        let mut seen = [false; CELLS];
        for &cell in &self.body {
            if cell as usize >= CELLS || seen[cell as usize] {
                return false;
            }
            seen[cell as usize] = true;
        }
        for (a, b) in self.body.iter().zip(self.body.iter().skip(1)) {
            if (a % WIDTH).abs_diff(b % WIDTH) + (a / WIDTH).abs_diff(b / WIDTH) != 1 {
                return false;
            }
        }
        let mut previous = self.direction;
        for &d in &self.buffered {
            if d == previous || d.opposite(previous) {
                return false;
            }
            previous = d;
        }
        match self.outcome {
            Outcome::Won => self.body.len() == CELLS && self.food.is_none(),
            _ => {
                self.body.len() < CELLS
                    && self
                        .food
                        .is_some_and(|c| (c as usize) < CELLS && !seen[c as usize])
            }
        }
    }
}
