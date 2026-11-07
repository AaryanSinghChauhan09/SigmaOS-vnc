//! vnc model — VNC server/client.

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sessions: Vec<Session>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { sessions: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn connect(&mut self, name: &str, host: &str, port: u16) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.sessions.push(Session { id: id.clone(), name: name.into(), host: host.into(), port, status: "connecting".into() });
        id
    }

    pub fn disconnect(&mut self, id: &str) -> bool {
        if let Some(s) = self.sessions.iter_mut().find(|s| s.id == id) { s.status = "disconnected".into(); true } else { false }
    }

    pub fn list(&self) -> &[Session] { &self.sessions }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_connect_disconnect() {
        let mut v = Model::new();
        let id = v.connect("desktop", "host.example.com", 5900);
        assert!(v.disconnect(&id));
    }
}
