use std::path::PathBuf;

pub fn get_exe_path() -> PathBuf {
    return std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
}
