use arcade_leaderboard::{Board, Client, Identity, Submission, Ticket};
use omarchy_stack::engine::*;
use serde_json::{json, Value};
use std::{
    net::TcpListener,
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::Duration,
};
struct Server {
    process: Child,
    dir: PathBuf,
    url: String,
}
impl Server {
    fn start() -> Self {
        let port = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let dir =
            std::env::temp_dir().join(format!("arcade-service-test-{}-{port}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let process = Command::new(env!("CARGO_BIN_EXE_arcade-leaderboard-service"))
            .env("ARCADE_DATABASE", dir.join("db.sqlite3"))
            .env("ARCADE_BIND", format!("127.0.0.1:{port}"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let server = Self {
            process,
            dir,
            url: format!("http://127.0.0.1:{port}"),
        };
        let c = Client::new(&server.url).unwrap();
        for _ in 0..100 {
            if c.call::<Value>("GET", "/health", None, None).is_ok() {
                return server;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        panic!("server startup timeout")
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}
#[test]
fn real_http_replay_retry_size_limits_backup_and_network_failure() {
    let mut server = Server::start();
    let client = Client::new(&server.url).unwrap();
    let id: Identity = client.call("POST", "/identity", None, None).unwrap();
    let ticket: Ticket = client
        .call(
            "POST",
            "/tickets",
            Some(&id.credential),
            Some(&json!({"mode":"Marathon","rules":RULES})),
        )
        .unwrap();
    let mut sim = Sim::new(ticket.seed, Mode::Marathon, 10, 2);
    let mut events = vec![];
    while sim.outcome == Outcome::Playing {
        let input = if sim.ticks.is_multiple_of(2) { HARD } else { 0 };
        events.push(InputEvent {
            tick: sim.ticks,
            input,
        });
        sim.tick(input);
    }
    let replay = Replay {
        rules: RULES.into(),
        mode: sim.mode,
        das: sim.das,
        arr: sim.arr,
        ticks: sim.ticks,
        events,
        pauses: vec![],
        score: sim.score,
        lines: sim.lines,
    };
    let sub = serde_json::to_value(Submission {
        ticket: ticket.ticket,
        alias: "HTTP Player".into(),
        replay: serde_json::to_value(replay).unwrap(),
    })
    .unwrap();
    // Ignore the first response to model a connection disappearing after commit, then retry.
    let _: Value = client
        .call("POST", "/submissions", Some(&id.credential), Some(&sub))
        .unwrap();
    let retry: Value = client
        .call("POST", "/submissions", Some(&id.credential), Some(&sub))
        .unwrap();
    assert_eq!(retry["result"], sim.score);
    let board: Board = client
        .call(
            "GET",
            "/boards/stack-v1/Marathon",
            Some(&id.credential),
            None,
        )
        .unwrap();
    assert_eq!(board.rows.len(), 1);
    assert_eq!(board.own.unwrap().identity, id.id);
    let public = serde_json::to_string(&board.rows).unwrap();
    assert!(!public.contains(&id.credential));
    let error = client
        .call::<Value>(
            "POST",
            "/submissions",
            Some(&id.credential),
            Some(&json!({"padding":"x".repeat(2_100_000)})),
        )
        .unwrap_err();
    assert!(error.contains("413"));
    let backup = server.dir.join("backup.sqlite3");
    assert!(
        Command::new(env!("CARGO_BIN_EXE_arcade-leaderboard-service"))
            .env("ARCADE_DATABASE", server.dir.join("db.sqlite3"))
            .arg("backup")
            .arg(&backup)
            .status()
            .unwrap()
            .success()
    );
    let db = rusqlite::Connection::open(backup).unwrap();
    let count: i32 = db
        .query_row("SELECT COUNT(*) FROM scores", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
    let _: Value = client
        .call("DELETE", "/scores", Some(&id.credential), None)
        .unwrap();
    let board: Board = client
        .call("GET", "/boards/stack-v1/Marathon", None, None)
        .unwrap();
    assert!(board.rows.is_empty());
    let mut limited = false;
    for _ in 0..310 {
        if let Err(e) = client.call::<Value>("GET", "/health", None, None) {
            assert!(e.contains("429"));
            limited = true;
            break;
        }
    }
    assert!(limited);
    server.process.kill().unwrap();
    server.process.wait().unwrap();
    assert!(client
        .call::<Value>("POST", "/submissions", Some(&id.credential), Some(&sub))
        .is_err());
}
