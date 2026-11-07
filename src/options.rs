//! Options model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Default,
    Unmount,
    Debug,
    Reconnect,
    Idmap(String),
}

#[derive(Debug, Clone)]
pub struct SshfsOptions {
    pub mode: Mode,
    pub mount_point: String,
}

impl Default for SshfsOptions {
    fn default() -> Self { Self { mode: Mode::Default, mount_point: String::new() } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(SshfsOptions::default().mode, Mode::Default); }
}
