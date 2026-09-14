use std::collections::{BTreeMap, BTreeSet};

use omarchy_freeski::{
    endless::{self, CHUNK_LENGTH},
    engine::{Event, Input, Mode, Point, Sim},
    world::{Kind, Obstacle, HALF_WIDTH},
};

fn signature(obstacle: &Obstacle) -> (usize, u64, u64, u8) {
    (
        obstacle.id,
        obstacle.at.x.to_bits(),
        obstacle.at.y.to_bits(),
        match obstacle.kind {
            Kind::Tree => 0,
            Kind::Rock => 1,
            Kind::Ramp => 2,
        },
    )
}

#[test]
fn windows_are_deterministic_bounded_and_cover_recovery_overlap() {
    for seed in 0..128 {
        for chunk in [0_u64, 1, 2, 9, 40, 400, 4_000] {
            let middle = chunk as f64 * CHUNK_LENGTH + CHUNK_LENGTH * 0.5;
            let a = endless::obstacles(seed, middle);
            let b = endless::obstacles(seed, middle);
            assert_eq!(
                a.iter().map(signature).collect::<Vec<_>>(),
                b.iter().map(signature).collect::<Vec<_>>()
            );
            assert!(a.len() <= 72, "seed {seed}, chunk {chunk}: {}", a.len());
            assert_eq!(
                a.len(),
                a.iter()
                    .map(|obstacle| obstacle.id)
                    .collect::<BTreeSet<_>>()
                    .len()
            );

            let first = chunk.saturating_sub(1);
            let last = chunk + 2;
            assert!(a.iter().all(|obstacle| {
                let obstacle_chunk = obstacle.id / 64;
                obstacle_chunk >= first as usize && obstacle_chunk <= last as usize
            }));
            if chunk > 0 {
                assert!(a.iter().any(|obstacle| obstacle.id / 64 == first as usize));
            }
            assert!(a.iter().any(|obstacle| obstacle.id / 64 == last as usize));
        }
    }
    let far = endless::obstacles(7, f64::MAX);
    assert!(far.len() <= 72);
    assert_eq!(
        far.len(),
        far.iter()
            .map(|obstacle| obstacle.id)
            .collect::<BTreeSet<_>>()
            .len()
    );
}

#[test]
fn shared_chunks_are_identical_across_window_boundaries() {
    for seed in 0..128 {
        for chunk in 1..24_u64 {
            let before = endless::obstacles(seed, chunk as f64 * CHUNK_LENGTH - 0.001);
            let after = endless::obstacles(seed, chunk as f64 * CHUNK_LENGTH + 0.001);
            let before = before
                .iter()
                .map(|obstacle| (obstacle.id, signature(obstacle)))
                .collect::<BTreeMap<_, _>>();
            let after = after
                .iter()
                .map(|obstacle| (obstacle.id, signature(obstacle)))
                .collect::<BTreeMap<_, _>>();
            let shared = before
                .keys()
                .filter(|id| after.contains_key(id))
                .collect::<Vec<_>>();
            assert!(!shared.is_empty());
            for id in shared {
                assert_eq!(before[id], after[id]);
            }
        }
    }
}

#[test]
fn edge_recovery_corridors_stay_clear_for_a_long_seed_corpus() {
    const SKIER_RADIUS: f64 = 0.65;
    const RECOVERY_MARGIN: f64 = 2.;
    for seed in 0..128 {
        for chunk in 0..80_u64 {
            let obstacles = endless::obstacles(seed, chunk as f64 * CHUNK_LENGTH + 64.);
            for obstacle in obstacles {
                for x in [-36., 36.] {
                    assert!(
                        (obstacle.at.x - x).abs()
                            > obstacle.radius() + SKIER_RADIUS + RECOVERY_MARGIN,
                        "seed {seed}, obstacle {} blocks recovery at {x}",
                        obstacle.id
                    );
                }
                assert!(obstacle.at.x.abs() + obstacle.radius() < HALF_WIDTH);
            }
        }
    }
}

#[test]
fn density_caps_and_every_seed_offers_useful_jumps() {
    for seed in 0..128 {
        let mut ramps = 0;
        for chunk in 1..64_u64 {
            let at = chunk as f64 * CHUNK_LENGTH + 64.;
            let window = endless::obstacles(seed, at);
            let in_chunk = window
                .iter()
                .filter(|obstacle| obstacle.id / 64 == chunk as usize)
                .collect::<Vec<_>>();
            assert!(in_chunk.len() <= 18);
            for ramp in in_chunk
                .iter()
                .filter(|obstacle| obstacle.kind == Kind::Ramp)
            {
                ramps += 1;
                assert!(endless::is_ramp(seed, ramp.id));
                assert!(in_chunk.iter().any(|obstacle| {
                    obstacle.id == ramp.id + 1
                        && obstacle.kind == Kind::Rock
                        && obstacle.at.y > ramp.at.y
                        && obstacle.at.y - ramp.at.y <= 16.001
                }));
            }
        }
        assert!(ramps >= 10, "seed {seed} generated only {ramps} ramps");
    }
}

#[test]
fn reference_heading_is_finite_and_within_production_limits() {
    for seed in 0..128 {
        let mut sim = Sim::default();
        for chunk in 0..100_u64 {
            sim.position = Point {
                x: ((chunk as i64 % 9) - 4) as f64 * 4.,
                y: chunk as f64 * CHUNK_LENGTH + 31.,
            };
            let heading = endless::reference_heading(seed, &sim);
            assert!(heading.is_finite());
            assert!(heading.abs() <= std::f64::consts::FRAC_PI_2);
        }
    }
}

#[test]
fn production_engine_completes_a_long_seed_corpus_through_the_reserved_route() {
    let mut total_jumps = 0;
    for seed in 0..128 {
        let mut sim = Sim::default();
        sim.start();
        let mut active_chunk = u64::MAX;
        let mut obstacles = vec![];
        for _ in 0..12_000 {
            let chunk = (sim.position.y / CHUNK_LENGTH).floor() as u64;
            if chunk != active_chunk {
                active_chunk = chunk;
                obstacles = endless::obstacles(seed, sim.position.y);
            }
            let events = sim.step_mode(
                Input {
                    heading: endless::reference_heading(seed, &sim),
                    brake: false,
                },
                &obstacles,
                Mode::FreeSki,
            );
            total_jumps += events.iter().filter(|event| **event == Event::Jump).count();
            if sim.distance >= 5_000. {
                break;
            }
            assert!(!sim.ended(), "seed {seed} ended at {:.1} m", sim.distance);
        }
        assert!(
            sim.distance >= 5_000.,
            "seed {seed} reached only {:.1} m",
            sim.distance
        );
        assert_eq!(sim.crashes, 0, "seed {seed} left the reserved route");
        assert!(sim.valid_mode(&obstacles, Mode::FreeSki));
    }
    assert!(
        total_jumps >= 1_000,
        "corpus produced only {total_jumps} jumps"
    );
}
