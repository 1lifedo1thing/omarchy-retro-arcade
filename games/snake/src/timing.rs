//! A rational 60 Hz clock. Wall time only controls dispatch; it never enters Sim.
use std::time::Duration;
#[derive(Default)]
pub struct Clock {
    units: u128,
}
impl Clock {
    pub fn ticks(&mut self, elapsed: Duration) -> u64 {
        self.units += elapsed.as_nanos() * 60;
        let ticks = self.units / 1_000_000_000;
        self.units %= 1_000_000_000;
        ticks as u64
    }
    pub fn reset(&mut self) {
        self.units = 0;
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gate {
    Ready,
    Running,
    Paused,
    Countdown(Duration),
    Result,
}
impl Gate {
    pub fn pause(&mut self) {
        if matches!(self, Self::Running | Self::Countdown(_)) {
            *self = Self::Paused;
        }
    }
    pub fn resume(&mut self) {
        if *self == Self::Paused {
            *self = Self::Countdown(Duration::from_secs(3));
        }
    }
    /// Never dispatch leftover countdown time into the simulation.
    pub fn elapse(&mut self, elapsed: Duration) -> bool {
        if let Self::Countdown(left) = self {
            *left = left.saturating_sub(elapsed);
            if left.is_zero() {
                *self = Self::Running;
                return true;
            }
        }
        false
    }
}
