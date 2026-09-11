//! Shared accountless community leaderboard transport. No UI or game dependency.
use serde::{Deserialize, Serialize};
use std::{
    sync::mpsc::{self, Receiver},
    time::Duration,
};
#[derive(Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    pub credential: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub ticket: String,
    pub seed: u64,
    pub expires: u64,
    pub rules: String,
    pub mode: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Submission {
    pub ticket: String,
    pub alias: String,
    pub replay: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    pub identity: String,
    pub alias: String,
    pub result: u64,
    pub submitted: u64,
    pub rank: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub rows: Vec<Row>,
    pub own: Option<Row>,
}
#[derive(Clone)]
pub struct Client {
    base: String,
    http: reqwest::blocking::Client,
}
impl Client {
    pub fn new(url: &str) -> Result<Self, String> {
        let parsed = reqwest::Url::parse(url).map_err(|_| "Invalid leaderboard URL")?;
        let local = matches!(parsed.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
        if parsed.scheme() != "https" && !(local && parsed.scheme() == "http") {
            return Err("Leaderboard requires HTTPS".into());
        }
        if !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err("Invalid leaderboard URL".into());
        }
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(3))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "Network client unavailable")?;
        Ok(Self {
            base: url.trim_end_matches('/').into(),
            http,
        })
    }
    pub fn call<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        credential: Option<&str>,
        body: Option<&serde_json::Value>,
    ) -> Result<T, String> {
        let method =
            reqwest::Method::from_bytes(method.as_bytes()).map_err(|_| "Invalid method")?;
        let mut request = self.http.request(method, format!("{}{path}", self.base));
        if let Some(c) = credential {
            request = request.bearer_auth(c);
        }
        if let Some(b) = body {
            request = request.json(b);
        }
        let response = request
            .send()
            .map_err(|_| "Connection failed. Your local game and result are safe.")?;
        let status = response.status();
        use std::io::Read;
        let mut bytes = vec![];
        response
            .take(131_073)
            .read_to_end(&mut bytes)
            .map_err(|_| "Incomplete server response")?;
        if bytes.len() > 131_072 {
            return Err("Server response too large".into());
        }
        if !status.is_success() {
            let detail = serde_json::from_slice::<serde_json::Value>(&bytes)
                .ok()
                .and_then(|v| v["error"].as_str().map(str::to_owned))
                .filter(|s| {
                    matches!(
                        s.as_str(),
                        "choose another alias"
                            | "ticket expired"
                            | "ticket already used"
                            | "replay rejected"
                            | "request limit"
                            | "daily run limit"
                            | "unsupported board"
                    )
                })
                .unwrap_or_else(|| "request failed".into());
            return Err(format!(
                "Leaderboard returned {}: {detail}. Your local result is safe.",
                status.as_u16()
            ));
        }
        serde_json::from_slice(&bytes).map_err(|_| "Invalid server response".into())
    }
}
pub fn background<T: Send + 'static>(
    work: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Receiver<Result<T, String>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(work());
    });
    rx
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_remote_plaintext_and_embedded_credentials() {
        assert!(Client::new("http://example.com").is_err());
        assert!(Client::new("https://user:secret@example.com").is_err());
        assert!(Client::new("http://127.0.0.1:8787").is_ok());
    }
    #[test]
    fn network_failure_is_a_worker_result() {
        let rx = background(|| {
            Client::new("http://127.0.0.1:1")?
                .call::<serde_json::Value>("GET", "/health", None, None)
        });
        assert!(rx.recv_timeout(Duration::from_secs(12)).unwrap().is_err());
    }
}
