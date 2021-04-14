use std::path::PathBuf;

/**/
pub fn create_cross_platform_path(path: &PathBuf) -> String {
    if cfg!(windows) && !cfg!(target_os = "windows") {
        path.display().to_string().replace("\\", "/")
    } else if !cfg!(windows) && cfg!(target_os = "windows") {
        path.display().to_string().replace("/", "\\")
    }
    else {
        path.display().to_string()
    }
}