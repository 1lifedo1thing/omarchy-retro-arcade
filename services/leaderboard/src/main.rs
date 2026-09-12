use arcade_leaderboard_service::{hash, now, Service};
use std::{collections::HashMap, path::Path};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = std::env::var("ARCADE_DATABASE").unwrap_or("arcade.sqlite3".into());
    let mut service = Service::open(Path::new(&db))?;
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("backup") {
        let target = args.get(2).ok_or("backup requires destination")?;
        service.db.backup("main", Path::new(target), None)?;
        return Ok(());
    }
    if matches!(args.get(1).map(String::as_str), Some("remove" | "block")) {
        service.moderate(args.get(2).ok_or("identity required")?, args[1] == "block")?;
        return Ok(());
    }
    if let Ok(file) = std::env::var("ARCADE_ALIAS_BLOCKLIST") {
        for word in std::fs::read_to_string(file)?.lines() {
            if !word.trim().is_empty() {
                service.blocked_words.push(word.trim().to_ascii_lowercase());
            }
        }
    }
    let bind = std::env::var("ARCADE_BIND").unwrap_or("127.0.0.1:8787".into());
    let server = tiny_http::Server::http(&bind).map_err(|_| "Cannot bind service")?;
    let mut limits: HashMap<String, (u64, u32)> = HashMap::new();
    eprintln!("Arcade leaderboard listening; request logging disabled");
    for mut request in server.incoming_requests() {
        let time = now();
        limits.retain(|_, v| time.saturating_sub(v.0) < 60);
        // Behind the supplied Caddy proxy this is one shared bucket, deliberately ignoring spoofable headers.
        let remote = request
            .remote_addr()
            .map(|a| a.ip().to_string())
            .unwrap_or_default();
        let key = hash(&remote);
        let count = limits.entry(key).or_insert((time, 0));
        count.1 += 1;
        let mut status = 200;
        let result = if count.1 > 300 || limits.len() > 10000 {
            Err((429, "request limit"))
        } else if request.body_length().unwrap_or(0) > 2_097_152 {
            Err((413, "request too large"))
        } else {
            use std::io::Read;
            let mut bytes = vec![];
            match request.as_reader().take(2_097_153).read_to_end(&mut bytes) {
                Err(_) => Err((400, "unreadable request")),
                Ok(_) if bytes.len() > 2_097_152 => Err((413, "request too large")),
                Ok(_) => {
                    let body = if bytes.is_empty() {
                        Ok(serde_json::Value::Null)
                    } else {
                        serde_json::from_slice(&bytes)
                    };
                    match body {
                        Err(_) => Err((400, "invalid JSON")),
                        Ok(body) => {
                            let token = request
                                .headers()
                                .iter()
                                .find(|h| h.field.equiv("Authorization"))
                                .and_then(|h| h.value.as_str().strip_prefix("Bearer "))
                                .map(str::to_owned);
                            service.handle(
                                request.method().as_str(),
                                request.url(),
                                token.as_deref(),
                                body,
                            )
                        }
                    }
                }
            }
        };
        let body = match result {
            Ok(v) => v,
            Err((code, message)) => {
                status = code;
                serde_json::json!({"error":message})
            }
        };
        let response = tiny_http::Response::from_string(body.to_string())
            .with_status_code(status)
            .with_header(tiny_http::Header::from_bytes("Content-Type", "application/json").unwrap())
            .with_header(tiny_http::Header::from_bytes("Cache-Control", "no-store").unwrap());
        let _ = request.respond(response);
    }
    Ok(())
}
