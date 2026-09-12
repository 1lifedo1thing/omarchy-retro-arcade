//! Generate inspectable native save fixtures through ordinary production inputs.
use omarchy_freeski::{
    engine::{Input, Phase},
    storage::{self, Save},
    world,
};
fn main() {
    let dir = std::path::PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    let obstacles = world::practice();
    // A straight run reaches the braking-section tree and exercises actual recovery.
    let mut save = Save::default();
    save.run.start();
    for _ in 0..12000 {
        save.run.step(Input::default(), &obstacles);
        if save.run.tumble > 0 {
            storage::write(&dir.join("recovery.json"), &save, &obstacles).unwrap();
            break;
        }
    }
    assert!(save.run.tumble > 0);
    // Aim for the first ramp with bounded normal steering, then capture in flight.
    save = Save::default();
    save.run.start();
    let mut captured = false;
    for _ in 0..12000 {
        let target_x = if save.run.position.y < 280. { 0. } else { -12. };
        let heading = (target_x - save.run.position.x).atan2(24.);
        save.run.step(
            Input {
                heading,
                brake: false,
            },
            &obstacles,
        );
        if save.run.jump.is_some_and(|t| t > 0.3) {
            storage::write(&dir.join("mid-jump.json"), &save, &obstacles).unwrap();
            captured = true;
            break;
        }
        assert!(!save.run.ended());
    }
    assert!(captured);
    assert_eq!(save.run.phase, Phase::Running);
    println!(
        "Generated mid-jump and recovery saves through production controls in {}",
        dir.display()
    );
}
