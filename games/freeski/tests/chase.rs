use omarchy_freeski::{
    chase::{
        Chase, ChaseEvent, ChasePhase, TickInput, CATCH_RADIUS, MAX_FAILED_RETRIES,
        RECOVERY_SAFE_GAP, SPAWN_RETRY_TICKS, TRIGGER_DISTANCE, WARNING_TICKS,
    },
    endless,
    engine::{Event, Input, Phase, Point, Sim},
    session::Session,
    storage::Save,
    world::{Kind, Obstacle, HALF_WIDTH},
};

fn tick<'a>(
    start: Point,
    end: Point,
    distance: f64,
    speed: f64,
    protected: bool,
    obstacles: &'a [Obstacle],
) -> TickInput<'a> {
    TickInput {
        skier_start: start,
        skier_end: end,
        skier_distance: distance,
        skier_speed: speed,
        protected,
        obstacles,
    }
}

fn active(at: Point, speed: f64, heading: f64) -> Chase {
    Chase {
        phase: ChasePhase::Active,
        position: at,
        speed,
        heading,
        warning_ticks: 0,
        retry_ticks: 0,
        failed_retries: 0,
    }
}

#[test]
fn threshold_emits_one_warning_then_spawns_after_simulation_ticks() {
    let skier = Point { x: 3., y: 1_000. };
    let below = Chase::default()
        .plan_tick(tick(
            skier,
            skier,
            TRIGGER_DISTANCE - 0.001,
            40.,
            false,
            &[],
        ))
        .finish();
    assert_eq!(below, Chase::default());

    let warning = below.plan_tick(tick(skier, skier, TRIGGER_DISTANCE, 40., false, &[]));
    assert_eq!(warning.events, vec![ChaseEvent::Warning]);
    let mut chase = warning.finish();
    assert_eq!(chase.phase, ChasePhase::Warning);
    assert_eq!(chase.warning_ticks, WARNING_TICKS);

    for _ in 0..WARNING_TICKS {
        let plan = chase.plan_tick(tick(skier, skier, skier.y, 40., false, &[]));
        assert!(plan.events.is_empty());
        chase = plan.finish();
    }
    let spawn = chase.plan_tick(tick(skier, skier, skier.y, 40., false, &[]));
    assert_eq!(spawn.events, vec![ChaseEvent::Spawned]);
    chase = spawn.finish();
    assert_eq!(chase.phase, ChasePhase::Active);
    assert!(chase.position.y < skier.y - CATCH_RADIUS);
    assert!(chase.valid());
}

#[test]
fn blocked_spawn_search_is_bounded_and_retries_without_invalid_state() {
    let skier = Point { x: 0., y: 1_100. };
    let mut blockers = Vec::new();
    for (id, gap) in [48., 54., 60., 42.].into_iter().enumerate() {
        for x_index in -20..=20 {
            blockers.push(Obstacle {
                id: id * 100 + (x_index + 20) as usize,
                at: Point {
                    x: x_index as f64 * 2.,
                    y: skier.y - gap,
                },
                kind: Kind::Tree,
            });
        }
    }
    let mut chase = Chase {
        phase: ChasePhase::Warning,
        ..Chase::default()
    };
    for expected in 1..=MAX_FAILED_RETRIES {
        let plan = chase.plan_tick(tick(skier, skier, skier.y, 0., false, &blockers));
        assert!(plan.events.is_empty());
        chase = plan.finish();
        assert_eq!(chase.failed_retries, expected);
        assert_eq!(chase.retry_ticks, SPAWN_RETRY_TICKS);
        for _ in 0..SPAWN_RETRY_TICKS {
            chase = chase
                .plan_tick(tick(skier, skier, skier.y, 0., false, &blockers))
                .finish();
        }
    }
    // The counter saturates and the actor remains a valid warning, rather than
    // overflowing or spawning inside blocked terrain.
    chase = chase
        .plan_tick(tick(skier, skier, skier.y, 0., false, &blockers))
        .finish();
    assert_eq!(chase.phase, ChasePhase::Warning);
    assert_eq!(chase.failed_retries, MAX_FAILED_RETRIES);
    assert!(chase.valid());
}

#[test]
fn relative_sweep_detects_crossing_between_tick_endpoints() {
    let chase = active(Point { x: 0., y: 100. }, 56., 0.);
    let skier_start = Point { x: -2., y: 100. };
    let skier_end = Point { x: 2., y: 100. };
    let plan = chase.plan_tick(tick(skier_start, skier_end, 1_200., 50., false, &[]));
    let contact = plan.catch_fraction.expect("paths cross within the tick");
    assert!((0. ..=1.).contains(&contact));
    let caught = plan.commit(contact);
    assert!(caught.position.x.abs() <= 0.1);
}

#[test]
fn obstacle_contacts_stop_the_actor_instead_of_phasing() {
    let tree = Obstacle {
        id: 7,
        at: Point { x: 0., y: 102.5 },
        kind: Kind::Tree,
    };
    let chase = active(Point { x: 0., y: 100. }, 56., 0.);
    let skier = Point { x: 0., y: 180. };
    let plan = chase.plan_tick(tick(skier, skier, 1_200., 50., false, &[tree]));
    let end = plan.creature_end.expect("active actor moves");
    assert!(end.y < tree.at.y);
    assert!((end.x - tree.at.x).hypot(end.y - tree.at.y) >= tree.radius() + 0.95 - 1e-5);
    assert!(plan.finish().speed < chase.speed);
}

#[test]
fn protection_holds_a_physical_gap_and_expiry_is_not_an_instant_catch() {
    let skier = Point { x: 0., y: 200. };
    let mut chase = active(
        Point {
            x: 0.,
            y: skier.y - RECOVERY_SAFE_GAP - 0.2,
        },
        56.,
        0.,
    );
    for _ in 0..120 {
        let plan = chase.plan_tick(tick(skier, skier, 1_200., 0., true, &[]));
        assert!(plan.catch_fraction.is_none());
        chase = plan.finish();
        assert!(
            (chase.position.x - skier.x).hypot(chase.position.y - skier.y)
                >= RECOVERY_SAFE_GAP - 1e-4
        );
    }
    let first_unprotected = chase.plan_tick(tick(skier, skier, 1_200., 0., false, &[]));
    assert_ne!(first_unprotected.catch_fraction, Some(0.));
}

#[test]
fn committed_turns_gain_separation_but_straight_skiing_is_caught() {
    fn run(evasive: bool) -> (bool, usize, f64, f64) {
        let mut skier = Point { x: 0., y: 1_100. };
        let mut chase = active(Point { x: 0., y: 1_052. }, 56., 0.);
        let initial = (skier.x - chase.position.x).hypot(skier.y - chase.position.y);
        let mut widest = initial;
        for frame in 0_usize..1_800 {
            let start = skier;
            let heading: f64 = if evasive && (frame / 90) % 2 == 0 {
                0.55
            } else if evasive {
                -0.55
            } else {
                0.
            };
            skier.x =
                (skier.x + heading.sin() * 50. / 60.).clamp(-HALF_WIDTH + 0.65, HALF_WIDTH - 0.65);
            skier.y += heading.cos() * 50. / 60.;
            let plan = chase.plan_tick(tick(start, skier, skier.y, 50., false, &[]));
            if let Some(contact) = plan.catch_fraction {
                chase = plan.commit(contact);
                let separation = (skier.x - chase.position.x).hypot(skier.y - chase.position.y);
                return (true, frame, widest, separation);
            }
            chase = plan.finish();
            widest = widest.max((skier.x - chase.position.x).hypot(skier.y - chase.position.y));
        }
        (
            false,
            1_800,
            widest,
            (skier.x - chase.position.x).hypot(skier.y - chase.position.y),
        )
    }

    let straight = run(false);
    let evasive = run(true);
    assert!(
        straight.0,
        "higher straight speed must eventually close the gap"
    );
    assert!(
        evasive.1 > straight.1 + 60,
        "committed turns should create measurable separation: {straight:?} / {evasive:?}"
    );
}

#[test]
fn actor_cannot_claim_infinite_immunity_at_a_slope_edge() {
    let edge = HALF_WIDTH - 0.65;
    let mut skier = Point { x: edge, y: 1_100. };
    let mut chase = active(
        Point {
            x: edge - 8.,
            y: 1_055.,
        },
        30.,
        0.,
    );
    let mut caught = false;
    for _ in 0..1_800 {
        let start = skier;
        skier.y += 50. / 60.;
        let plan = chase.plan_tick(tick(start, skier, skier.y, 50., false, &[]));
        if let Some(contact) = plan.catch_fraction {
            chase = plan.commit(contact);
            caught = true;
            break;
        }
        chase = plan.finish();
    }
    assert!(caught, "edge corridor must not make pursuit impossible");
    assert!(chase.position.x.abs() <= HALF_WIDTH - 0.95 + 1e-8);
}

#[test]
fn serialized_pursuit_round_trips_and_invalid_numbers_are_rejected() {
    let chase = active(Point { x: 4., y: 1_234. }, 41., -0.4);
    let bytes = serde_json::to_vec(&chase).unwrap();
    let restored: Chase = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(restored, chase);
    assert!(restored.valid());

    let mut invalid = restored;
    invalid.position.x = f64::NAN;
    assert!(!invalid.valid());
}

fn chase_session() -> Session {
    let mut state = Save::default();
    state.select_chase(true, 17);
    state.run = Sim {
        phase: Phase::Running,
        position: Point { x: 0., y: 1_100. },
        speed: 50.,
        distance: 1_100.,
        ..Sim::default()
    };
    state.chase = active(Point { x: 0., y: 1_052. }, 56., 0.);
    let mut session = Session::new(state);
    // A clear derived window isolates pursuit policy while retaining the exact
    // production Session and skier physics path.
    session.obstacles.clear();
    session
}

#[test]
fn production_session_freezes_warning_and_pursuit_while_paused() {
    let mut state = Save::default();
    state.select_chase(true, 11);
    state.run.phase = Phase::Running;
    state.run.position.y = TRIGGER_DISTANCE;
    state.run.distance = TRIGGER_DISTANCE;
    let mut session = Session::new(state);
    assert_eq!(session.step(Input::default()), vec![Event::Warning]);
    let warning = session.state.chase.clone();
    session.state.run.pause();
    for _ in 0..600 {
        assert!(session.step(Input::default()).is_empty());
    }
    assert_eq!(session.state.chase, warning);

    session.state.run.start();
    for _ in 0..WARNING_TICKS {
        session.step(Input::default());
    }
    assert_eq!(session.state.chase.phase, ChasePhase::Warning);
    assert_eq!(session.step(Input::default()), vec![Event::Spawn]);
    let pursuit = session.state.chase.clone();
    session.state.run.pause();
    session.step(Input::default());
    assert_eq!(session.state.chase, pursuit);
}

#[test]
fn production_session_catches_a_straight_run_and_evasive_turns_buy_space() {
    fn run(evasive: bool) -> (bool, usize, f64, usize) {
        let mut session = chase_session();
        let mut widest: f64 = 0.;
        let mut prior = 48.;
        let mut gain_streak = 0;
        let mut longest_gain = 0;
        for tick in 0_usize..2_400 {
            session.obstacles.clear();
            let heading = if !evasive {
                0.
            } else if (tick / 90) % 2 == 0 {
                0.55
            } else {
                -0.55
            };
            let events = session.step(Input {
                heading,
                brake: false,
            });
            let separation = (session.state.run.position.x - session.state.chase.position.x)
                .hypot(session.state.run.position.y - session.state.chase.position.y);
            widest = widest.max(separation);
            if separation > prior + 1e-6 {
                gain_streak += 1;
                longest_gain = longest_gain.max(gain_streak);
            } else {
                gain_streak = 0;
            }
            prior = separation;
            if events.contains(&Event::Caught) {
                return (true, tick, widest, longest_gain);
            }
        }
        (false, 2_400, widest, longest_gain)
    }

    let straight = run(false);
    let evasive = run(true);
    assert!(straight.0, "straight capped-speed skiing should be caught");
    assert!(evasive.1 > straight.1 + 60, "{straight:?} / {evasive:?}");
    assert!(
        evasive.3 >= 30,
        "evasive carving should increase separation for at least half a second: {evasive:?}"
    );
}

#[test]
fn serialized_warning_and_pursuit_continue_tick_for_tick() {
    fn round_trip(state: &Save) -> Save {
        serde_json::from_slice(&serde_json::to_vec(state).unwrap()).unwrap()
    }

    let mut warning_state = Save::default();
    warning_state.select_chase(true, 29);
    warning_state.run.phase = Phase::Running;
    warning_state.run.position.y = TRIGGER_DISTANCE;
    warning_state.run.distance = TRIGGER_DISTANCE;
    let mut warning_a = Session::new(warning_state);
    warning_a.step(Input::default());
    for _ in 0..47 {
        warning_a.step(Input::default());
    }
    let mut warning_b = Session::new(round_trip(&warning_a.state));
    for _ in 0..WARNING_TICKS + 30 {
        let a = warning_a.step(Input::default());
        let b = warning_b.step(Input::default());
        assert_eq!(a, b);
        assert_eq!(warning_a.state, warning_b.state);
    }

    let mut pursuit_a = chase_session();
    let mut pursuit_b = Session::new(round_trip(&pursuit_a.state));
    for tick in 0..240 {
        pursuit_a.obstacles.clear();
        pursuit_b.obstacles.clear();
        let input = Input {
            heading: if (tick / 60) % 2 == 0 { 0.45 } else { -0.45 },
            brake: false,
        };
        assert_eq!(pursuit_a.step(input), pursuit_b.step(input));
        assert_eq!(pursuit_a.state, pursuit_b.state);
        if pursuit_a.state.run.ended() {
            break;
        }
    }
}

#[test]
fn production_session_preserves_recovery_gap_until_protection_expires() {
    let mut session = chase_session();
    session.state.run.speed = 0.;
    session.state.run.tumble = 42;
    session.state.run.protection = 90;
    session.state.chase = active(
        Point {
            x: 0.,
            y: session.state.run.position.y - RECOVERY_SAFE_GAP - 0.2,
        },
        56.,
        0.,
    );
    for _ in 0..132 {
        session.obstacles.clear();
        let events = session.step(Input::default());
        assert!(!events.contains(&Event::Caught));
        let gap = (session.state.run.position.x - session.state.chase.position.x)
            .hypot(session.state.run.position.y - session.state.chase.position.y);
        assert!(gap >= RECOVERY_SAFE_GAP - 1e-4);
    }
    session.obstacles.clear();
    assert!(!session.step(Input::default()).contains(&Event::Caught));
}

#[test]
fn production_run_from_rest_quantifies_straight_and_evasive_pursuit() {
    let mut state = Save::default();
    state.select_chase(true, 17);
    state.run.start();
    let mut baseline = Session::new(state);
    let mut warnings = 0;
    let mut spawn_tick = None;
    for _ in 0..8_000 {
        let heading = endless::reference_heading(17, &baseline.state.run);
        let events = baseline.step(Input {
            heading,
            brake: false,
        });
        warnings += events
            .iter()
            .filter(|event| **event == Event::Warning)
            .count();
        if events.contains(&Event::Spawn) {
            spawn_tick = Some(baseline.state.run.ticks);
            break;
        }
        assert!(!baseline.state.run.ended());
    }
    assert_eq!(warnings, 1);
    assert!(
        spawn_tick.is_some(),
        "chase did not spawn from a normal run"
    );
    assert_eq!(baseline.state.run.crashes, 0);

    fn branch(mut session: Session, evasive: bool) -> (u64, f64, Phase, usize) {
        let start_tick = session.state.run.ticks;
        let mut closest = f64::INFINITY;
        let mut prior = (session.state.run.position.x - session.state.chase.position.x)
            .hypot(session.state.run.position.y - session.state.chase.position.y);
        let mut gain_streak = 0;
        let mut longest_gain = 0;
        for _ in 0_u64..2_400 {
            let base = endless::reference_heading(session.state.seed, &session.state.run);
            let heading = if evasive { base } else { 0. };
            session.step(Input {
                heading,
                brake: false,
            });
            let separation = (session.state.run.position.x - session.state.chase.position.x)
                .hypot(session.state.run.position.y - session.state.chase.position.y);
            closest = closest.min(separation);
            if separation > prior + 1e-6 {
                gain_streak += 1;
                longest_gain = longest_gain.max(gain_streak);
            } else {
                gain_streak = 0;
            }
            prior = separation;
            if session.state.run.ended() {
                break;
            }
        }
        (
            session.state.run.ticks - start_tick,
            closest,
            session.state.run.phase,
            longest_gain,
        )
    }

    let straight = branch(Session::new(round_trip_save(&baseline.state)), false);
    let evasive = branch(Session::new(round_trip_save(&baseline.state)), true);
    assert_eq!(straight.2, Phase::Caught, "straight: {straight:?}");
    assert!(
        evasive.0 > straight.0 + 60,
        "deliberate turns should buy at least one second: {straight:?} / {evasive:?}"
    );
    assert!(
        evasive.3 >= 30,
        "the evasive route should grow separation for at least half a second: {evasive:?}"
    );
}

fn round_trip_save(state: &Save) -> Save {
    serde_json::from_slice(&serde_json::to_vec(state).unwrap()).unwrap()
}
