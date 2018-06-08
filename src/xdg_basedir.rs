use std::env;
use std::path::PathBuf;

pub fn config_home() -> PathBuf {
    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
        PathBuf::from(config_home)
    } else {
        // fallback: $HOME/.config
        let home = env::var_os("HOME").expect("HOME environment variable");
        let mut path = PathBuf::from(home);
        path.push(".config");
        path
    }
}
