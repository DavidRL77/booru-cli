use std::path::PathBuf;
use crate::uzers;

/// Application specific local configuration directory
pub fn config_dir() -> PathBuf {
    dirs::config_local_dir().unwrap().join("booru-cli")
}

pub fn credentials_path() -> PathBuf {
    config_dir().join("credentials.toml")
}

/// Application specific temporary path, including a user-specific discriminator (only on unix)
/// 
/// On unix-based systems, the user's id will be appended to the directory's name
/// On non-unix systems, it will be appended with a 0
pub fn temp_dir() -> PathBuf {
    std::env::temp_dir().join(format!("booru-cli-{}", uzers::get_uid()))
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(unix)]
    fn temp_dir_contains_uid() {
        let dir = super::temp_dir();
        assert!(dir.display().to_string().contains(&uzers::get_current_uid().to_string()))
    }
}