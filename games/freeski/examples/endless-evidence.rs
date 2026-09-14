//! Generate inspectable endless-mode saves through ordinary production inputs.
use omarchy_freeski::{
    endless::{self, CHUNK_LENGTH},
    engine::{Input, Mode, Phase, Sim},
    storage::{self, Save},
    world::{Kind, Obstacle},
};

const EVIDENCE_SEED: u64 = 0x4652_4545_534b_4903;

fn refresh(seed: u64, sim: &Sim, chunk: &mut u64, obstacles: &mut Vec<Obstacle>) {
    let current = (sim.position.y / CHUNK_LENGTH).floor() as u64;
    if current != *chunk {
        *chunk = current;
        *obstacles = endless::obstacles(seed, sim.position.y);
    }
}

fn fresh_run() -> Save {
    let mut save = Save::default();
    save.select_mode(Mode::FreeSki, EVIDENCE_SEED);
    save.run.start();
    save
}

fn write(output: &std::path::Path, name: &str, save: &Save, obstacles: &[Obstacle]) {
    storage::write(&output.join(name), save, obstacles).unwrap();
}

fn main() {
    let output = std::path::PathBuf::from(
        std::env::args_os()
            .nth(1)
            .expect("output directory argument"),
    );
    std::fs::create_dir_all(&output).unwrap();

    let mut save = fresh_run();
    let mut chunk = u64::MAX;
    let mut obstacles = vec![];
    let mut captured_jump = false;
    for _ in 0..20_000 {
        refresh(EVIDENCE_SEED, &save.run, &mut chunk, &mut obstacles);
        let heading = endless::reference_heading(EVIDENCE_SEED, &save.run);
        save.run.step_mode(
            Input {
                heading,
                brake: false,
            },
            &obstacles,
            Mode::FreeSki,
        );
        if !captured_jump && save.run.jump.is_some_and(|elapsed| elapsed > 0.3) {
            write(&output, "free-mid-jump.json", &save, &obstacles);
            captured_jump = true;
        }
        if save.run.distance > 1_600. {
            write(&output, "free-long-run.json", &save, &obstacles);
            break;
        }
        assert!(!save.run.ended());
    }
    assert!(captured_jump);
    assert!(save.run.distance > 1_600.);
    assert_eq!(save.run.phase, Phase::Running);

    // Deliberately steer from the ordinary start into the first non-ramp
    // obstacle, then capture the engine-selected clear recovery position.
    save = fresh_run();
    chunk = u64::MAX;
    obstacles.clear();
    let target = endless::obstacles(EVIDENCE_SEED, 0.)
        .into_iter()
        .filter(|obstacle| obstacle.kind != Kind::Ramp)
        .min_by(|a, b| a.at.y.total_cmp(&b.at.y))
        .expect("opening chunk has a deliberate collision target");
    for _ in 0..8_000 {
        refresh(EVIDENCE_SEED, &save.run, &mut chunk, &mut obstacles);
        let heading =
            (target.at.x - save.run.position.x).atan2((target.at.y - save.run.position.y).max(2.));
        save.run.step_mode(
            Input {
                heading,
                brake: false,
            },
            &obstacles,
            Mode::FreeSki,
        );
        if save.run.tumble > 0 {
            write(&output, "free-recovery.json", &save, &obstacles);
            break;
        }
        assert!(!save.run.ended());
    }
    assert!(save.run.tumble > 0);
    assert_eq!(save.run.crashes, 1);

    println!(
        "Generated endless long-run, mid-jump and recovery saves through production controls in {}",
        output.display()
    );
}
