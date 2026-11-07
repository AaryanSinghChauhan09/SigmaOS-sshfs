//! sshfs model — filesystem over SSH.

#[derive(Debug, Clone)]
pub struct Mount {
    pub id: String,
    pub remote: String,
    pub local: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub mounts: Vec<Mount>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { mounts: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn mount(&mut self, remote: &str, local: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.mounts.push(Mount { id: id.clone(), remote: remote.into(), local: local.into(), status: "mounted".into() });
        id
    }

    pub fn unmount(&mut self, id: &str) -> bool {
        if let Some(m) = self.mounts.iter_mut().find(|m| m.id == id) {
            m.status = "unmounted".into();
            true
        } else { false }
    }

    pub fn list(&self) -> &[Mount] { &self.mounts }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mount_unmount() {
        let mut s = Model::new();
        let id = s.mount("user@server:/remote", "/mnt/sshfs");
        assert!(s.unmount(&id));
    }
}
