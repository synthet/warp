use std::ffi::OsStr;
use std::os::windows::process::CommandExt as _;

use anyhow::{Context, Result};
use warp_errors::report_error;

#[derive(Debug, thiserror::Error)]
pub enum JobObjectError {
    #[error("Failed to create job: {0}")]
    CreateFailed(std::io::Error),

    #[error("Failed to assign process to job: {0}")]
    AssignFailed(std::io::Error),

    #[error("Failed to set info for job: {0}")]
    SetInfoFailed(std::io::Error),

    #[error("Failed to get info for job: {0}")]
    GetInfoFailed(std::io::Error),

    #[error(transparent)]
    Other(anyhow::Error),
}

impl From<win32job::JobError> for JobObjectError {
    fn from(error: win32job::JobError) -> Self {
        match error {
            win32job::JobError::CreateFailed(e) => JobObjectError::CreateFailed(e),
            win32job::JobError::AssignFailed(e) => JobObjectError::AssignFailed(e),
            win32job::JobError::SetInfoFailed(e) => JobObjectError::SetInfoFailed(e),
            win32job::JobError::GetInfoFailed(e) => JobObjectError::GetInfoFailed(e),
            _ => JobObjectError::Other(error.into()),
        }
    }
}

/// We use Job Objects to handle killing child processes when the program is
/// closed. This builder struct is used to configure a Job Object and associate it
/// with processes. Processes associated with a job will be killed when the handle
/// to the job is dropped at the end of the program's lifecycle.
///
/// NOTE: We've encountered issues with assigning some processes to jobs that
/// already contain other processes (i.e. `pwsh.exe`), so we only want to
/// assign a single process to a job.
///
/// For more information on Job Objects, see:
/// https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects
#[derive(Debug, Default)]
pub struct JobObject {
    assign_current_process: bool,
    assign_process: Option<isize>,
    kill_children_on_close: bool,
}

impl JobObject {
    pub fn new() -> Self {
        Self::default()
    }

    /// Assigns the current process to the Job Object. This can be used to ensure
    /// that children of the current process are associated with the job.
    pub fn assign_current_process(mut self) -> Self {
        self.assign_current_process = true;
        self
    }

    /// Assigns a process to the Job Object. This process will be killed when the
    /// current process is closed.
    pub fn assign_process(mut self, process: isize) -> Self {
        self.assign_process = Some(process);
        self
    }

    /// Configures the Job Object so children of the assigned processes are
    /// automatically associated with the job, thus killing them along with
    /// their parents on close.
    pub fn kill_children_on_close(mut self) -> Self {
        self.kill_children_on_close = true;
        self
    }

    fn create_internal(self) -> Result<(), win32job::JobError> {
        let job = win32job::Job::create()?;

        let mut info = job.query_extended_limit_info()?;
        // Mark the job as "kill on job close", so all processes associated with
        // the job are killed when the handle to the job is closed.
        info.limit_kill_on_job_close();
        info.limit_breakaway_ok();
        if !self.kill_children_on_close {
            info.limit_silent_breakaway_ok();
        }
        job.set_extended_limit_info(&info)?;

        if self.assign_current_process {
            job.assign_current_process()?;
        }
        if let Some(process) = self.assign_process {
            job.assign_process(process)?;
        }

        Box::leak(Box::new(job));
        Ok(())
    }

    /// Creates a new Job Object and assigns any specified processes to it. The
    /// handle to the job is leaked to ensure that the job lives for the lifetime
    /// of the program.
    pub fn create(self) -> Result<(), JobObjectError> {
        self.create_internal().map_err(Into::into)
    }
}

/// Whether `flags` asks `CreateProcess` to break the new process out of this
/// process's job object.
pub(crate) fn has_breakaway(flags: u32) -> bool {
    flags & windows::Win32::System::Threading::CREATE_BREAKAWAY_FROM_JOB.0 != 0
}

/// Whether a spawn failed because `CREATE_BREAKAWAY_FROM_JOB` was refused.
///
/// Warp puts itself in a job object that permits breakaway (see [`init`]), but a
/// nested job cannot widen the limits of the job it is nested in. Launched from a
/// more restrictive parent, `CreateProcess` rejects the flag outright and the spawn
/// fails with `ERROR_ACCESS_DENIED`, so no command runs at all. Callers retry once
/// without the flag: a child that exits with Warp is a much smaller problem than a
/// child that never starts.
pub(crate) fn is_breakaway_denied<T>(result: &std::io::Result<T>) -> bool {
    const ACCESS_DENIED: i32 = windows::Win32::Foundation::ERROR_ACCESS_DENIED.0 as i32;

    result.as_ref().err().and_then(std::io::Error::raw_os_error) == Some(ACCESS_DENIED)
}

/// Clears `CREATE_BREAKAWAY_FROM_JOB` from `flags`, warning the first time it happens.
pub(crate) fn without_breakaway(flags: u32) -> u32 {
    static WARNED: std::sync::Once = std::sync::Once::new();
    WARNED.call_once(|| {
        log::warn!(
            "CreateProcess denied CREATE_BREAKAWAY_FROM_JOB: Warp is inside a job object \
             that forbids breakaway. Retrying without it; spawned processes will exit \
             with Warp."
        );
    });

    flags & !windows::Win32::System::Threading::CREATE_BREAKAWAY_FROM_JOB.0
}

pub fn init() {
    if let Err(e) = JobObject::new()
        .kill_children_on_close()
        .assign_current_process()
        .create()
        .context("Failed to create job object for the program")
    {
        report_error!(e);
    }
}

pub trait CommandExt {
    /// Append literal text to the command line without any quoting or escaping.
    ///
    /// This is useful for passing arguments to `cmd.exe /c`, which doesn't follow
    /// `CommandLineToArgvW` escaping rules.
    fn raw_arg<S: AsRef<OsStr>>(&mut self, text_to_append_as_is: S) -> &mut Self;
}

use async_process::windows::CommandExt as _;

impl CommandExt for crate::blocking::Command {
    fn raw_arg<S: AsRef<OsStr>>(&mut self, text_to_append_as_is: S) -> &mut Self {
        self.inner.raw_arg(text_to_append_as_is);
        self
    }
}

impl CommandExt for crate::r#async::Command {
    fn raw_arg<S: AsRef<OsStr>>(&mut self, text_to_append_as_is: S) -> &mut Self {
        self.inner.raw_arg(text_to_append_as_is);
        self
    }
}
