//! Deterministic hex-board rules. Rendering and time never affect shot outcomes.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
pub const R: f64 = 20.;
pub const WIDTH: f64 = 400.;
pub const STEP: f64 = 34.64101615137754;
pub const DANGER: f64 = 556.;
pub const LAUNCH: Point = Point { x: 200., y: 620. };
pub const ROWS: usize = 15;
pub const COLS: usize = 10;
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn distance(self, b: Self) -> f64 {
        (self.x - b.x).hypot(self.y - b.y)
    }
}
#[derive(Clone, Debug, Deserialize)]
pub struct Level {
    pub name: String,
    pub rows: Vec<String>,
    pub magazine: Vec<u8>,
    pub pressure_every: usize,
}
pub fn levels() -> Vec<Level> {
    serde_json::from_str(include_str!("../levels/levels.json")).expect("validated authored levels")
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Playing,
    Won,
    Lost,
}
#[derive(Clone, Debug)]
pub struct Board {
    pub cells: [Option<u8>; ROWS * COLS],
    pub pressure: usize,
    pub shots: usize,
    pub score: u32,
    pub status: Status,
    magazine: Vec<u8>,
    pub pressure_every: usize,
}
#[derive(Clone, Debug)]
pub struct Shot {
    pub points: Vec<Point>,
    pub slot: Option<usize>,
    pub color: u8,
}
#[derive(Clone, Debug, Default)]
pub struct Outcome {
    pub popped: Vec<(Point, u8)>,
    pub fallen: Vec<(Point, u8)>,
    pub lowered: bool,
}
pub fn valid(i: usize) -> bool {
    i < ROWS * COLS && ((i / COLS).is_multiple_of(2) || i % COLS < COLS - 1)
}
pub fn neighbors(i: usize) -> Vec<usize> {
    let r = (i / COLS) as i32;
    let c = (i % COLS) as i32;
    let diagonal = if r % 2 == 0 { -1 } else { 1 };
    [
        (r, c - 1),
        (r, c + 1),
        (r - 1, c),
        (r - 1, c + diagonal),
        (r + 1, c),
        (r + 1, c + diagonal),
    ]
    .into_iter()
    .filter_map(|(r, c)| {
        if r < 0 || c < 0 || c >= COLS as i32 {
            return None;
        }
        let j = r as usize * COLS + c as usize;
        valid(j).then_some(j)
    })
    .collect()
}
impl Board {
    pub fn new(level: &Level) -> Result<Self, String> {
        if level.pressure_every == 0
            || level.magazine.is_empty()
            || level.magazine.iter().any(|c| *c > 5)
            || level.rows.len() > 12
        {
            return Err("Invalid level metadata".into());
        }
        let mut b = Self {
            cells: [None; ROWS * COLS],
            pressure: 0,
            shots: 0,
            score: 0,
            status: Status::Playing,
            magazine: level.magazine.clone(),
            pressure_every: level.pressure_every,
        };
        for (r, row) in level.rows.iter().enumerate() {
            if row.len() != if r % 2 == 0 { COLS } else { COLS - 1 } {
                return Err(format!("Invalid row width in {}", level.name));
            }
            for (c, byte) in row.bytes().enumerate() {
                b.cells[r * COLS + c] = match byte {
                    b'.' => None,
                    b'0'..=b'5' => Some(byte - b'0'),
                    _ => return Err("Invalid colour".into()),
                };
            }
        }
        if b.count() == 0 {
            b.status = Status::Won;
        } else if b.connected().len() != b.count() {
            return Err(format!("Floating initial cluster in {}", level.name));
        }
        Ok(b)
    }
    pub fn center(&self, i: usize) -> Point {
        Point {
            x: R + (i % COLS) as f64 * 2. * R + if (i / COLS) % 2 == 1 { R } else { 0. },
            y: 40. + (i / COLS + self.pressure) as f64 * STEP,
        }
    }
    pub fn count(&self) -> usize {
        self.cells.iter().flatten().count()
    }
    pub fn colors(&self) -> BTreeSet<u8> {
        self.cells.iter().flatten().copied().collect()
    }
    pub fn color(&self, ahead: usize) -> Option<u8> {
        let colors = self.colors();
        let desired = self.magazine[(self.shots + ahead) % self.magazine.len()];
        if colors.contains(&desired) {
            Some(desired)
        } else {
            colors.first().copied()
        }
    }
    pub fn connected(&self) -> BTreeSet<usize> {
        self.flood(
            (0..COLS).filter(|i| self.cells[*i].is_some()).collect(),
            None,
        )
    }
    fn flood(&self, mut todo: Vec<usize>, color: Option<u8>) -> BTreeSet<usize> {
        let mut seen = BTreeSet::new();
        while let Some(i) = todo.pop() {
            if self.cells[i].is_none()
                || color.is_some_and(|c| self.cells[i] != Some(c))
                || !seen.insert(i)
            {
                continue;
            }
            todo.extend(neighbors(i));
        }
        seen
    }
    /// Analytic swept-circle collision: earliest event, then a deterministic exposed hex.
    /// Angles are degrees from vertical; positive aims right. No fixed-step tunnelling.
    pub fn trace(&self, angle: f64) -> Option<Shot> {
        if self.status != Status::Playing || !angle.is_finite() {
            return None;
        }
        let color = self.color(0)?;
        let radians = angle.clamp(-78., 78.).to_radians();
        let mut dx = radians.sin();
        let dy = -radians.cos();
        let mut p = LAUNCH;
        let mut points = vec![p];
        for _ in 0..32 {
            let ceiling = 40. + self.pressure as f64 * STEP;
            let mut time = (ceiling - p.y) / dy;
            let mut hit = None;
            let mut wall = false;
            let wall_time = if dx > 1e-9 {
                (WIDTH - R - p.x) / dx
            } else if dx < -1e-9 {
                (R - p.x) / dx
            } else {
                f64::INFINITY
            };
            if wall_time < time {
                time = wall_time;
                wall = true;
            }
            for (i, cell) in self.cells.iter().enumerate() {
                if cell.is_none() {
                    continue;
                }
                let c = self.center(i);
                let x = p.x - c.x;
                let y = p.y - c.y;
                let dot = x * dx + y * dy;
                let disc = dot * dot - (x * x + y * y - 4. * R * R);
                if disc < 0. {
                    continue;
                }
                let t = -dot - disc.sqrt();
                if t >= -1e-7 && t < time - 1e-7 {
                    time = t.max(0.);
                    hit = Some(i);
                    wall = false;
                }
            }
            if !time.is_finite() || time < -1e-6 {
                return Some(Shot {
                    points,
                    slot: None,
                    color,
                });
            }
            p = Point {
                x: p.x + dx * time,
                y: p.y + dy * time,
            };
            points.push(p);
            if wall {
                dx = -dx;
                continue;
            }
            let candidates = hit.map(neighbors).unwrap_or_else(|| (0..COLS).collect());
            let slot = candidates
                .into_iter()
                .filter(|i| self.cells[*i].is_none())
                .filter(|i| {
                    let c = self.center(*i);
                    c.distance(p) <= 2. * R + 1e-6
                        && hit.is_none_or(|h| {
                            let origin = self.center(h);
                            (c.x - origin.x) * (p.x - origin.x)
                                + (c.y - origin.y) * (p.y - origin.y)
                                >= -1e-6
                        })
                })
                .min_by(|a, b| {
                    self.center(*a)
                        .distance(p)
                        .total_cmp(&self.center(*b).distance(p))
                        .then(a.cmp(b))
                });
            return Some(Shot {
                points,
                slot,
                color,
            });
        }
        Some(Shot {
            points,
            slot: None,
            color,
        })
    }
    /// Only commit against the unchanged board which produced this shot.
    pub fn fire(&mut self, angle: f64) -> Option<Outcome> {
        let shot = self.trace(angle)?;
        Some(self.attach(&shot))
    }
    pub fn attach(&mut self, shot: &Shot) -> Outcome {
        let mut out = Outcome::default();
        if self.status != Status::Playing {
            return out;
        }
        self.shots += 1;
        let Some(i) = shot.slot.filter(|i| valid(*i) && self.cells[*i].is_none()) else {
            self.status = Status::Lost;
            return out;
        };
        self.cells[i] = Some(shot.color);
        let cluster = self.flood(vec![i], Some(shot.color));
        if cluster.len() >= 3 {
            for j in cluster {
                out.popped
                    .push((self.center(j), self.cells[j].take().unwrap()));
            }
            let supported = self.connected();
            for j in 0..self.cells.len() {
                if self.cells[j].is_some() && !supported.contains(&j) {
                    out.fallen
                        .push((self.center(j), self.cells[j].take().unwrap()));
                }
            }
            self.score += out.popped.len() as u32 * 100 + out.fallen.len() as u32 * 200;
        }
        if self.count() == 0 {
            self.status = Status::Won;
            self.score += 1000;
            return out;
        }
        if self.shots.is_multiple_of(self.pressure_every) {
            self.pressure += 1;
            out.lowered = true;
        }
        if self
            .cells
            .iter()
            .enumerate()
            .any(|(i, c)| c.is_some() && self.center(i).y + R >= DANGER)
        {
            self.status = Status::Lost;
        }
        out
    }
    pub fn pressure_in(&self) -> usize {
        self.pressure_every - self.shots % self.pressure_every
    }
}
