//! Sshfs model — mount remote directory via SFTP.

#[derive(Debug, Clone)]
pub struct Mount {
    pub host: String,
    pub remote_path: String,
    pub local_path: String,
    pub user: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Sshfs {
    pub identity: Option<String>,
    pub port: u16,
    pub debug: bool,
}

impl Default for Sshfs {
    fn default() -> Self { Self { identity: None, port: 22, debug: false } }
}

impl Sshfs {
    pub fn new() -> Self { Self::default() }

    pub fn mount(&self, user_host: &str, remote: &str, local: &str) -> Mount {
        Mount {
            host: user_host.to_string(),
            remote_path: remote.to_string(),
            local_path: local.to_string(),
            user: user_host.split('@').next().unwrap_or("user").to_string(),
            options: vec!["reconnect".into(), "idmap=user".into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mount() {
        let s = Sshfs::new();
        let m = s.mount("user@host", "/remote", "/local");
        assert_eq!(m.local_path, "/local");
    }
}
