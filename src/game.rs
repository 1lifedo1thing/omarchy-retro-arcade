use serde::{Deserialize, Serialize};
pub const W: f32 = 800.;
pub const H: f32 = 700.;
// Sparse formation slots, deliberately independent from replaceable sprite pixels.
const LETTERS: [[&str; 5]; 7] = [
    ["###", "#.#", "#.#", "#.#", "###"],
    ["#.#", "###", "###", "#.#", "#.#"],
    [".#.", "#.#", "###", "#.#", "#.#"],
    ["##.", "#.#", "##.", "#.#", "#.#"],
    ["###", "#..", "#..", "#..", "###"],
    ["#.#", "#.#", "###", "#.#", "#.#"],
    ["#.#", "#.#", ".#.", ".#.", ".#."],
];
#[derive(Clone, Debug)]
pub struct Burst {
    pub p: Body,
    pub age: f32,
    pub player: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Body {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alien {
    pub p: Body,
    pub kind: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(default)]
    pub orbit: bool,
    #[serde(skip)]
    pub bursts: Vec<Burst>,
    #[serde(default)]
    pub march_frame: u64,
    pub ship: f32,
    pub aliens: Vec<Alien>,
    pub shots: Vec<Body>,
    pub bombs: Vec<Body>,
    pub shields: Vec<Body>,
    pub score: u32,
    pub wave: u32,
    pub lives: u8,
    pub over: bool,
    pub tick: u64,
    dir: f32,
    march: f32,
    fire: f32,
    cooldown: f32,
    pub grace: f32,
    rng: u64,
    pub ufo: Option<Body>,
    ufo_timer: f32,
    pub transition: f32,
}
impl Default for Game {
    fn default() -> Self {
        let mut g = Self {
            orbit: true,
            bursts: vec![],
            march_frame: 0,
            ship: W / 2.,
            aliens: vec![],
            shots: vec![],
            bombs: vec![],
            shields: vec![],
            score: 0,
            wave: 1,
            lives: 3,
            over: false,
            tick: 0,
            dir: 1.,
            march: 0.,
            fire: 0.,
            cooldown: 0.,
            grace: 1.,
            rng: 0x123abc,
            ufo: None,
            ufo_timer: 12.,
            transition: 0.,
        };
        g.populate();
        g
    }
}
impl Game {
    fn populate(&mut self) {
        self.orbit = true;
        self.bursts.clear();
        self.aliens.clear();
        for (letter, rows) in LETTERS.iter().enumerate() {
            for (row, pixels) in rows.iter().enumerate() {
                for (col, pixel) in pixels.bytes().enumerate() {
                    if pixel == b'#' {
                        self.aliens.push(Alien {
                            p: Body {
                                x: 88. + (letter * 4 + col) as f32 * 24.,
                                y: 160. + row as f32 * 28.,
                            },
                            kind: row,
                        });
                    }
                }
            }
        }
        self.shields.clear();
        for base in [140., 310., 480., 650.] {
            for y in 0..7 {
                for x in 0..11 {
                    if (y > 3 && (4..7).contains(&x)) || (y == 0 && !(2..9).contains(&x)) {
                        continue;
                    }
                    self.shields.push(Body {
                        x: base + (x as f32 - 5.) * 6.,
                        y: 560. + y as f32 * 6.,
                    });
                }
            }
        }
        self.shots.clear();
        self.bombs.clear();
        self.ufo = None;
        self.dir = 1.;
        self.march = 0.;
        self.fire = 0.;
        self.grace = 1.5;
    }
    fn random(&mut self) -> u64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        self.rng
    }
    pub fn valid(&self) -> bool {
        self.ship.is_finite()
            && (24. ..=776.).contains(&self.ship)
            && self.wave >= 1
            && self.wave <= 10000
            && self.lives <= 3
            && self.aliens.len() <= 120
            && self.shots.len() <= 10
            && self.bombs.len() <= 100
            && self.shields.len() <= 308
            && self.aliens.iter().all(|a| a.kind < 5 && finite(a.p))
            && self
                .shots
                .iter()
                .chain(&self.bombs)
                .chain(&self.shields)
                .all(|p| finite(*p))
            && [
                self.dir,
                self.march,
                self.fire,
                self.cooldown,
                self.grace,
                self.ufo_timer,
                self.transition,
            ]
            .iter()
            .all(|x| x.is_finite())
            && self.ufo.is_none_or(finite)
    }
    /// One fixed simulation step. Returns whether an impact occurred.
    pub fn step(&mut self, dt: f32, axis: f32, shoot: bool) -> bool {
        if self.over {
            return false;
        }
        for burst in &mut self.bursts {
            burst.age += dt;
        }
        self.bursts.retain(|burst| burst.age < 0.36);
        if self.transition > 0. {
            self.transition = (self.transition - dt).max(0.);
            return false;
        }
        self.tick += 1;
        self.ship = (self.ship + axis.clamp(-1., 1.) * 340. * dt).clamp(24., W - 24.);
        self.cooldown -= dt;
        self.grace = (self.grace - dt).max(0.);
        if shoot && self.cooldown <= 0. && self.shots.len() < 3 {
            self.shots.push(Body {
                x: self.ship,
                y: 640.,
            });
            self.cooldown = 0.24;
        }
        let mut hit = false;
        self.march += dt;
        let period = (0.055 + self.aliens.len() as f32 * if self.orbit { 0.0055 } else { 0.009 })
            / (1. + (self.wave - 1).min(15) as f32 * 0.08);
        if self.march >= period {
            self.march = 0.;
            self.march_frame = self.march_frame.wrapping_add(1);
            let edge = self
                .aliens
                .iter()
                .any(|a| a.p.x + self.dir * 12. < 28. || a.p.x + self.dir * 12. > W - 28.);
            if edge {
                self.dir = -self.dir;
            }
            for a in &mut self.aliens {
                if edge {
                    a.p.y += 18.;
                } else {
                    a.p.x += self.dir * 12.;
                }
            }
        }
        self.fire -= dt;
        if self.fire <= 0. && !self.aliens.is_empty() {
            let i = self.random() as usize % self.aliens.len();
            let x = self.aliens[i].p.x;
            let y = self
                .aliens
                .iter()
                .filter(|a| (a.p.x - x).abs() < 1.)
                .map(|a| a.p.y)
                .fold(0., f32::max);
            self.bombs.push(Body { x, y: y + 16. });
            self.fire = (0.85 - self.wave.min(12) as f32 * 0.04).max(0.25);
        }
        self.ufo_timer -= dt;
        if self.ufo_timer <= 0. {
            self.ufo = Some(Body { x: -30., y: 52. });
            self.ufo_timer = 18.;
        }
        if let Some(u) = &mut self.ufo {
            u.x += 100. * dt;
            if u.x > W + 30. {
                self.ufo = None;
            }
        }
        for s in &mut self.shots {
            s.y -= 550. * dt;
        }
        for b in &mut self.bombs {
            b.y += (180. + self.wave.min(15) as f32 * 12.) * dt;
        }
        let half = if self.orbit { 10. } else { 18. };
        let mut shots = std::mem::take(&mut self.shots);
        shots.retain(|s| {
            if s.y < 0. {
                return false;
            }
            if erode(&mut self.shields, *s) {
                return false;
            }
            if let Some(i) = self.aliens.iter().position(|a| {
                (a.p.x - s.x).abs() < half
                    && (a.p.y - s.y).abs() < if self.orbit { 11. } else { 15. }
            }) {
                let a = self.aliens.remove(i);
                self.bursts.push(Burst {
                    p: a.p,
                    age: 0.,
                    player: false,
                });
                self.score = self.score.saturating_add(if a.kind == 0 {
                    30
                } else if a.kind < 3 {
                    20
                } else {
                    10
                });
                hit = true;
                return false;
            }
            if self
                .ufo
                .is_some_and(|u| (u.x - s.x).abs() < 27. && (u.y - s.y).abs() < 12.)
            {
                self.bursts.push(Burst {
                    p: self.ufo.unwrap(),
                    age: 0.,
                    player: false,
                });
                self.ufo = None;
                self.score = self.score.saturating_add(150);
                hit = true;
                return false;
            }
            true
        });
        self.shots = shots;
        let mut bombs = std::mem::take(&mut self.bombs);
        let mut damaged = false;
        bombs.retain(|b| {
            if b.y > H {
                return false;
            }
            if erode(&mut self.shields, *b) {
                return false;
            }
            if (b.x - self.ship).abs() < 21. && (b.y - 652.).abs() < 15. {
                if self.grace <= 0. {
                    damaged = true;
                }
                return false;
            }
            true
        });
        self.bombs = bombs;
        if damaged {
            self.bursts.push(Burst {
                p: Body {
                    x: self.ship,
                    y: 652.,
                },
                age: 0.,
                player: true,
            });
            self.lives = self.lives.saturating_sub(1);
            self.grace = 2.;
            self.bombs.clear();
            hit = true;
        }
        self.shields.retain(|s| {
            !self
                .aliens
                .iter()
                .any(|a| (a.p.x - s.x).abs() < 20. && (a.p.y - s.y).abs() < 17.)
        });
        if self.lives == 0 || self.aliens.iter().any(|a| a.p.y >= 630.) {
            self.over = true;
        }
        if self.aliens.is_empty() && !self.over {
            self.wave = (self.wave + 1).min(10000);
            self.populate();
            self.transition = 2.;
        }
        hit
    }
}
fn finite(p: Body) -> bool {
    p.x.is_finite() && p.y.is_finite() && p.x.abs() < 2000. && p.y.abs() < 2000.
}
fn erode(shields: &mut Vec<Body>, p: Body) -> bool {
    if let Some(s) = shields
        .iter()
        .find(|s| (s.x - p.x).abs() < 5. && (s.y - p.y).abs() < 7.)
        .copied()
    {
        shields.retain(|b| (b.x - s.x).powi(2) + (b.y - s.y).powi(2) > 90.);
        true
    } else {
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initial_state() {
        let g = Game::default();
        assert!(g.aliens.len() > 55 && g.aliens.len() <= 120);
        assert!(g.valid());
    }
    #[test]
    fn formation_slots_do_not_overlap_and_march_together() {
        let mut g = Game::default();
        for (i, a) in g.aliens.iter().enumerate() {
            for b in g.aliens.iter().skip(i + 1) {
                assert!((a.p.x - b.p.x).abs() >= 24. || (a.p.y - b.p.y).abs() >= 28.);
            }
        }
        let original = g.aliens.clone();
        for _ in 0..120 {
            g.step(1. / 120., 0., false);
        }
        let dx = g.aliens[0].p.x - original[0].p.x;
        assert!(dx > 0.);
        assert!(g
            .aliens
            .iter()
            .zip(&original)
            .all(|(a, b)| a.p.x - b.p.x == dx));
    }
    #[test]
    fn legacy_save_keeps_original_hit_geometry() {
        let mut value = serde_json::to_value(Game::default()).unwrap();
        value.as_object_mut().unwrap().remove("orbit");
        value.as_object_mut().unwrap().remove("march_frame");
        let old: Game = serde_json::from_value(value).unwrap();
        assert!(!old.orbit);
        assert!(old.valid());
        assert!(old.bursts.is_empty());
    }
    #[test]
    fn boundaries() {
        let mut g = Game::default();
        for _ in 0..500 {
            g.step(1. / 120., -1., false);
        }
        assert_eq!(g.ship, 24.);
    }
    #[test]
    fn shields_erode() {
        let mut g = Game::default();
        let n = g.shields.len();
        assert!(erode(&mut g.shields, Body { x: 140., y: 560. }));
        assert!(g.shields.len() < n);
    }
    #[test]
    fn kill_scores_once() {
        let mut g = Game::default();
        let n = g.aliens.len();
        let p = g.aliens[0].p;
        g.shots.push(Body {
            x: p.x,
            y: p.y + 3.,
        });
        g.step(1. / 120., 0., false);
        assert_eq!(g.score, 30);
        assert_eq!(g.aliens.len(), n - 1);
        assert_eq!(g.bursts.len(), 1);
        assert!(g.shots.is_empty());
    }
    #[test]
    fn wave_progresses() {
        let mut g = Game::default();
        g.aliens.clear();
        g.step(1. / 120., 0., false);
        assert_eq!(g.wave, 2);
        assert!(g.aliens.len() > 55 && g.aliens.len() <= 120);
        assert!(g.transition > 0.);
    }
    #[test]
    fn damage_grace() {
        let mut g = Game {
            grace: 0.,
            bombs: vec![Body { x: 400., y: 650. }; 3],
            ..Default::default()
        };
        g.step(1. / 120., 0., false);
        assert_eq!(g.lives, 2);
        g.bombs.push(Body { x: 400., y: 650. });
        g.step(1. / 120., 0., false);
        assert_eq!(g.lives, 2);
    }
    #[test]
    fn game_over_stops() {
        let mut g = Game {
            over: true,
            ..Default::default()
        };
        let before = serde_json::to_string(&g).unwrap();
        g.step(1. / 120., 1., true);
        assert_eq!(before, serde_json::to_string(&g).unwrap());
    }
    #[test]
    fn long_run_is_bounded() {
        let mut g = Game::default();
        for i in 0..100000 {
            g.step(1. / 120., if i % 700 < 350 { 1. } else { -1. }, true);
            assert!(g.valid());
            if g.over {
                g = Game::default();
            }
        }
    }
    #[test]
    fn save_roundtrip() {
        let g = Game::default();
        let s = serde_json::to_string(&g).unwrap();
        let g: Game = serde_json::from_str(&s).unwrap();
        assert!(g.valid());
    }
}
