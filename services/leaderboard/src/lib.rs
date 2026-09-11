use arcade_leaderboard::{Board, Identity, Row, Submission, Ticket};
use omarchy_stack::engine::{self, Mode, Replay};
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{io::Read, path::Path};
pub type ApiResult = Result<Value, (u16, &'static str)>;
pub fn hash(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes()))
}
fn random() -> String {
    let mut bytes = [0u8; 32];
    std::fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut bytes))
        .expect("OS random source required");
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub struct Service {
    pub db: Connection,
    pub blocked_words: Vec<String>,
}
impl Service {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let db = Connection::open(path)?;
        db.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON; PRAGMA busy_timeout=5000;
 CREATE TABLE IF NOT EXISTS identities(id TEXT PRIMARY KEY, credential TEXT UNIQUE NOT NULL, blocked INTEGER NOT NULL DEFAULT 0);
 CREATE TABLE IF NOT EXISTS tickets(id TEXT PRIMARY KEY, identity TEXT NOT NULL REFERENCES identities(id), seed TEXT NOT NULL, mode TEXT NOT NULL, rules TEXT NOT NULL, expires INTEGER NOT NULL, digest TEXT, response TEXT);
 CREATE INDEX IF NOT EXISTS tickets_identity ON tickets(identity,expires);
 CREATE TABLE IF NOT EXISTS scores(identity TEXT NOT NULL REFERENCES identities(id), mode TEXT NOT NULL, rules TEXT NOT NULL, alias TEXT NOT NULL, result INTEGER NOT NULL, submitted INTEGER NOT NULL, PRIMARY KEY(identity,mode,rules));")?;
        Ok(Self {
            db,
            blocked_words: vec![
                "admin".into(),
                "moderator".into(),
                "fuck".into(),
                "shit".into(),
            ],
        })
    }
    fn identity(&self, token: Option<&str>) -> Result<String, (u16, &'static str)> {
        let token = token.ok_or((401, "identity required"))?;
        if token.len() != 64 || !token.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err((401, "invalid identity"));
        }
        self.db
            .query_row(
                "SELECT id FROM identities WHERE credential=? AND blocked=0",
                [hash(token)],
                |r| r.get(0),
            )
            .optional()
            .map_err(db_error)?
            .ok_or((401, "invalid identity"))
    }
    pub fn handle(
        &mut self,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Value,
    ) -> ApiResult {
        self.db
            .execute(
                "DELETE FROM tickets WHERE expires < ?",
                [now().saturating_sub(7 * 86400) as i64],
            )
            .map_err(db_error)?;
        if method == "GET" && path == "/health" {
            return Ok(json!({"ok":true,"rules":[engine::RULES]}));
        }
        if method == "POST" && path == "/identity" {
            let credential = random();
            let id = random()[..32].to_owned();
            self.db
                .execute(
                    "INSERT INTO identities(id,credential) VALUES (?,?)",
                    params![id, hash(&credential)],
                )
                .map_err(db_error)?;
            return Ok(serde_json::to_value(Identity { id, credential }).unwrap());
        }
        if method == "GET" && path.starts_with("/boards/") {
            let parts: Vec<_> = path.split('/').collect();
            if parts.len() != 4 {
                return Err((404, "unknown board"));
            }
            let rules = parts[2];
            let mode = parts[3];
            if rules != engine::RULES || !matches!(mode, "Marathon" | "Sprint") {
                return Err((404, "unknown board"));
            }
            let id = if token.is_some() {
                Some(self.identity(token)?)
            } else {
                None
            };
            let order = if mode == "Sprint" { "ASC" } else { "DESC" };
            let query=format!("SELECT identity,alias,result,submitted,RANK() OVER (ORDER BY result {order}) AS rank FROM scores WHERE mode=? AND rules=? ORDER BY rank,submitted,identity");
            let mut stmt = self.db.prepare(&query).map_err(db_error)?;
            let all = stmt
                .query_map(params![mode, rules], |r| {
                    Ok(Row {
                        identity: r.get(0)?,
                        alias: r.get(1)?,
                        result: r.get::<_, i64>(2)? as u64,
                        submitted: r.get::<_, i64>(3)? as u64,
                        rank: r.get::<_, i64>(4)? as u64,
                    })
                })
                .map_err(db_error)?;
            let mut rows = vec![];
            let mut own = None;
            for row in all {
                let row = row.map_err(db_error)?;
                if id.as_deref() == Some(&row.identity) {
                    own = Some(row.clone());
                }
                if rows.len() < 100 {
                    rows.push(row);
                }
            }
            return Ok(serde_json::to_value(Board { rows, own }).unwrap());
        }
        let id = self.identity(token)?;
        if method == "DELETE" && path == "/scores" {
            let tx = self.db.transaction().map_err(db_error)?;
            tx.execute("DELETE FROM scores WHERE identity=?", [&id])
                .map_err(db_error)?;
            tx.execute("DELETE FROM tickets WHERE identity=?", [&id])
                .map_err(db_error)?;
            tx.commit().map_err(db_error)?;
            return Ok(json!({"deleted":true}));
        }
        if method == "POST" && path == "/tickets" {
            let mode = body["mode"].as_str().ok_or((400, "mode required"))?;
            let rules = body["rules"].as_str().ok_or((400, "rules required"))?;
            if !matches!(mode, "Marathon" | "Sprint") || rules != engine::RULES {
                return Err((400, "unsupported board"));
            }
            let count: u32 = self
                .db
                .query_row(
                    "SELECT count(*) FROM tickets WHERE identity=? AND expires>?",
                    params![id, now() as i64],
                    |r| r.get(0),
                )
                .map_err(db_error)?;
            if count >= 50 {
                return Err((429, "daily run limit"));
            }
            let seed = u64::from_str_radix(&random()[..16], 16).unwrap();
            let ticket = random();
            let expires = now() + 86400;
            self.db
                .execute(
                    "INSERT INTO tickets(id,identity,seed,mode,rules,expires) VALUES (?,?,?,?,?,?)",
                    params![
                        hash(&ticket),
                        id,
                        seed.to_string(),
                        mode,
                        rules,
                        expires as i64
                    ],
                )
                .map_err(db_error)?;
            return Ok(serde_json::to_value(Ticket {
                ticket,
                seed,
                expires,
                rules: rules.into(),
                mode: mode.into(),
            })
            .unwrap());
        }
        if method == "POST" && path == "/submissions" {
            let sub: Submission =
                serde_json::from_value(body).map_err(|_| (400, "invalid submission"))?;
            let alias = sub.alias.trim();
            if !(3..=24).contains(&alias.len())
                || !alias
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b" _-".contains(&c))
                || self
                    .blocked_words
                    .iter()
                    .any(|w| alias.to_ascii_lowercase().contains(w))
            {
                return Err((400, "choose another alias"));
            }
            if sub.ticket.len() != 64 {
                return Err((400, "invalid ticket"));
            }
            let t=self.db.query_row("SELECT seed,mode,rules,expires,digest,response FROM tickets WHERE id=? AND identity=?",params![hash(&sub.ticket),id],|r|Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,String>(2)?,r.get::<_,i64>(3)? as u64,r.get::<_,Option<String>>(4)?,r.get::<_,Option<String>>(5)?))).optional().map_err(db_error)?.ok_or((400,"invalid ticket"))?;
            let digest = hash(&serde_json::to_string(&sub).unwrap());
            if let Some(prior) = t.4 {
                return if prior == digest {
                    serde_json::from_str(&t.5.unwrap_or_default())
                        .map_err(|_| (500, "stored response unavailable"))
                } else {
                    Err((409, "ticket already used"))
                };
            }
            if now() > t.3 {
                return Err((410, "ticket expired"));
            }
            let replay: Replay =
                serde_json::from_value(sub.replay).map_err(|_| (400, "invalid replay"))?;
            if format!("{:?}", replay.mode) != t.1 || replay.rules != t.2 {
                return Err((400, "ticket board mismatch"));
            }
            let sim = engine::replay(
                t.0.parse().map_err(|_| (500, "stored seed invalid"))?,
                &replay,
            )
            .map_err(|_| (422, "replay rejected"))?;
            let result = if sim.mode == Mode::Sprint {
                sim.ticks
            } else {
                sim.score
            };
            let submitted = now();
            let response = json!({"accepted":true,"result":result,"rules":t.2,"mode":t.1});
            let tx = self.db.transaction().map_err(db_error)?;
            let previous: Option<i64> = tx
                .query_row(
                    "SELECT result FROM scores WHERE identity=? AND mode=? AND rules=?",
                    params![id, t.1, t.2],
                    |r| r.get(0),
                )
                .optional()
                .map_err(db_error)?;
            if previous.is_none_or(|v| {
                if sim.mode == Mode::Sprint {
                    result < (v as u64)
                } else {
                    result > (v as u64)
                }
            }) {
                tx.execute("INSERT OR REPLACE INTO scores(identity,mode,rules,alias,result,submitted) VALUES (?,?,?,?,?,?)",params![id,t.1,t.2,alias,result as i64,submitted as i64]).map_err(db_error)?;
            }
            tx.execute(
                "UPDATE tickets SET digest=?,response=? WHERE id=? AND digest IS NULL",
                params![digest, response.to_string(), hash(&sub.ticket)],
            )
            .map_err(db_error)?;
            tx.commit().map_err(db_error)?;
            return Ok(response);
        }
        Err((404, "unknown endpoint"))
    }
    pub fn moderate(&mut self, id: &str, block: bool) -> rusqlite::Result<()> {
        let tx = self.db.transaction()?;
        tx.execute("DELETE FROM scores WHERE identity=?", [id])?;
        tx.execute("DELETE FROM tickets WHERE identity=?", [id])?;
        if block {
            tx.execute("UPDATE identities SET blocked=1 WHERE id=?", [id])?;
        }
        tx.commit()
    }
}
fn db_error(_: rusqlite::Error) -> (u16, &'static str) {
    (500, "database unavailable")
}
