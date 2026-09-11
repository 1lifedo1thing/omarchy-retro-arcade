use pgn_reader::{BufferedReader, RawHeader, Skip, Visitor};
use serde::{Deserialize, Serialize};
use shakmaty::{
    fen::Fen,
    san::{San, SanPlus},
    uci::UciMove,
    CastlingMode, Chess, Color, EnPassantMode, Move, Position,
};
use std::collections::BTreeMap;

pub const MAX_PGN: usize = 1_000_000;
pub const MAX_PLIES: usize = 2048;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    Computer,
    Local,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Gentle,
    #[default]
    Casual,
    Club,
    Strong,
}
impl Difficulty {
    pub const ALL: [Self; 4] = [Self::Gentle, Self::Casual, Self::Club, Self::Strong];
    pub fn settings(self) -> (u8, u64) {
        match self {
            Self::Gentle => (0, 150),
            Self::Casual => (5, 300),
            Self::Club => (10, 600),
            Self::Strong => (20, 1000),
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Gentle => "Gentle",
            Self::Casual => "Casual",
            Self::Club => "Club",
            Self::Strong => "Strong",
        }
    }
}
#[derive(Clone, Debug)]
pub struct Game {
    pub positions: Vec<Chess>,
    pub moves: Vec<Move>,
    pub notation: Vec<String>,
    pub mode: Mode,
    pub human_white: bool,
    pub difficulty: Difficulty,
    pub result: String,
    pub ending: String,
    pub headers: BTreeMap<String, String>,
}
impl Default for Game {
    fn default() -> Self {
        Self::new(Chess::default())
    }
}
impl Game {
    pub fn new(initial: Chess) -> Self {
        Self {
            positions: vec![initial],
            moves: vec![],
            notation: vec![],
            mode: Mode::Computer,
            human_white: true,
            difficulty: Difficulty::Casual,
            result: "*".into(),
            ending: String::new(),
            headers: BTreeMap::new(),
        }
    }
    pub fn position(&self) -> &Chess {
        self.positions
            .last()
            .expect("game always has a starting position")
    }
    pub fn human_turn(&self) -> bool {
        !self.finished()
            && (self.mode == Mode::Local
                || (self.position().turn() == Color::White) == self.human_white)
    }
    pub fn fen(&self) -> String {
        fen(self.position())
    }
    pub fn automatic_result(&self) -> Option<(&'static str, &'static str)> {
        let p = self.position();
        if p.is_checkmate() {
            return Some((
                if p.turn() == Color::White {
                    "0-1"
                } else {
                    "1-0"
                },
                "Checkmate",
            ));
        }
        if p.is_stalemate() {
            return Some(("1/2-1/2", "Stalemate"));
        }
        if p.is_insufficient_material() {
            return Some(("1/2-1/2", "Insufficient material"));
        }
        if p.halfmoves() >= 150 {
            return Some(("1/2-1/2", "Seventy-five moves"));
        }
        if self.repetitions(p) >= 5 {
            return Some(("1/2-1/2", "Fivefold repetition"));
        }
        None
    }
    pub fn finished(&self) -> bool {
        self.result != "*" || self.automatic_result().is_some()
    }
    pub fn result_text(&self) -> &str {
        if self.result != "*" {
            &self.result
        } else {
            self.automatic_result().map_or("*", |x| x.0)
        }
    }
    pub fn status(&self) -> String {
        if self.result != "*" {
            return format!("{} · {}", self.ending, self.result);
        }
        if let Some((result, ending)) = self.automatic_result() {
            return format!("{ending} · {result}");
        }
        format!(
            "{} to move{}",
            if self.position().turn() == Color::White {
                "White"
            } else {
                "Black"
            },
            if self.position().is_check() {
                " · Check"
            } else {
                ""
            }
        )
    }
    fn repetitions(&self, p: &Chess) -> usize {
        let target = key(p);
        self.positions
            .iter()
            .filter(|other| key(other) == target)
            .count()
    }
    pub fn can_claim_draw(&self) -> bool {
        if self.finished() {
            return false;
        }
        if self.position().halfmoves() >= 100 || self.repetitions(self.position()) >= 3 {
            return true;
        }
        self.position().legal_moves().iter().any(|m| {
            let mut p = self.position().clone();
            p.play_unchecked(m);
            !p.is_checkmate() && (p.halfmoves() >= 100 || self.repetitions(&p) >= 2)
        })
    }
    pub fn claim_draw(&mut self) -> Result<(), String> {
        if !self.can_claim_draw() {
            return Err("No draw can be claimed here.".into());
        }
        self.result = "1/2-1/2".into();
        self.ending = "Draw claimed".into();
        Ok(())
    }
    pub fn resign(&mut self) {
        if self.finished() {
            return;
        }
        let white = if self.mode == Mode::Computer {
            self.human_white
        } else {
            self.position().turn() == Color::White
        };
        self.result = if white { "0-1" } else { "1-0" }.into();
        self.ending = format!("{} resigned", if white { "White" } else { "Black" });
    }
    pub fn play(&mut self, m: Move) -> Result<(), String> {
        if self.finished() || !self.position().is_legal(&m) {
            return Err("That move is not legal in this position.".into());
        }
        if self.moves.len() >= MAX_PLIES {
            return Err("This preview supports at most 2048 plies. Export this game before starting another.".into());
        }
        let p = self.position();
        let san = SanPlus::from_move(p.clone(), &m).to_string();
        let prefix = format!(
            "{}{}",
            p.fullmoves(),
            if p.turn() == Color::White { "." } else { "…" }
        );
        let mut next = p.clone();
        next.play_unchecked(&m);
        self.notation.push(format!("{prefix}  {san}"));
        self.moves.push(m);
        self.positions.push(next);
        Ok(())
    }
    pub fn parse_move(&self, text: &str) -> Result<Move, String> {
        let p = self.position();
        if let Ok(san) = San::from_ascii(text.trim().as_bytes()) {
            if let Ok(m) = san.to_move(p) {
                return Ok(m);
            }
        }
        text.trim()
            .parse::<UciMove>()
            .map_err(|_| "Move not recognised. Try e4, Nf3, O-O or e7e8n.".to_string())?
            .to_move(p)
            .map_err(|_| "That move is not legal.".into())
    }
    pub fn can_takeback(&self) -> bool {
        !self.moves.is_empty()
            && !(self.mode == Mode::Computer && !self.human_white && self.moves.len() == 1)
    }
    pub fn takeback(&mut self) {
        if !self.can_takeback() {
            return;
        }
        self.result = "*".into();
        self.ending.clear();
        self.pop();
        if self.mode == Mode::Computer
            && (self.position().turn() == Color::White) != self.human_white
            && !self.moves.is_empty()
        {
            self.pop();
        }
    }
    fn pop(&mut self) {
        self.moves.pop();
        self.positions.pop();
        self.notation.pop();
    }
    pub fn uci_position(&self) -> String {
        let moves = self
            .moves
            .iter()
            .map(|m| UciMove::from_move(m, CastlingMode::Standard).to_string())
            .collect::<Vec<_>>()
            .join(" ");
        format!(
            "position fen {}{}",
            fen(&self.positions[0]),
            if moves.is_empty() {
                String::new()
            } else {
                format!(" moves {moves}")
            }
        )
    }
    pub fn pgn(&self) -> String {
        let mut headers = self.headers.clone();
        headers
            .entry("Event".into())
            .or_insert_with(|| "Omarchy Chess".into());
        headers.entry("White".into()).or_insert_with(|| {
            if self.mode == Mode::Computer && !self.human_white {
                "Stockfish"
            } else {
                "White"
            }
            .into()
        });
        headers.entry("Black".into()).or_insert_with(|| {
            if self.mode == Mode::Computer && self.human_white {
                "Stockfish"
            } else {
                "Black"
            }
            .into()
        });
        headers.insert("Result".into(), self.result_text().into());
        if self.positions[0] != Chess::default() {
            headers.insert("FEN".into(), fen(&self.positions[0]));
            headers.insert("SetUp".into(), "1".into());
        } else {
            headers.remove("FEN");
            headers.remove("SetUp");
        }
        let mut output = String::new();
        for (k, v) in headers {
            output.push_str(&format!("[{} \"{}\"]\n", k, escape(&v)));
        }
        output.push('\n');
        for (i, m) in self.moves.iter().enumerate() {
            let p = &self.positions[i];
            if p.turn() == Color::White {
                output.push_str(&format!("{}. ", p.fullmoves()));
            } else if i == 0 {
                output.push_str(&format!("{}... ", p.fullmoves()));
            }
            output.push_str(&SanPlus::from_move(p.clone(), m).to_string());
            output.push(' ');
        }
        output.push_str(self.result_text());
        output.push('\n');
        output
    }
    pub fn from_pgn(text: &str) -> Result<Self, String> {
        if text.len() > MAX_PGN || text.trim().is_empty() {
            return Err("PGN must contain one game, at most 1 MB.".into());
        }
        let mut reader = BufferedReader::new_cursor(text.trim_start_matches('\u{feff}').as_bytes());
        let mut visitor = Import::default();
        let game = reader
            .read_game(&mut visitor)
            .map_err(|e| e.to_string())?
            .ok_or("PGN is empty.")??;
        if reader
            .read_game(&mut Import::default())
            .map_err(|e| e.to_string())?
            .is_some()
        {
            return Err("Import one game at a time.".into());
        }
        Ok(game)
    }
}
pub fn fen(p: &Chess) -> String {
    Fen::from_position(p.clone(), EnPassantMode::Legal).to_string()
}
fn key(p: &Chess) -> String {
    fen(p)
        .split_whitespace()
        .take(4)
        .collect::<Vec<_>>()
        .join(" ")
}
fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(['\n', '\r'], " ")
}

#[derive(Default)]
struct Import {
    game: Game,
    error: Option<String>,
    result: Option<String>,
    saw_data: bool,
}
impl Visitor for Import {
    type Result = Result<Game, String>;
    fn begin_game(&mut self) {
        *self = Self::default();
        self.game.mode = Mode::Local;
    }
    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        self.saw_data = true;
        let key = String::from_utf8_lossy(key).to_string();
        let value = value.decode_utf8_lossy().into_owned();
        if key == "Variant" && !["Standard", "Chess", "Normal"].contains(&value.as_str()) {
            self.error = Some("Only standard chess is supported.".into());
        }
        if key == "FEN" {
            match value
                .parse::<Fen>()
                .ok()
                .and_then(|f| f.into_position(CastlingMode::Standard).ok())
            {
                Some(p) => self.game.positions = vec![p],
                None => self.error = Some("Invalid PGN starting position.".into()),
            }
        }
        if key == "Result" {
            self.result = Some(value.clone());
        }
        self.game.headers.insert(key, value);
    }
    fn san(&mut self, san: SanPlus) {
        self.saw_data = true;
        if self.error.is_some() {
            return;
        }
        match san
            .san
            .to_move(self.game.position())
            .map_err(|e| e.to_string())
            .and_then(|m| self.game.play(m))
        {
            Ok(()) => (),
            Err(e) => self.error = Some(e),
        }
    }
    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }
    fn outcome(&mut self, outcome: Option<shakmaty::Outcome>) {
        let result = outcome.map_or("*", |o| match o {
            shakmaty::Outcome::Decisive {
                winner: Color::White,
            } => "1-0",
            shakmaty::Outcome::Decisive {
                winner: Color::Black,
            } => "0-1",
            shakmaty::Outcome::Draw => "1/2-1/2",
        });
        if let Some(header) = &self.result {
            if header != result {
                self.error = Some("PGN result header and movetext disagree.".into());
            }
        }
        self.result = Some(result.into());
        self.saw_data = true;
    }
    fn end_game(&mut self) -> Self::Result {
        if let Some(error) = self.error.take() {
            return Err(error);
        }
        if !self.saw_data {
            return Err("No chess game found.".into());
        }
        let result = self.result.take().unwrap_or_else(|| "*".into());
        if !["*", "1-0", "0-1", "1/2-1/2"].contains(&result.as_str()) {
            return Err("Invalid game result.".into());
        }
        if let Some((actual, _)) = self.game.automatic_result() {
            if result != "*" && result != actual {
                return Err("Result contradicts the final position.".into());
            }
        }
        self.game.result = result;
        if self.game.result != "*" {
            self.game.ending = "Imported result".into();
        }
        Ok(self.game.clone())
    }
}
