use crate::{
    collision::circle_interval,
    world::{self, Kind, Obstacle},
};
use serde::{Deserialize, Serialize};
pub const RULES_VERSION: u32 = 2;
pub const HZ: u32 = 60;
pub const DT: f64 = 1. / HZ as f64;
pub const MAX_SPEED: f64 = 50.;
pub const MAX_HEADING: f64 = std::f64::consts::FRAC_PI_2;
pub const TURN_RATE: f64 = 1.6;
pub const RADIUS: f64 = 0.65;
pub const JUMP_DURATION: f64 = 1.2;
pub const JUMP_HEIGHT: f64 = 2.5;
pub const PROTECTION_TICKS: u32 = 90;
pub const TUMBLE_TICKS: u32 = 42;
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn lerp(self, other: Self, t: f64) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Phase {
    Ready,
    Running,
    Paused,
    Finished,
    Crashed,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    /// Desired heading in radians, not a position.
    pub heading: f64,
    pub brake: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Jump,
    Crash,
    Finish,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Sim {
    pub phase: Phase,
    pub position: Point,
    pub speed: f64,
    pub heading: f64,
    pub ticks: u64,
    pub crashes: u8,
    pub tumble: u32,
    pub protection: u32,
    /// Elapsed flight time. None means grounded.
    pub jump: Option<f64>,
    pub last_ramp: Option<usize>,
    pub distance: f64,
}
impl Default for Sim {
    fn default() -> Self {
        Self {
            phase: Phase::Ready,
            position: Point::default(),
            speed: 0.,
            heading: 0.,
            ticks: 0,
            crashes: 0,
            tumble: 0,
            protection: 0,
            jump: None,
            last_ramp: None,
            distance: 0.,
        }
    }
}
impl Sim {
    pub fn start(&mut self) {
        if matches!(self.phase, Phase::Ready | Phase::Paused) {
            self.phase = Phase::Running;
        }
    }
    pub fn pause(&mut self) {
        if self.phase == Phase::Running {
            self.phase = Phase::Paused;
        }
    }
    pub fn ended(&self) -> bool {
        matches!(self.phase, Phase::Finished | Phase::Crashed)
    }
    pub fn height(&self) -> f64 {
        self.jump.map(height).unwrap_or(0.)
    }
    pub fn valid(&self, obstacles: &[Obstacle]) -> bool {
        [
            self.position.x,
            self.position.y,
            self.speed,
            self.heading,
            self.distance,
        ]
        .iter()
        .all(|n| n.is_finite())
            && self.position.x.abs() <= world::HALF_WIDTH - RADIUS + 1e-8
            && (0. ..=world::FINISH).contains(&self.position.y)
            && (self.position.y..=world::FINISH).contains(&self.distance)
            && (0. ..=MAX_SPEED).contains(&self.speed)
            && self.heading.abs() <= MAX_HEADING
            && self.crashes <= 3
            && self.tumble <= TUMBLE_TICKS
            && self.protection <= PROTECTION_TICKS
            && self
                .jump
                .is_none_or(|t| t.is_finite() && (0. ..JUMP_DURATION).contains(&t))
            && self
                .last_ramp
                .is_none_or(|id| obstacles.iter().any(|o| o.id == id && o.kind == Kind::Ramp))
            && (self.phase != Phase::Crashed || self.crashes == 3)
            && (self.crashes != 3 || self.phase == Phase::Crashed)
            && (self.phase != Phase::Finished || self.position.y == world::FINISH)
            && (self.phase != Phase::Ready || *self == Self::default())
            && (self.tumble == 0 || (self.jump.is_none() && self.speed == 0.))
            && self.ticks < HZ as u64 * 60 * 60 * 24 * 365
    }
    pub fn step(&mut self, input: Input, obstacles: &[Obstacle]) -> Vec<Event> {
        if self.phase != Phase::Running {
            return vec![];
        }
        self.ticks += 1;
        if self.tumble > 0 {
            self.tumble -= 1;
            return vec![];
        }
        let protected = self.protection > 0;
        self.protection = self.protection.saturating_sub(1);
        let desired = if input.heading.is_finite() {
            input.heading.clamp(-MAX_HEADING, MAX_HEADING)
        } else {
            0.
        };
        let turn = TURN_RATE * DT * if self.jump.is_some() { 0.4 } else { 1. };
        self.heading += (desired - self.heading).clamp(-turn, turn);
        let accel = 6.5 * self.heading.cos()
            - 0.05 * self.speed
            - 4.5 * self.heading.sin().abs()
            - if input.brake { 18. } else { 0. };
        self.speed = (self.speed + accel * DT).clamp(0., MAX_SPEED);
        let a = self.position;
        let b = Point {
            x: (a.x + self.heading.sin() * self.speed * DT)
                .clamp(-world::HALF_WIDTH + RADIUS, world::HALF_WIDTH - RADIUS),
            y: a.y + self.heading.cos() * self.speed * DT,
        };
        let finish = if b.y >= world::FINISH && b.y > a.y {
            Some((world::FINISH - a.y) / (b.y - a.y))
        } else {
            None
        };
        // Find ramps first, so flight height can be evaluated at every contact time.
        let ramp = if self.jump.is_none() {
            obstacles
                .iter()
                .filter(|o| o.kind == Kind::Ramp && Some(o.id) != self.last_ramp)
                .filter_map(|o| {
                    circle_interval(a, b, o.at, o.radius() + RADIUS).map(|(t, _)| (t, o.id))
                })
                .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)))
        } else {
            None
        };
        let flight_at = |t: f64| {
            self.jump
                .map(|elapsed| elapsed + t * DT)
                .or_else(|| ramp.filter(|(at, _)| t >= *at).map(|(at, _)| (t - at) * DT))
        };
        let crash = if protected {
            None
        } else {
            obstacles
                .iter()
                .filter(|o| o.kind != Kind::Ramp)
                .filter_map(|o| {
                    let (lo, hi) = circle_interval(a, b, o.at, o.radius() + RADIUS)?;
                    if flight_at(lo).map(height).unwrap_or(0.) <= o.height() {
                        return Some(lo);
                    }
                    // On the descending half of the parabola, solve exact clearance loss.
                    let landing_time =
                        JUMP_DURATION * 0.5 * (1. + (1. - o.height() / JUMP_HEIGHT).max(0.).sqrt());
                    let elapsed = flight_at(lo)?;
                    let contact = lo + (landing_time - elapsed) / DT;
                    (contact >= lo && contact <= hi).then_some(contact)
                })
                .min_by(f64::total_cmp)
        };
        let mut events = vec![];
        let stop = crash
            .filter(|t| finish.is_none_or(|f| *t <= f))
            .or(finish)
            .unwrap_or(1.);
        if let Some((at, id)) = ramp.filter(|(t, _)| *t < stop) {
            self.last_ramp = Some(id);
            self.jump = Some(-at * DT);
            events.push(Event::Jump);
        }
        self.position = a.lerp(b, stop);
        self.position.y = self.position.y.min(world::FINISH);
        self.distance = self.distance.max(self.position.y);
        if crash.is_some_and(|t| t == stop) {
            self.crashes += 1;
            self.speed = 0.;
            self.jump = None;
            self.heading = 0.;
            if self.crashes == 3 {
                self.phase = Phase::Crashed;
            } else {
                self.position = recovery(self.position, obstacles);
                self.distance = self.distance.max(self.position.y);
                self.tumble = TUMBLE_TICKS;
                self.protection = PROTECTION_TICKS;
            }
            events.push(Event::Crash);
        } else if finish.is_some() {
            self.position.y = world::FINISH;
            self.distance = world::FINISH;
            self.phase = Phase::Finished;
            self.jump = None;
            events.push(Event::Finish);
        } else if let Some(t) = self.jump {
            let t = t + DT;
            self.jump = (t < JUMP_DURATION).then_some(t);
        }
        events
    }
}
pub fn height(elapsed: f64) -> f64 {
    let t = (elapsed / JUMP_DURATION).clamp(0., 1.);
    4. * JUMP_HEIGHT * t * (1. - t)
}
pub fn clear(p: Point, obstacles: &[Obstacle]) -> bool {
    p.x.abs() <= world::HALF_WIDTH - RADIUS
        && (0. ..world::FINISH).contains(&p.y)
        && obstacles
            .iter()
            .all(|o| (p.x - o.at.x).hypot(p.y - o.at.y) > o.radius() + RADIUS + 2.)
}
fn recovery(p: Point, obstacles: &[Obstacle]) -> Point {
    // Search sideways and uphill only: recovery never manufactures distance.
    for dy in [0., -3., -6., -9.] {
        for dx in [0., -4., 4., -8., 8., -12., 12., -20., 20., -30., 30.] {
            let q = Point {
                x: (p.x + dx).clamp(-world::HALF_WIDTH + RADIUS, world::HALF_WIDTH - RADIUS),
                y: (p.y + dy).max(0.),
            };
            if clear(q, obstacles) {
                return q;
            }
        }
    }
    // The authored course reserves these two hazard-free boundary corridors.
    for x in [-36., 36.] {
        let q = Point { x, y: p.y };
        if clear(q, obstacles) {
            return q;
        }
    }
    // Only reachable for an invalid, non-production course: stay stopped and protected.
    p
}
