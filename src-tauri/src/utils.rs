use std::path::PathBuf;

pub fn get_download_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE")
            .ok()
            .map(|p| PathBuf::from(p).join("Downloads"))
    } else if cfg!(any(
        target_os = "linux",
        target_os = "openbsd",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "macos",
    )) {
        if let Ok(download_dir) = std::env::var("XDG_DOWNLOAD_DIR") {
            return Some(PathBuf::from(download_dir));
        }

        std::env::var("HOME")
            .ok()
            .map(|p| PathBuf::from(p).join("Downloads"))
    } else {
        None
    }
}
