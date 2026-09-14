use omarchy_freeski::{
    chase::{Chase, ChasePhase},
    endless,
    engine::{Event, Input, Mode, Phase, Point, Sim, MAX_SPEED},
    session::Session,
    storage::Save,
};

#[test]
fn session_matches_independently_captured_13fa3bf_physics_snapshots() {
    let mut crash = Session::new(Save::default());
    crash.state.run.start();
    while crash.state.run.tumble == 0 {
        crash.step(Input::default());
    }
    assert_eq!(crash.state.run.position, Point { x: -4., y: 357.95 });
    assert_eq!((crash.state.run.ticks, crash.state.run.crashes), (697, 1));

    let mut ramp = Session::new(Save::default());
    ramp.state.run.start();
    while !ramp.state.run.jump.is_some_and(|time| time > 0.3) {
        let x = if ramp.state.run.position.y < 280. {
            0.
        } else {
            -12.
        };
        let heading = (x - ramp.state.run.position.x).atan2(24.);
        ramp.step(Input {
            heading,
            brake: false,
        });
    }
    assert_eq!(ramp.state.run.ticks, 788);
    assert_eq!(
        ramp.state.run.position,
        Point {
            x: -11.975125883710705,
            y: 432.7142332724272
        }
    );
    assert_eq!(ramp.state.run.jump, Some(0.3032758679015278));

    let seed = 0x51_4b_49;
    let mut save = Save::default();
    save.select_mode(Mode::FreeSki, seed);
    save.run.start();
    let mut free = Session::new(save);
    while free.state.run.position.y < endless::CHUNK_LENGTH * 5.5 {
        let heading = endless::reference_heading(seed, &free.state.run);
        free.step(Input {
            heading,
            brake: false,
        });
    }
    assert_eq!(free.state.run.ticks, 1125);
    assert_eq!(
        free.state.run.position,
        Point {
            x: 6.80951531792883,
            y: 704.6253373973223
        }
    );
    assert_eq!(free.state.run.last_ramp, Some(192));
}

#[test]
fn paused_session_does_not_refresh_or_advance() {
    let mut save = Save::default();
    save.select_mode(Mode::FreeSki, 7);
    let mut session = Session::new(save);
    let state = session.state.clone();
    let obstacles = session.obstacles.clone();
    assert!(session.step(Input::default()).is_empty());
    assert_eq!(session.state, state);
    assert_eq!(session.obstacles, obstacles);
}

#[test]
fn catch_at_or_before_a_crash_discards_the_unreached_crash_and_recovery() {
    let seed = 41;
    let obstacle = endless::obstacles(seed, 0.)
        .into_iter()
        .filter(|obstacle| obstacle.kind != omarchy_freeski::world::Kind::Ramp)
        .min_by(|a, b| a.at.y.total_cmp(&b.at.y))
        .unwrap();
    let position = Point {
        x: obstacle.at.x,
        y: obstacle.at.y - obstacle.radius() - omarchy_freeski::engine::RADIUS - 0.01,
    };
    let save = Save {
        mode: Mode::FreeSki,
        seed,
        chase_enabled: true,
        chase: Chase {
            phase: ChasePhase::Active,
            position,
            speed: 0.,
            heading: 0.,
            warning_ticks: 0,
            retry_ticks: 0,
            failed_retries: 0,
        },
        run: Sim {
            phase: Phase::Running,
            position,
            speed: MAX_SPEED,
            distance: position.y,
            crashes: 1,
            ..Sim::default()
        },
        ..Save::default()
    };
    let mut session = Session::new(save);
    let previous_distance = session.state.run.distance;
    assert_eq!(
        session.step(Input {
            heading: omarchy_freeski::engine::MAX_HEADING,
            brake: false,
        }),
        vec![Event::Caught]
    );
    assert_eq!(session.state.run.phase, Phase::Caught);
    assert_eq!(session.state.run.crashes, 1);
    assert_eq!(session.state.run.distance, previous_distance);
    assert_eq!(session.state.run.tumble, 0);
    assert_eq!(
        session.state.run.heading,
        omarchy_freeski::engine::TURN_RATE * omarchy_freeski::engine::DT
    );
    assert!(session.state.result_recorded);
}

#[test]
fn session_refreshes_endless_cache_across_many_chunks() {
    let seed = 0x5151;
    let mut save = Save::default();
    save.select_mode(Mode::FreeSki, seed);
    save.run.start();
    let mut session = Session::new(save);
    let initial_ids: Vec<_> = session
        .obstacles
        .iter()
        .map(|obstacle| obstacle.id)
        .collect();
    for _ in 0..20_000 {
        let heading = endless::reference_heading(seed, &session.state.run);
        session.step(Input {
            heading,
            brake: false,
        });
        if session.state.run.distance > endless::CHUNK_LENGTH * 4. {
            break;
        }
    }
    assert!(session.state.run.distance > endless::CHUNK_LENGTH * 4.);
    assert_ne!(
        session
            .obstacles
            .iter()
            .map(|obstacle| obstacle.id)
            .collect::<Vec<_>>(),
        initial_ids
    );
}

#[test]
fn chase_cache_is_the_bounded_union_of_skier_and_creature_windows() {
    let seed = 90210;
    let mut save = Save::default();
    save.select_chase(true, seed);
    save.run = Sim {
        phase: Phase::Paused,
        position: Point { x: 0., y: 2_000. },
        distance: 2_000.,
        ..Sim::default()
    };
    save.chase = Chase {
        phase: ChasePhase::Active,
        position: Point { x: 0., y: 1_600. },
        speed: 30.,
        heading: 0.,
        warning_ticks: 0,
        retry_ticks: 0,
        failed_retries: 0,
    };
    let session = Session::new(save);
    let ids: std::collections::BTreeSet<_> = session
        .obstacles
        .iter()
        .map(|obstacle| obstacle.id)
        .collect();
    for obstacle in endless::obstacles(seed, 2_000.)
        .into_iter()
        .chain(endless::obstacles(seed, 1_600.))
    {
        assert!(ids.contains(&obstacle.id));
    }
    assert_eq!(ids.len(), session.obstacles.len());
    assert!(session.obstacles.len() <= 144);
}
