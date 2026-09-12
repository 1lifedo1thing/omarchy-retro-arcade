use omarchy_blast::rules::*;
fn quiet() -> [Input; 4] {
    [Input::default(); 4]
}
fn empty() -> Arena {
    let mut a = Arena::new(0, 2, 2);
    for p in 0..N {
        if Arena::ring(p) > 0 {
            a.tiles[p] = Tile::Floor;
        }
    }
    a.hidden = [None; N];
    a
}
fn bomb(pos: usize, due: u32, range: u8, owner: usize) -> Bomb {
    Bomb {
        pos,
        due,
        range,
        owner,
        pass: 0,
    }
}
#[test]
fn walls_and_crates_stop_all_rays() {
    let mut a = empty();
    let p = 5 * W + 7;
    a.tiles[p + 2] = Tile::Wall;
    a.tiles[p - 2] = Tile::Crate;
    let cells = Arena::blast(&a.tiles, p, 6);
    assert!(cells.contains(&(p + 1)));
    assert!(!cells.contains(&(p + 2)));
    assert!(cells.contains(&(p - 2)));
    assert!(!cells.contains(&(p - 3)));
    assert!(!cells.contains(&7));
}
#[test]
fn chain_wave_is_order_independent_and_crate_shields_every_ray() {
    let mut a = empty();
    let p = 5 * W + 7;
    a.tiles[p + 3] = Tile::Crate;
    a.bombs = vec![
        bomb(p, 1, 3, 0),
        bomb(p + 2, 100, 5, 1),
        bomb(p - 2, 120, 3, 2),
    ];
    let mut b = a.clone();
    b.bombs.reverse();
    a.step(quiet());
    b.step(quiet());
    assert!(a.bombs.is_empty());
    assert!(b.bombs.is_empty());
    assert_eq!(a.tiles, b.tiles);
    assert_eq!(a.flames, b.flames);
    assert_eq!(a.tiles[p + 3], Tile::Floor);
    assert_eq!(a.flames[p + 4], 0);
}
#[test]
fn simultaneous_elimination_is_draw_and_transition_fires_once() {
    let mut a = empty();
    a.players.truncate(2);
    a.players[0].pos = 5 * W + 6;
    a.players[1].pos = 5 * W + 8;
    a.bombs.push(bomb(5 * W + 7, 1, 2, 0));
    let events = a.step(quiet());
    assert_eq!(a.outcome, Some(Outcome::Draw));
    assert!(events.contains(&Event::End));
    let before = a.clone();
    assert!(a.step(quiet()).is_empty());
    assert_eq!(a, before);
}
#[test]
fn leaving_your_bomb_revokes_pass_and_other_players_cannot_cross() {
    let mut a = empty();
    a.players[0].pos = 5 * W + 7;
    let mut i = quiet();
    i[0] = Input {
        direction: Some(Direction::Right),
        bomb: true,
    };
    a.step(i);
    assert_eq!(a.players[0].pos, 5 * W + 8);
    assert_eq!(a.bombs[0].pass, 0);
    assert!(!a.can_enter(0, 5 * W + 7));
    assert!(!a.can_enter(1, 5 * W + 7));
    i[0] = Input {
        direction: Some(Direction::Left),
        bomb: false,
    };
    for _ in 0..MOVE + 1 {
        a.step(i);
    }
    assert_eq!(a.players[0].pos, 5 * W + 8);
}
#[test]
fn contested_tiles_and_swaps_have_no_player_priority() {
    let mut a = empty();
    a.players[0].pos = 5 * W + 6;
    a.players[1].pos = 5 * W + 8;
    let mut i = quiet();
    i[0].direction = Some(Direction::Right);
    i[1].direction = Some(Direction::Left);
    a.step(i);
    assert_eq!(a.players[0].pos, 5 * W + 6);
    assert_eq!(a.players[1].pos, 5 * W + 8);
    a.players[1].pos -= 1;
    a.step(i);
    assert_eq!(a.players[0].pos, 5 * W + 6);
    assert_eq!(a.players[1].pos, 5 * W + 7);
}
#[test]
fn reveal_is_protected_until_flame_expires_then_caps_apply() {
    let mut a = empty();
    let p = 5 * W + 7;
    a.tiles[p] = Tile::Crate;
    a.hidden[p] = Some(Upgrade::Capacity);
    a.bombs.push(bomb(p - 1, 1, 1, 0));
    a.step(quiet());
    assert_eq!(a.upgrades[p], Some(Upgrade::Capacity));
    a.players[0].pos = p;
    a.step(quiet());
    assert!(!a.players[0].alive);
    assert_eq!(a.upgrades[p], Some(Upgrade::Capacity));
    for _ in 0..FLAME {
        a.step(quiet());
    }
    a.players[1].pos = p;
    a.players[1].capacity = MAX_CAPACITY;
    a.step(quiet());
    assert_eq!(a.upgrades[p], None);
    assert_eq!(a.players[1].capacity, MAX_CAPACITY);
    a.upgrades[p] = Some(Upgrade::Range);
    a.players[1].range = MAX_RANGE;
    a.step(quiet());
    assert_eq!(a.players[1].range, MAX_RANGE);
}
#[test]
fn movement_walls_crates_bombs_and_capacity_are_enforced() {
    let mut a = empty();
    let p = a.players[0].pos;
    a.tiles[p + 1] = Tile::Crate;
    let mut i = quiet();
    i[0] = Input {
        direction: Some(Direction::Right),
        bomb: true,
    };
    a.step(i);
    assert_eq!(a.players[0].pos, p);
    assert_eq!(a.bombs.len(), 1);
    i[0].direction = Some(Direction::Down);
    for _ in 0..25 {
        a.step(i);
    }
    assert_eq!(a.bombs.len(), 1);
    assert_eq!(a.players[0].pos, p + 3 * W);
}
#[test]
fn exact_fuse_and_forecast_include_chain_and_later_crate_removal() {
    let mut a = empty();
    let p = 5 * W + 5;
    a.tiles[p + 1] = Tile::Crate;
    a.bombs = vec![
        bomb(p, 5, 4, 0),
        bomb(p + 3, 15, 4, 1),
        bomb(p + 4, 100, 2, 2),
    ];
    let forecast = a.forecast(120);
    assert!(!forecast[4][p]);
    assert!(forecast[5][p]);
    assert!(!forecast[5][p + 2]);
    assert!(forecast[15][p + 2]);
    assert!(forecast[15][p + 4]);
    for time in 1..=120 {
        a.step(quiet());
        for (p, expected) in forecast[time as usize].iter().enumerate() {
            assert_eq!(
                *expected,
                a.flames[p] > a.tick || a.tiles[p] == Tile::Wall,
                "tick {time} cell {p}"
            );
        }
    }
}
#[test]
fn every_supported_spawn_has_two_escape_routes_and_safe_bomb_opening() {
    for arena in 0..3 {
        for (humans, bots) in [(1, 3), (2, 0), (2, 1), (2, 2)] {
            let a = Arena::new(arena, humans, bots);
            for p in &a.players {
                assert_eq!(a.tiles[p.pos], Tile::Floor);
                let exits: Vec<_> = Direction::ALL
                    .into_iter()
                    .filter_map(|d| d.next(p.pos))
                    .filter(|p| a.tiles[*p] == Tile::Floor)
                    .collect();
                assert_eq!(exits.len(), 2);
                for exit in exits {
                    assert!(Direction::ALL
                        .into_iter()
                        .filter_map(|d| d.next(exit))
                        .any(|next| next != p.pos && a.tiles[next] == Tile::Floor));
                }
            }
        }
    }
}
#[test]
fn sudden_death_is_warned_then_closes_all_players_simultaneously() {
    let mut a = Arena::new(0, 2, 0);
    a.tick = ROUND - 3 * HZ;
    assert!(a.warning(a.players[0].pos));
    assert!(!a.warning(5 * W + 7));
    a.tick = ROUND - 1;
    a.step(quiet());
    assert_eq!(a.outcome, Some(Outcome::Draw));
    assert_eq!(a.tiles[SPAWNS[0]], Tile::Wall);
}
#[test]
fn bots_plan_a_deliberate_bomb_and_escape_using_normal_movement() {
    let mut a = empty();
    a.players[0].bot = true;
    a.players[0].pos = 5 * W + 5;
    a.tiles[5 * W + 7] = Tile::Crate;
    let input = a.bot_inputs();
    assert!(input[0].bomb, "safe deliberate crate attack");
    a.step(input);
    let mut moved = false;
    for _ in 0..FUSE + FLAME + MOVE {
        let old = a.players[0].pos;
        let input = a.bot_inputs();
        a.step(input);
        if a.players[0].pos != old {
            moved = true;
            assert_eq!(
                (a.players[0].pos % W).abs_diff(old % W) + (a.players[0].pos / W).abs_diff(old / W),
                1
            );
        }
        assert!(a.players[0].alive, "bot failed escape at {}", a.tick);
    }
    assert!(moved);
    assert_eq!(a.tiles[5 * W + 7], Tile::Floor);
}
#[test]
fn trapped_bot_refuses_a_suicide_bomb() {
    let mut a = empty();
    let p = 5 * W + 5;
    a.players[0].pos = p;
    a.players[0].bot = true;
    for d in Direction::ALL {
        a.tiles[d.next(p).unwrap()] = Tile::Crate;
    }
    assert!(!a.bot_inputs()[0].bomb);
}
#[test]
fn match_counts_each_win_once_and_resets_round_state() {
    let mut m = Match::new(0, 2, 0);
    for n in 1..=3 {
        m.arena.players[1].alive = false;
        m.step(quiet());
        m.step(quiet());
        assert_eq!(m.wins[0], n);
        if n < 3 {
            m.next_round();
            assert_eq!(m.arena.tick, 0);
            assert!(m.arena.players.iter().all(|p| p.alive));
        }
    }
    assert_eq!(m.champion, Some(0));
    m.next_round();
    assert_eq!(m.rounds, 3);
    let fresh = Match::new(1, 1, 3);
    assert_eq!(fresh.wins, [0; 4]);
    assert_eq!(fresh.arena.arena, 1);
}
#[test]
fn bot_replays_are_deterministic_across_all_arenas_and_stay_legal() {
    for arena in 0..3 {
        let mut a = Arena::new(arena, 1, 3);
        a.players[0].bot = true;
        let mut b = a.clone();
        for _ in 0..1200 {
            let input = a.bot_inputs();
            assert_eq!(input, b.bot_inputs());
            a.step(input);
            b.step(input);
            for p in a.players.iter().filter(|p| p.alive) {
                assert_eq!(a.tiles[p.pos], Tile::Floor);
                assert!(a.flames[p.pos] <= a.tick);
            }
            assert_eq!(a, b);
            if a.outcome.is_some() {
                break;
            }
        }
    }
}
