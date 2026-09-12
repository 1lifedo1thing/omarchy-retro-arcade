//! Integer, fixed-step rules. No clocks, input devices, audio or rendering.
use std::collections::{BTreeSet, VecDeque};
pub const W: usize = 15;
pub const H: usize = 11;
pub const N: usize = W * H;
pub const HZ: u32 = 60;
pub const MOVE: u32 = 10;
pub const FUSE: u32 = 150;
pub const FLAME: u32 = 30;
pub const ROUND: u32 = 120 * HZ;
pub const COLLAPSE: u32 = 5 * HZ;
pub const MAX_CAPACITY: u8 = 4;
pub const MAX_RANGE: u8 = 6;
pub const SPAWNS: [usize; 4] = [W + 1, (H - 2) * W + W - 2, W + W - 2, (H - 2) * W + 1];
pub const ARENAS: [&str; 3] = ["Copper Yard", "Crossroads", "The Foundry"];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
    Crate,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upgrade {
    Capacity,
    Range,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
    pub fn next(self, p: usize) -> Option<usize> {
        let (x, y) = (p % W, p / W);
        match self {
            Self::Up if y > 0 => Some(p - W),
            Self::Down if y + 1 < H => Some(p + W),
            Self::Left if x > 0 => Some(p - 1),
            Self::Right if x + 1 < W => Some(p + 1),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Input {
    pub direction: Option<Direction>,
    pub bomb: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub pos: usize,
    pub alive: bool,
    pub bot: bool,
    pub capacity: u8,
    pub range: u8,
    pub ready: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bomb {
    pub pos: usize,
    pub owner: usize,
    pub due: u32,
    pub range: u8,
    pub pass: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Winner(usize),
    Draw,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Place,
    Explode,
    Pickup,
    End,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arena {
    pub tiles: [Tile; N],
    pub hidden: [Option<Upgrade>; N],
    pub upgrades: [Option<Upgrade>; N],
    pub flames: [u32; N],
    pub bombs: Vec<Bomb>,
    pub players: Vec<Player>,
    pub tick: u32,
    pub outcome: Option<Outcome>,
    pub arena: usize,
}
impl Arena {
    /// Authored, deterministic patterns. Every spawn has two three-tile exits.
    pub fn new(arena: usize, humans: usize, bots: usize) -> Self {
        assert!(
            arena < ARENAS.len() && (1..=2).contains(&humans) && (2..=4).contains(&(humans + bots))
        );
        let mut tiles = [Tile::Floor; N];
        let mut hidden = [None; N];
        for p in 0..N {
            let (x, y) = (p % W, p / W);
            let wall = x == 0 || y == 0 || x == W - 1 || y == H - 1 || (x % 2 == 0 && y % 2 == 0);
            let crate_here = match arena {
                0 => (x + y * 3) % 5 != 0,
                1 => x != W / 2 && y != H / 2 && (x * 3 + y) % 4 != 0,
                _ => (x + y) % 3 != 0 || (5..=9).contains(&x),
            };
            tiles[p] = if wall {
                Tile::Wall
            } else if crate_here {
                Tile::Crate
            } else {
                Tile::Floor
            };
            if tiles[p] == Tile::Crate && (x * 7 + y * 11) % 3 == 0 {
                hidden[p] = Some(if (x + y) % 2 == 0 {
                    Upgrade::Capacity
                } else {
                    Upgrade::Range
                });
            }
        }
        for spawn in SPAWNS {
            for (p, tile) in tiles.iter_mut().enumerate() {
                let dist = (p % W).abs_diff(spawn % W) + (p / W).abs_diff(spawn / W);
                if dist <= 3 && *tile != Tile::Wall {
                    *tile = Tile::Floor;
                    hidden[p] = None;
                }
            }
        }
        Self {
            tiles,
            hidden,
            upgrades: [None; N],
            flames: [0; N],
            bombs: Vec::new(),
            players: SPAWNS
                .into_iter()
                .take(humans + bots)
                .enumerate()
                .map(|(i, pos)| Player {
                    pos,
                    alive: true,
                    bot: i >= humans,
                    capacity: 1,
                    range: 2,
                    ready: 0,
                })
                .collect(),
            tick: 0,
            outcome: None,
            arena,
        }
    }
    pub fn blast(tiles: &[Tile; N], pos: usize, range: u8) -> BTreeSet<usize> {
        let mut cells = BTreeSet::from([pos]);
        for direction in Direction::ALL {
            let mut p = pos;
            for _ in 0..range {
                let Some(next) = direction.next(p) else {
                    break;
                };
                p = next;
                if tiles[p] == Tile::Wall {
                    break;
                }
                cells.insert(p);
                if tiles[p] == Tile::Crate {
                    break;
                }
            }
        }
        cells
    }
    pub fn ring(p: usize) -> usize {
        (p % W).min(W - 1 - p % W).min(p / W).min(H - 1 - p / W)
    }
    pub fn collapse_at(p: usize) -> u32 {
        ROUND + Self::ring(p).saturating_sub(1) as u32 * COLLAPSE
    }
    pub fn warning(&self, p: usize) -> bool {
        self.tiles[p] != Tile::Wall
            && Self::collapse_at(p) > self.tick
            && Self::collapse_at(p) <= self.tick + 3 * HZ
    }
    pub fn can_enter(&self, id: usize, pos: usize) -> bool {
        self.tiles[pos] == Tile::Floor
            && !self
                .bombs
                .iter()
                .any(|b| b.pos == pos && b.pass & (1 << id) == 0)
    }
    fn place(&mut self, id: usize) -> bool {
        let p = &self.players[id];
        if !p.alive
            || self.bombs.iter().any(|b| b.pos == p.pos)
            || self.bombs.iter().filter(|b| b.owner == id).count() >= p.capacity as usize
        {
            return false;
        }
        self.bombs.push(Bomb {
            pos: p.pos,
            owner: id,
            due: self.tick + FUSE,
            range: p.range,
            pass: 1 << id,
        });
        true
    }
    /// Explosions use a common tile snapshot; a crate blocks every blast in this wave.
    fn hazards(&mut self) -> bool {
        for p in 0..N {
            if self.tick >= Self::collapse_at(p) && Self::ring(p) > 0 {
                self.tiles[p] = Tile::Wall;
                self.hidden[p] = None;
                self.upgrades[p] = None;
            }
        }
        self.bombs.retain(|b| self.tiles[b.pos] != Tile::Wall);
        let mut firing = BTreeSet::new();
        let mut hit = BTreeSet::new();
        loop {
            let mut changed = false;
            for (i, bomb) in self.bombs.iter().enumerate() {
                if !firing.contains(&i)
                    && (bomb.due <= self.tick
                        || self.flames[bomb.pos] > self.tick
                        || hit.contains(&bomb.pos))
                {
                    firing.insert(i);
                    hit.extend(Self::blast(&self.tiles, bomb.pos, bomb.range));
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        for p in hit {
            self.flames[p] = self.tick + FLAME;
            if self.tiles[p] == Tile::Crate {
                self.tiles[p] = Tile::Floor;
                self.upgrades[p] = self.hidden[p].take();
            }
        }
        let mut i = 0;
        self.bombs.retain(|_| {
            let keep = !firing.contains(&i);
            i += 1;
            keep
        });
        !firing.is_empty()
    }
    fn eliminate(&mut self) {
        for p in &mut self.players {
            if self.flames[p.pos] > self.tick || self.tiles[p.pos] == Tile::Wall {
                p.alive = false;
            }
        }
    }
    pub fn step(&mut self, input: [Input; 4]) -> Vec<Event> {
        if self.outcome.is_some() {
            return Vec::new();
        }
        self.tick += 1;
        let mut events = Vec::new();
        if self.hazards() {
            events.push(Event::Explode);
        }
        self.eliminate();
        for (i, action) in input.iter().enumerate().take(self.players.len()) {
            if action.bomb && self.place(i) {
                events.push(Event::Place);
            }
        }
        // Snapshot occupancy: no swaps, no priority for a player index, no stacking.
        let mut targets = [None; 4];
        for (i, p) in self.players.iter().enumerate() {
            if !p.alive || p.ready > self.tick {
                continue;
            }
            if let Some(next) = input[i].direction.and_then(|d| d.next(p.pos)) {
                if self.can_enter(i, next)
                    && !self
                        .players
                        .iter()
                        .any(|other| other.alive && other.pos == next)
                {
                    targets[i] = Some(next);
                }
            }
        }
        for (i, target) in targets.iter().enumerate().take(self.players.len()) {
            if let Some(next) = target {
                if targets.iter().filter(|t| **t == Some(*next)).count() == 1 {
                    self.players[i].pos = *next;
                    self.players[i].ready = self.tick + MOVE;
                }
            }
        }
        for b in &mut self.bombs {
            for (i, p) in self.players.iter().enumerate() {
                if p.pos != b.pos {
                    b.pass &= !(1 << i);
                }
            }
        }
        // Newly placed bombs in active fire detonate in the same tick.
        if self.hazards() {
            events.push(Event::Explode);
        }
        self.eliminate();
        for p in &mut self.players {
            if p.alive && self.flames[p.pos] <= self.tick {
                if let Some(upgrade) = self.upgrades[p.pos].take() {
                    match upgrade {
                        Upgrade::Capacity => p.capacity = (p.capacity + 1).min(MAX_CAPACITY),
                        Upgrade::Range => p.range = (p.range + 1).min(MAX_RANGE),
                    }
                    events.push(Event::Pickup);
                }
            }
        }
        let alive: Vec<_> = self
            .players
            .iter()
            .enumerate()
            .filter(|(_, p)| p.alive)
            .map(|(i, _)| i)
            .collect();
        self.outcome = match alive.as_slice() {
            [] => Some(Outcome::Draw),
            [id] => Some(Outcome::Winner(*id)),
            _ => None,
        };
        if self.outcome.is_some() {
            events.push(Event::End);
        }
        events
    }
    /// Exact future hazard evolution with static occupants. Includes later crate removal,
    /// overlapping flames, chain reactions and the closing arena. No future player bombs.
    pub fn forecast(&self, horizon: u32) -> Vec<[bool; N]> {
        let mut copy = self.clone();
        let mut result = Vec::with_capacity(horizon as usize + 1);
        for _ in 0..=horizon {
            copy.hazards();
            result.push(std::array::from_fn(|p| {
                copy.flames[p] > copy.tick || copy.tiles[p] == Tile::Wall
            }));
            copy.tick += 1;
        }
        result
    }
    fn route(&self, id: usize, danger: &[[bool; N]], escape: bool) -> Option<Option<Direction>> {
        let start = self.players[id].pos;
        let horizon = danger.len() - 1;
        let mut seen = vec![[false; N]; horizon / MOVE as usize + 1];
        let mut queue = VecDeque::from([(start, 0usize, None)]);
        seen[0][start] = true;
        while let Some((pos, time, first)) = queue.pop_front() {
            let safe = (time..=horizon).all(|t| !danger[t][pos]);
            let useful = Direction::ALL
                .into_iter()
                .filter_map(|d| d.next(pos))
                .any(|p| self.tiles[p] == Tile::Crate)
                || self.players.iter().enumerate().any(|(j, p)| {
                    j != id
                        && p.alive
                        && Self::blast(&self.tiles, pos, self.players[id].range).contains(&p.pos)
                });
            if safe && (escape || useful || self.upgrades[pos].is_some()) {
                return Some(first);
            }
            let next_time = time + MOVE as usize;
            if next_time > horizon {
                continue;
            }
            for direction in Direction::ALL.into_iter().map(Some).chain([None]) {
                let next = match direction {
                    Some(d) => match d.next(pos) {
                        Some(p) => p,
                        None => continue,
                    },
                    None => pos,
                };
                if !self.can_enter(id, next)
                    || (next == start && pos != start && self.bombs.iter().any(|b| b.pos == start))
                {
                    continue;
                }
                if self
                    .players
                    .iter()
                    .enumerate()
                    .any(|(j, p)| j != id && p.alive && p.pos == next)
                {
                    continue;
                }
                if seen[next_time / MOVE as usize][next] {
                    continue;
                }
                // The command moves on the next tick, then stays until the next command.
                if (time + 1..=next_time).any(|t| danger[t][next]) {
                    continue;
                }
                seen[next_time / MOVE as usize][next] = true;
                queue.push_back((next, next_time, if time == 0 { direction } else { first }));
            }
        }
        None
    }
    pub fn bot_inputs(&self) -> [Input; 4] {
        let mut inputs = [Input::default(); 4];
        if !self.tick.is_multiple_of(MOVE) {
            return inputs;
        }
        let danger = self.forecast(FUSE + FLAME + MOVE);
        for (id, p) in self.players.iter().enumerate() {
            if !p.alive || !p.bot {
                continue;
            }
            let threatened = danger.iter().any(|d| d[p.pos]);
            if threatened {
                inputs[id].direction = self.route(id, &danger, true).flatten();
                continue;
            }
            let cells = Self::blast(&self.tiles, p.pos, p.range);
            let useful = cells.iter().any(|c| self.tiles[*c] == Tile::Crate)
                || self
                    .players
                    .iter()
                    .enumerate()
                    .any(|(j, other)| j != id && other.alive && cells.contains(&other.pos));
            if useful {
                let mut trial = self.clone();
                if trial.place(id) {
                    let future = trial.forecast(FUSE + FLAME + MOVE);
                    if let Some(direction) = trial.route(id, &future, true) {
                        inputs[id] = Input {
                            direction,
                            bomb: true,
                        };
                        continue;
                    }
                }
            }
            inputs[id].direction = self.route(id, &danger, false).flatten();
        }
        inputs
    }
}
#[derive(Clone, Debug)]
pub struct Match {
    pub arena: Arena,
    pub wins: [u8; 4],
    pub rounds: u32,
    pub champion: Option<usize>,
    humans: usize,
    bots: usize,
}
impl Match {
    pub fn new(arena: usize, humans: usize, bots: usize) -> Self {
        Self {
            arena: Arena::new(arena, humans, bots),
            wins: [0; 4],
            rounds: 1,
            champion: None,
            humans,
            bots,
        }
    }
    pub fn step(&mut self, input: [Input; 4]) -> Vec<Event> {
        let was_over = self.arena.outcome.is_some();
        let events = self.arena.step(input);
        if !was_over {
            if let Some(Outcome::Winner(id)) = self.arena.outcome {
                self.wins[id] += 1;
                if self.wins[id] == 3 {
                    self.champion = Some(id);
                }
            }
        }
        events
    }
    pub fn next_round(&mut self) {
        if self.arena.outcome.is_some() && self.champion.is_none() {
            self.rounds += 1;
            self.arena = Arena::new(self.arena.arena, self.humans, self.bots);
        }
    }
}
