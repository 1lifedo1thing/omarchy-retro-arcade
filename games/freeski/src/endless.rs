//! Deterministic, bounded terrain for Free Ski mode.
//!
//! Chunks reserve a connected central route and hazard-free recovery corridors
//! near both slope edges. Calls return whole chunks so a caller can cache by the
//! skier's current chunk without changing collision geometry mid-chunk.
use crate::{
    engine::{Point, Sim, MAX_ENDLESS_DISTANCE, MAX_HEADING},
    world::{Kind, Obstacle, HALF_WIDTH},
};

pub const GENERATOR_VERSION: u32 = 1;
pub const CHUNK_LENGTH: f64 = 128.;

const ID_STRIDE: usize = 64;
const RAMP_SLOT: usize = 0;
const JUMP_ROCK_SLOT: usize = 1;
const FIRST_HAZARD_SLOT: usize = 2;
const CHUNKS_BEHIND: u64 = 1;
const CHUNKS_AHEAD: u64 = 2;
const ROUTE_HALF_WIDTH: f64 = 8.;
const RECOVERY_INSET: f64 = 4.;

/// Return the bounded obstacle window for the whole chunk containing `y`.
///
/// At least one full chunk is retained behind and two are generated ahead. The
/// same chunk always has the same obstacles and IDs for a given seed.
pub fn obstacles(seed: u64, y: f64) -> Vec<Obstacle> {
    let current = chunk_at(y);
    let first = current.saturating_sub(CHUNKS_BEHIND);
    let last = current.saturating_add(CHUNKS_AHEAD);
    let mut result = Vec::with_capacity(((last - first + 1) as usize) * 18);
    for chunk in first..=last {
        generate_chunk(seed, chunk, &mut result);
    }
    result
}

/// Test whether a stable endless obstacle ID names a ramp for this seed.
///
/// This does not require the ramp's chunk to be in the current visible window,
/// which allows suspended mid-jump runs to validate after crossing a boundary.
pub fn is_ramp(seed: u64, id: usize) -> bool {
    id % ID_STRIDE == RAMP_SLOT && ramp_for_chunk(seed, (id / ID_STRIDE) as u64)
}

/// Steering target used by deterministic production-engine evidence runs.
pub fn reference_heading(seed: u64, sim: &Sim) -> f64 {
    let look_ahead = 72.;
    let upcoming_ramp = obstacles(seed, sim.position.y)
        .into_iter()
        .filter(|obstacle| {
            obstacle.kind == Kind::Ramp
                && obstacle.at.y >= sim.position.y + 2.
                && obstacle.at.y <= sim.position.y + 100.
        })
        .min_by(|a, b| a.at.y.total_cmp(&b.at.y));
    let (target_x, target_y) = upcoming_ramp
        .map(|ramp| (ramp.at.x, ramp.at.y))
        .unwrap_or_else(|| {
            let y = sim.position.y + look_ahead;
            (route_x(seed, y), y)
        });
    (target_x - sim.position.x)
        .atan2((target_y - sim.position.y).max(2.))
        .clamp(-MAX_HEADING, MAX_HEADING)
}

fn chunk_at(y: f64) -> u64 {
    if y.is_finite() && y > 0. {
        (y.min(MAX_ENDLESS_DISTANCE) / CHUNK_LENGTH).floor() as u64
    } else {
        0
    }
}

fn generate_chunk(seed: u64, chunk: u64, result: &mut Vec<Obstacle>) {
    let start = chunk as f64 * CHUNK_LENGTH;

    // The opening gives a new run time to accelerate and choose a line.
    let opening = chunk == 0;
    let open_snow = hash(seed, chunk, 0x4f50_454e).is_multiple_of(6);
    let difficulty = (chunk / 3).min(8) as usize;
    let hazard_count = if open_snow { 4 } else { 8 + difficulty };

    // Optional jumps lie on the reserved route. Their paired low rock is far
    // enough downhill to be cleared at normal Free Ski speed.
    if !opening && ramp_for_chunk(seed, chunk) {
        let y = start + 48. + unit(seed, chunk, 0x5241_4d50) * 18.;
        push(result, chunk, RAMP_SLOT, route_x(seed, y), y, Kind::Ramp);
        let rock_y = y + 16.;
        push(
            result,
            chunk,
            JUMP_ROCK_SLOT,
            route_x(seed, rock_y),
            rock_y,
            Kind::Rock,
        );
    }

    for slot in 0..hazard_count {
        let id_slot = FIRST_HAZARD_SLOT + slot;
        let kind = if hash(seed, chunk, 0x4b49_4e44 + slot as u64).is_multiple_of(5) {
            Kind::Rock
        } else {
            Kind::Tree
        };
        // Bounded rejection avoids the connected route, the edge recovery
        // corridors and other hazards. Skipping a slot is the known-safe
        // fallback when no candidate fits.
        for attempt in 0..12_u64 {
            let key = slot as u64 * 16 + attempt;
            let y_margin = if opening { 92. } else { 9. };
            let y_span = (CHUNK_LENGTH - y_margin - 9.).max(0.);
            let y = start + y_margin + unit(seed, chunk, 0x5900_0000 + key) * y_span;
            let x = -30.5 + unit(seed, chunk, 0x5800_0000 + key) * 61.;
            let radius = obstacle_radius(kind);
            if (x - route_x(seed, y)).abs() < ROUTE_HALF_WIDTH + radius + 0.65 {
                continue;
            }
            if x.abs() + radius + 0.65 + 2. > HALF_WIDTH - RECOVERY_INSET {
                continue;
            }
            if result.iter().any(|other| {
                other.at.y >= start
                    && other.at.y < start + CHUNK_LENGTH
                    && (other.at.x - x).hypot(other.at.y - y) < other.radius() + radius + 1.5
            }) {
                continue;
            }
            push(result, chunk, id_slot, x, y, kind);
            break;
        }
    }
}

fn push(result: &mut Vec<Obstacle>, chunk: u64, slot: usize, x: f64, y: f64, kind: Kind) {
    let base = usize::try_from(chunk)
        .ok()
        .and_then(|value| value.checked_mul(ID_STRIDE))
        .expect("endless chunk index fits obstacle IDs");
    result.push(Obstacle {
        id: base + slot,
        at: Point { x, y },
        kind,
    });
}

fn obstacle_radius(kind: Kind) -> f64 {
    match kind {
        Kind::Tree => 1.4,
        Kind::Rock => 1.2,
        Kind::Ramp => 1.8,
        Kind::Pole => 0.3,
    }
}

fn ramp_for_chunk(seed: u64, chunk: u64) -> bool {
    chunk > 0 && hash(seed, chunk, 0x4a55_4d50).is_multiple_of(3)
}

fn route_x(seed: u64, y: f64) -> f64 {
    let y = y.max(0.);
    let chunk = chunk_at(y);
    let local = (y / CHUNK_LENGTH - chunk as f64).clamp(0., 1.);
    let a = route_node(seed, chunk);
    let b = route_node(seed, chunk.saturating_add(1));
    a + (b - a) * smoothstep(local)
}

fn route_node(seed: u64, node: u64) -> f64 {
    -12. + unit(seed, node, 0x524f_5554) * 24.
}

fn smoothstep(t: f64) -> f64 {
    t * t * (3. - 2. * t)
}

fn unit(seed: u64, chunk: u64, stream: u64) -> f64 {
    let bits = hash(seed, chunk, stream) >> 11;
    bits as f64 * (1. / ((1_u64 << 53) as f64))
}

fn hash(seed: u64, chunk: u64, stream: u64) -> u64 {
    let mut value = seed
        ^ chunk.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ stream.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_finite_and_uphill_positions_use_the_opening_window() {
        assert_eq!(obstacles(7, f64::NAN).len(), obstacles(7, -10.).len());
        assert_eq!(obstacles(7, f64::INFINITY).len(), obstacles(7, 0.).len());
    }

    #[test]
    fn ramp_identity_does_not_depend_on_the_active_window() {
        for seed in 0..16 {
            for chunk in 1..30 {
                let id = chunk * ID_STRIDE + RAMP_SLOT;
                assert_eq!(
                    is_ramp(seed, id),
                    obstacles(seed, chunk as f64 * CHUNK_LENGTH)
                        .iter()
                        .any(|obstacle| obstacle.id == id && obstacle.kind == Kind::Ramp)
                );
            }
        }
    }
}
