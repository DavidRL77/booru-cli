use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    dirs::config_local_dir().unwrap().join("booru-cli")
}

pub fn credentials_path() -> PathBuf {
    config_dir().join("credentials.toml")
}

pub fn temp_dir() -> PathBuf {
    std::env::temp_dir().join("booru-cli")
}
