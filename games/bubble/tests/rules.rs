use omarchy_bubble::rules::*;
fn level(rows: &[&str], magazine: &[u8], pressure_every: usize) -> Level {
    Level {
        name: "test".into(),
        rows: rows.iter().map(|s| s.to_string()).collect(),
        magazine: magazine.to_vec(),
        pressure_every,
    }
}
fn invariant(b: &Board) {
    for (i, c) in b.cells.iter().enumerate() {
        if c.is_some() {
            assert!(valid(i));
            for (j, d) in b.cells.iter().enumerate().skip(i + 1) {
                if d.is_some() {
                    assert!(
                        b.center(i).distance(b.center(j)) >= 2. * R - 1e-6,
                        "overlap {i}/{j}"
                    );
                }
            }
        }
    }
    if b.status == Status::Playing {
        assert_eq!(b.connected().len(), b.count());
        assert!(b.color(0).is_some_and(|c| b.colors().contains(&c)));
        assert!(b.color(1).is_some_and(|c| b.colors().contains(&c)));
    }
}
#[test]
fn hex_neighbors_are_symmetric_and_touching() {
    let b = Board::new(&level(&["0000000000"], &[0], 9)).unwrap();
    for i in 0..ROWS * COLS {
        if valid(i) {
            for j in neighbors(i) {
                assert!(valid(j));
                assert!(neighbors(j).contains(&i));
                assert!((b.center(i).distance(b.center(j)) - 40.).abs() < 1e-8);
            }
        }
    }
    assert_eq!(neighbors(0).len(), 2);
    assert_eq!(neighbors(14).len(), 6);
}
#[test]
fn match_and_win_are_exactly_once() {
    let mut b = Board::new(&level(&["....00...."], &[0], 1)).unwrap();
    let out = b.fire(0.).unwrap();
    assert_eq!(out.popped.len(), 3);
    assert_eq!(b.status, Status::Won);
    assert_eq!(b.score, 1300);
    assert_eq!(b.pressure, 0);
    assert!(b.fire(0.).is_none());
    assert_eq!(b.score, 1300);
    assert_eq!(b.shots, 1);
}
#[test]
fn only_connected_same_color_matches_pop() {
    let mut b = Board::new(&level(&["....01...."], &[0], 12)).unwrap();
    let out = b.fire(-1.).unwrap();
    assert!(out.popped.is_empty());
    assert_eq!(b.count(), 3);
    invariant(&b);
}
#[test]
fn unsupported_clusters_fall_and_score() {
    let l = levels().remove(2);
    let mut b = Board::new(&l).unwrap();
    let out = b.fire(4.).unwrap();
    assert_eq!(out.popped.len(), 4);
    assert_eq!(out.fallen.len(), 4);
    assert_eq!(b.score, 2200);
    assert_eq!(b.status, Status::Won);
}
#[test]
fn first_wall_bounce_obeys_reflection() {
    let b = Board::new(&level(&["....00...."], &[0], 12)).unwrap();
    for angle in [-65., 65.] {
        let s = b.trace(angle).unwrap();
        assert!(s.points.len() > 2);
        let p = s.points[1];
        assert!((p.x - R).abs() < 1e-6 || (p.x - (WIDTH - R)).abs() < 1e-6);
        let a = s.points[0];
        let c = s.points[2];
        assert!((((p.x - a.x) / (p.y - a.y)) + ((c.x - p.x) / (c.y - p.y))).abs() < 1e-6);
    }
}
#[test]
fn ceiling_uses_top_row_and_no_overwrite() {
    let b = Board::new(&level(&["00........"], &[0], 12)).unwrap();
    let shot = b.trace(0.).unwrap();
    assert!(shot.slot.unwrap() < COLS);
    assert!(b.cells[shot.slot.unwrap()].is_none());
    let mut b = b;
    b.attach(&shot);
    assert!(b.cells[0].is_some());
    invariant(&b);
}
#[test]
fn pressure_and_danger_end_once() {
    let mut b = Board::new(&level(&["....01...."], &[0, 1], 1)).unwrap();
    b.pressure = 13;
    b.fire(0.);
    assert_eq!(b.status, Status::Lost);
    let shots = b.shots;
    assert!(b.fire(0.).is_none());
    assert_eq!(shots, b.shots);
    let mut b = Board::new(&level(&["....01...."], &[0, 1], 1)).unwrap();
    b.fire(-1.);
    assert_eq!(b.pressure, 1);
    assert_eq!(b.pressure_in(), 1);
}
#[test]
fn empty_is_won_and_colours_track_remaining_board() {
    let b = Board::new(&level(&[".........."], &[5], 8)).unwrap();
    assert_eq!(b.status, Status::Won);
    assert_eq!(b.color(0), None);
    assert!(b.trace(0.).is_none());
    let b = Board::new(&level(&["....00...."], &[5, 4], 8)).unwrap();
    assert_eq!(b.color(0), Some(0));
    assert_eq!(b.color(1), Some(0));
}
#[test]
fn authored_completion_routes_replay_through_real_collisions() {
    let ls = levels();
    let routes: Vec<Vec<f64>> =
        serde_json::from_str(include_str!("../levels/routes.json")).unwrap();
    assert_eq!(ls.len(), 20);
    assert_eq!(routes.len(), ls.len());
    for (l, route) in ls.iter().zip(routes) {
        let mut b = Board::new(l).unwrap();
        invariant(&b);
        for angle in route {
            assert_eq!(b.status, Status::Playing, "{}", l.name);
            let shot = b.trace(angle).unwrap();
            assert!(shot.slot.is_some(), "{} {angle}", l.name);
            assert!(shot.points.len() <= 33);
            for offset in [-0.25, 0.25] {
                let mut nearby = b.clone();
                nearby.fire(angle + offset);
                let mut exact = b.clone();
                exact.fire(angle);
                assert_eq!(
                    nearby.cells, exact.cells,
                    "{} angle {} margin {}",
                    l.name, angle, offset
                );
                assert_eq!(nearby.status, exact.status);
            }
            b.fire(angle);
            invariant(&b);
        }
        assert_eq!(b.status, Status::Won, "{}", l.name);
    }
}
#[test]
fn every_half_degree_on_every_level_is_bounded_and_repeatable() {
    for l in levels() {
        let b = Board::new(&l).unwrap();
        for a in -156..=156 {
            let angle = a as f64 / 2.;
            let s = b.trace(angle).unwrap();
            let repeat = b.trace(angle).unwrap();
            assert_eq!(s.points, repeat.points);
            assert_eq!(s.slot, repeat.slot);
            assert!(s.points.len() <= 33);
            for p in &s.points {
                assert!(p.x.is_finite() && p.y.is_finite());
                assert!(p.x >= R - 1e-5 && p.x <= WIDTH - R + 1e-5);
            }
            let mut next = b.clone();
            next.attach(&s);
            invariant(&next);
        }
    }
}
#[test]
fn crowded_boards_never_overlap_or_leave_a_shot_unresolved() {
    let mut b = Board::new(&levels()[19]).unwrap();
    for shot in 0..90 {
        if b.status != Status::Playing {
            break;
        }
        let angle = ((shot * 47) % 157) as f64 - 78.;
        let s = b.trace(angle).unwrap();
        assert!(!s.points.is_empty());
        b.attach(&s);
        invariant(&b);
    }
    assert_ne!(b.status, Status::Playing);
}
#[test]
fn invalid_levels_and_angles_are_rejected() {
    assert!(Board::new(&level(&["....0.....", ".........", "....0....."], &[0], 5)).is_err());
    assert!(Board::new(&level(&["......"], &[0], 5)).is_err());
    assert!(Board::new(&level(&["0000000000"], &[], 5)).is_err());
    assert!(Board::new(&level(&["0000000000"], &[0], 0)).is_err());
    let b = Board::new(&levels()[0]).unwrap();
    assert!(b.trace(f64::NAN).is_none());
}
