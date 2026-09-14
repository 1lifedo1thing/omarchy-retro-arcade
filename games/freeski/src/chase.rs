//! Deterministic pursuit policy for optional Free Ski chase runs.
//!
//! The actor is planned independently from rendering and wall time. `plan_tick`
//! is deliberately two phase: the session can order a catch against skier
//! contacts, then commit the actor at the same fractional point in the tick.

use crate::{
    collision::circle_interval,
    engine::{Point, DT, MAX_ENDLESS_DISTANCE},
    world::{Kind, Obstacle, HALF_WIDTH},
};
use serde::{Deserialize, Serialize};

pub const TRIGGER_DISTANCE: f64 = 1_000.;
pub const WARNING_TICKS: u32 = 180;
pub const SPAWN_RETRY_TICKS: u32 = 60;
pub const MAX_FAILED_RETRIES: u8 = 8;
pub const RADIUS: f64 = 0.95;
pub const CATCH_RADIUS: f64 = RADIUS + crate::engine::RADIUS;
pub const RECOVERY_SAFE_GAP: f64 = 14.;
pub const MAX_SPEED_PURSUER: f64 = 56.;
pub const ACCELERATION: f64 = 10.;
pub const BRAKING: f64 = 24.;
pub const TURN_RATE: f64 = 1.05;
pub const TURN_DRAG: f64 = 14.;

const OBSTACLE_MARGIN: f64 = 0.35;
const LOOK_AHEAD: f64 = 12.;
const SPAWN_GAPS: [f64; 4] = [48., 54., 60., 42.];
const SPAWN_OFFSETS: [f64; 5] = [0., -14., 14., -25., 25.];

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChasePhase {
    #[default]
    Dormant,
    Warning,
    Active,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Chase {
    pub phase: ChasePhase,
    pub position: Point,
    pub speed: f64,
    pub heading: f64,
    pub warning_ticks: u32,
    pub retry_ticks: u32,
    pub failed_retries: u8,
}

impl Default for Chase {
    fn default() -> Self {
        Self {
            phase: ChasePhase::Dormant,
            position: Point::default(),
            speed: 0.,
            heading: 0.,
            warning_ticks: 0,
            retry_ticks: 0,
            failed_retries: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChaseEvent {
    Warning,
    Spawned,
}

#[derive(Clone, Copy)]
pub struct TickInput<'a> {
    pub skier_start: Point,
    /// The skier's physical segment endpoint, before any recovery relocation.
    pub skier_end: Point,
    pub skier_distance: f64,
    pub skier_speed: f64,
    pub protected: bool,
    pub obstacles: &'a [Obstacle],
}

#[derive(Clone, Debug)]
pub struct TickPlan {
    next: Chase,
    pub events: Vec<ChaseEvent>,
    pub creature_start: Option<Point>,
    pub creature_end: Option<Point>,
    pub catch_fraction: Option<f64>,
}

impl TickPlan {
    /// Commit this plan at the session's earliest physical contact fraction.
    /// Timers and phase transitions belong to the tick; only actor movement is
    /// clipped when another contact stops the tick early.
    pub fn commit(mut self, fraction: f64) -> Chase {
        if let (Some(start), Some(end)) = (self.creature_start, self.creature_end) {
            self.next.position = start.lerp(end, fraction.clamp(0., 1.));
        }
        self.next
    }

    pub fn finish(self) -> Chase {
        self.next
    }
}

impl Chase {
    pub fn valid(&self) -> bool {
        let finite = [self.position.x, self.position.y, self.speed, self.heading]
            .iter()
            .all(|value| value.is_finite());
        if !finite
            || !(0. ..=MAX_SPEED_PURSUER).contains(&self.speed)
            || self.heading.abs() > std::f64::consts::PI
            || self.warning_ticks > WARNING_TICKS
            || self.retry_ticks > SPAWN_RETRY_TICKS
            || self.failed_retries > MAX_FAILED_RETRIES
        {
            return false;
        }
        match self.phase {
            ChasePhase::Dormant => {
                self.position == Point::default()
                    && self.speed == 0.
                    && self.heading == 0.
                    && self.warning_ticks == 0
                    && self.retry_ticks == 0
                    && self.failed_retries == 0
            }
            ChasePhase::Warning => {
                self.position == Point::default() && self.speed == 0. && self.heading == 0.
            }
            ChasePhase::Active => {
                self.position.x.abs() <= HALF_WIDTH - RADIUS + 1e-8
                    && (0. ..=MAX_ENDLESS_DISTANCE).contains(&self.position.y)
                    && self.warning_ticks == 0
                    && self.retry_ticks == 0
            }
        }
    }

    pub fn plan_tick(&self, input: TickInput<'_>) -> TickPlan {
        let mut next = self.clone();
        let mut events = Vec::new();
        let mut creature_start = None;
        let mut creature_end = None;
        let mut catch_fraction = None;

        match self.phase {
            ChasePhase::Dormant => {
                if input.skier_distance >= TRIGGER_DISTANCE {
                    next.phase = ChasePhase::Warning;
                    next.warning_ticks = WARNING_TICKS;
                    events.push(ChaseEvent::Warning);
                }
            }
            ChasePhase::Warning => {
                if next.warning_ticks > 0 {
                    next.warning_ticks -= 1;
                } else if next.retry_ticks > 0 {
                    next.retry_ticks -= 1;
                } else if let Some(position) = spawn_position(input.skier_end, input.obstacles) {
                    next.phase = ChasePhase::Active;
                    next.position = position;
                    next.speed = 26.;
                    next.heading = heading_to(position, input.skier_end);
                    next.failed_retries = 0;
                    events.push(ChaseEvent::Spawned);
                } else {
                    next.failed_retries = next
                        .failed_retries
                        .saturating_add(1)
                        .min(MAX_FAILED_RETRIES);
                    next.retry_ticks = SPAWN_RETRY_TICKS;
                }
            }
            ChasePhase::Active => {
                let start = self.position;
                let (end, speed, heading) = move_actor(self, input);
                next.position = end;
                next.speed = speed;
                next.heading = heading;
                creature_start = Some(start);
                creature_end = Some(end);
                if !input.protected {
                    catch_fraction = relative_catch(start, end, input.skier_start, input.skier_end);
                }
            }
        }

        TickPlan {
            next,
            events,
            creature_start,
            creature_end,
            catch_fraction,
        }
    }
}

fn spawn_position(skier: Point, obstacles: &[Obstacle]) -> Option<Point> {
    for gap in SPAWN_GAPS {
        let y = (skier.y - gap).max(0.);
        for offset in SPAWN_OFFSETS {
            let candidate = Point {
                x: (skier.x + offset).clamp(-HALF_WIDTH + RADIUS, HALF_WIDTH - RADIUS),
                y,
            };
            if candidate.y < skier.y - CATCH_RADIUS
                && obstacles.iter().all(|obstacle| {
                    (candidate.x - obstacle.at.x).hypot(candidate.y - obstacle.at.y)
                        > obstacle.radius() + RADIUS + OBSTACLE_MARGIN
                })
            {
                return Some(candidate);
            }
        }
    }
    None
}

fn move_actor(chase: &Chase, input: TickInput<'_>) -> (Point, f64, f64) {
    let target = input.skier_end;
    let separation = distance(chase.position, target);
    let protected_close = input.protected && separation <= RECOVERY_SAFE_GAP + 8.;
    let desired = collision_aware_heading(chase.position, target, input.obstacles);
    let turn_error = wrap_angle(desired - chase.heading).abs();
    let desired_speed = if protected_close {
        0.
    } else {
        (input.skier_speed + 8.).clamp(34., MAX_SPEED_PURSUER)
    };
    let speed = (approach(
        chase.speed,
        desired_speed,
        if desired_speed < chase.speed {
            BRAKING
        } else {
            ACCELERATION
        } * DT,
    ) - TURN_DRAG * turn_error.sin().abs() * DT)
        .clamp(0., MAX_SPEED_PURSUER);
    let heading = turn_toward(chase.heading, desired, TURN_RATE * DT);
    let mut end = Point {
        x: (chase.position.x + heading.sin() * speed * DT)
            .clamp(-HALF_WIDTH + RADIUS, HALF_WIDTH - RADIUS),
        y: (chase.position.y + heading.cos() * speed * DT).clamp(0., MAX_ENDLESS_DISTANCE),
    };

    // The steering policy normally avoids geometry. This exact sweep is the
    // safety boundary: contacts stop the actor instead of allowing phasing.
    if let Some(contact) = first_obstacle_contact(chase.position, end, input.obstacles) {
        end = chase.position.lerp(end, (contact - 1e-6).max(0.));
        return (end, speed * 0.2, heading);
    }
    if input.protected {
        if let Some((entry, _)) = circle_interval(chase.position, end, target, RECOVERY_SAFE_GAP) {
            end = chase.position.lerp(end, (entry - 1e-6).max(0.));
            return (end, 0., heading);
        }
    }
    (end, speed, heading)
}

fn collision_aware_heading(from: Point, target: Point, obstacles: &[Obstacle]) -> f64 {
    let desired = heading_to(from, target);
    for offset in [0., -0.55, 0.55, -1.05, 1.05] {
        let heading = wrap_angle(desired + offset);
        let probe = Point {
            x: (from.x + heading.sin() * LOOK_AHEAD)
                .clamp(-HALF_WIDTH + RADIUS, HALF_WIDTH - RADIUS),
            y: (from.y + heading.cos() * LOOK_AHEAD).clamp(0., MAX_ENDLESS_DISTANCE),
        };
        if first_obstacle_contact(from, probe, obstacles).is_none() {
            return heading;
        }
    }
    // No local route is clear. Turning toward the least severe candidate and
    // stopping on exact contact is deterministic and cannot phase geometry.
    desired
}

fn first_obstacle_contact(a: Point, b: Point, obstacles: &[Obstacle]) -> Option<f64> {
    obstacles
        .iter()
        .filter(|obstacle| obstacle.kind != Kind::Ramp)
        .filter_map(|obstacle| {
            circle_interval(a, b, obstacle.at, obstacle.radius() + RADIUS).map(|(entry, _)| entry)
        })
        .min_by(f64::total_cmp)
}

fn relative_catch(
    creature_start: Point,
    creature_end: Point,
    skier_start: Point,
    skier_end: Point,
) -> Option<f64> {
    let relative_start = Point {
        x: creature_start.x - skier_start.x,
        y: creature_start.y - skier_start.y,
    };
    let relative_end = Point {
        x: creature_end.x - skier_end.x,
        y: creature_end.y - skier_end.y,
    };
    circle_interval(relative_start, relative_end, Point::default(), CATCH_RADIUS)
        .map(|(entry, _)| entry)
}

fn heading_to(from: Point, to: Point) -> f64 {
    (to.x - from.x).atan2(to.y - from.y)
}

fn distance(a: Point, b: Point) -> f64 {
    (a.x - b.x).hypot(a.y - b.y)
}

fn approach(current: f64, target: f64, amount: f64) -> f64 {
    current + (target - current).clamp(-amount, amount)
}

fn turn_toward(current: f64, target: f64, amount: f64) -> f64 {
    wrap_angle(current + wrap_angle(target - current).clamp(-amount, amount))
}

fn wrap_angle(angle: f64) -> f64 {
    let two_pi = std::f64::consts::TAU;
    (angle + std::f64::consts::PI).rem_euclid(two_pi) - std::f64::consts::PI
}
