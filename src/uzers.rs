/// Get the current user's unix id
#[cfg(unix)]
pub(crate) fn get_uid() -> uzers::uid_t {
    uzers::get_current_uid()
}

/// Non-unix platforms will always return 0
#[cfg(not(unix))]
pub(crate) fn get_uid() -> uzers::uid_t {
    0
}