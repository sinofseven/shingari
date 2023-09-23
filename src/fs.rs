use std::path::PathBuf;

pub fn load_text(path: &PathBuf) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read file: path={}, err={}", path.display(), e))
}

pub fn save_text(path: &PathBuf, text: &str) -> Result<(), String> {
    let dir = path.parent().ok_or(format!(
        "failed to resolve parent dir of file: path={}",
        path.display()
    ))?;
    if !dir.exists() {
        std::fs::create_dir_all(dir).map_err(|e| {
            format!(
                "failed to create parent directory of path: path={}, err={}",
                path.display(),
                e
            )
        })?;
    }
    std::fs::write(path, text)
        .map_err(|e| format!("failed to write file: path={}, err={}", path.display(), e))
}

pub fn delete_file(path: &PathBuf) -> Result<(), String> {
    std::fs::remove_file(path)
        .map_err(|e| format!("failed to delete file: path={}, err={}", path.display(), e))
}

pub fn copy_file(source: &PathBuf, destination: &PathBuf) -> Result<(), String> {
    std::fs::copy(source, destination).map(|_| ()).map_err(|e| {
        format!(
            "failed to copy file: from={}, to={}, err={}",
            source.display(),
            destination.display(),
            e
        )
    })
}
