use procfs::process::Process;
use procfs::ProcError;

pub fn get_process(pid: &i32) -> Result<Option<(String, String)>, String> {
    // cwd, cmdline
    let process = Process::new(*pid);
    if let Ok(process) = process {
        let cwd = process
            .cwd()
            .map_err(|e| format!("failed to resolve cwd of process: {e}"))?;
        let cmdline = process
            .cmdline()
            .map_err(|e| format!("failed to resolve cmdline of process: {e}"))?
            .join(" ");
        return Ok(Some((format!("{}", cwd.display()), cmdline)));
    }
    match process {
        Ok(process) => {
            let cwd = process
                .cwd()
                .map_err(|e| format!("failed to resolve cwd of process: {e}"))?;
            let cmdline = process
                .cmdline()
                .map_err(|e| format!("failed to resolve cmdline of process: {e}"))?
                .join(" ");
            Ok(Some((format!("{}", cwd.display()), cmdline)))
        }
        Err(err) => {
            if let ProcError::NotFound(_) = err {
                Ok(None)
            } else {
                Err(format!("failed to get process info: {err}"))
            }
        }
    }
}
