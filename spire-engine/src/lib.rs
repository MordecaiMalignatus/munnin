struct Job {
    pub name: String,
    pub source: Source,
    pub status: Status,
}

enum Status {
    NotStarted,
    Started,
    Running,
    Failed,
    Finished,
}

enum Source {
    Command { script: String },
    File{file: PathBuf},
}

/// Submit a job for running on the engine. Returns an identifier needed for
/// inspecting the process or retrieve logs after.
fn submit_job(job: Job) -> Result<JobRun> {
    todo!()
}

/// Retrieve logs for a job that has been submitted in the past.
fn retrieve_job_logs(job: &Job) -> Result<Logs> {
    todo!()
}

/// Installs a local socket for the daemon (this process) to listen on.
fn install_socket() -> Result<PathBuf> {
    todo!()
}

#[cfg(target_os = "macos")]
fn install_daemon() -> Result<()> {
    // Write plist to /Library/LaunchDaemons
    // https://www.launchd.info/
    todo!()
}
