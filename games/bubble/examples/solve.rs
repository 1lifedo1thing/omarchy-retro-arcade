use omarchy_bubble::rules::{levels, Board, Status};
use std::collections::HashSet;
fn main() {
    let mut routes = vec![];
    for (index, level) in levels().iter().enumerate() {
        let mut beam = vec![(Board::new(level).unwrap(), vec![])];
        let mut solution = None;
        'search: for _ in 0..50 {
            let mut next = vec![];
            let mut seen = HashSet::new();
            for (board, path) in &beam {
                let mut angles: Vec<i32> = (-156..=156).collect();
                angles.sort_by_key(|a| a.abs());
                for angle in angles {
                    let angle = angle as f64 / 2.;
                    let mut b = board.clone();
                    b.fire(angle);
                    // A playable route has an aiming margin, not a single boundary pixel.
                    let robust = [-0.25, 0.25].into_iter().all(|offset| {
                        let mut nearby = board.clone();
                        nearby.fire(angle + offset);
                        nearby.cells == b.cells && nearby.status == b.status
                    });
                    if !robust {
                        continue;
                    }
                    if b.status == Status::Lost {
                        continue;
                    }
                    let mut p = path.clone();
                    p.push(angle);
                    if b.status == Status::Won {
                        solution = Some(p);
                        break 'search;
                    }
                    if seen.insert((b.cells, b.pressure)) {
                        next.push((b, p));
                    }
                }
            }
            next.sort_by_key(|(b, _)| (b.count() * 100 + b.pressure * 60, b.shots));
            next.truncate(40);
            beam = next;
            if beam.is_empty() {
                break;
            }
        }
        let route = solution.unwrap_or_else(|| panic!("No solution for {}", level.name));
        eprintln!(
            "{} {}: {} shots {:?}",
            index + 1,
            level.name,
            route.len(),
            route
        );
        routes.push(route);
    }
    println!("{}", serde_json::to_string_pretty(&routes).unwrap());
}
