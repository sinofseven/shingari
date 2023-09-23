use glob::glob;

pub fn get_pid_files() -> Result<Vec<i32>, String> {
    let path = crate::path::get_path_directory_targets()?;
    let pattern = format!("{}/*.json", path.display());
    glob(&pattern)
        .map_err(|e| format!("failed to find monitoring target files: {e}"))
        .map(|paths| {
            paths
                .into_iter()
                .flatten()
                .flat_map(|path| {
                    path.file_name()
                        .ok_or(1)?
                        .to_str()
                        .ok_or(1)?
                        .replace(".json", "")
                        .parse()
                        .map_err(|_| 1)
                })
                .collect()
        })
}
