use dirs::home_dir;
use std::path::PathBuf;

pub fn get_path_directory_config() -> Result<PathBuf, String> {
    home_dir()
        .map(|h| h.join(".config/shingari"))
        .ok_or_else(|| "failed to resolve Home directory".to_string())
}

pub fn get_str_file_error_log() -> Result<String, String> {
    get_path_directory_config()
        .map(|p| p.join("error.log"))
        .map(|p| format!("{}", p.display()))
}

pub fn get_str_file_pattern_error_log() -> Result<String, String> {
    get_path_directory_config().map(|p| format!("{}/error.{{}}.log", p.display()))
}

pub fn get_path_file_slack_webhook() -> Result<PathBuf, String> {
    get_path_directory_config().map(|p| p.join("slack_webhook.json"))
}

pub fn get_path_directory_targets() -> Result<PathBuf, String> {
    get_path_directory_config().map(|p| p.join("targets"))
}

pub fn get_path_file_target(pid: &i32) -> Result<PathBuf, String> {
    get_path_directory_targets().map(|p| p.join(format!("{pid}.json")))
}

pub fn get_path_directory_histories() -> Result<PathBuf, String> {
    get_path_directory_config().map(|p| p.join("histories"))
}

pub fn get_path_file_history(rtid: &str) -> Result<PathBuf, String> {
    get_path_directory_histories().map(|p| p.join(format!("{rtid}.json")))
}
