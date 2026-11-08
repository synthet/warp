// These tests only build for Windows, where `std::time::Instant` works.
#![allow(clippy::disallowed_types)]

use std::time::{Duration, Instant};

use super::*;
use crate::blocking::Command;

const TIMEOUT: Duration = Duration::from_secs(20);

/// Waits until `done` accepts the number of live processes in `job` and at least
/// `min_wait` has passed, returning that number.
fn wait_for_active_processes(
    job: &ProcessTreeJob,
    min_wait: Duration,
    done: impl Fn(u32) -> bool,
) -> u32 {
    let start = Instant::now();
    loop {
        let count = job.active_process_count().expect("query job");
        if done(count) && start.elapsed() >= min_wait {
            return count;
        }
        assert!(
            start.elapsed() < TIMEOUT,
            "timed out with {count} live processes in the job"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// `cmd.exe` that starts a short `ping`, then a long one. The long one starts
/// about two seconds in, well after the job assignment, so it is certain to be
/// in the job.
fn spawn_process_tree() -> std::process::Child {
    Command::new("cmd.exe")
        .args([
            "/D",
            "/C",
            "ping -n 3 127.0.0.1 >nul & ping -n 60 127.0.0.1 >nul",
        ])
        .spawn()
        .expect("spawn cmd.exe")
}

#[test]
fn terminate_kills_processes_started_by_the_assigned_process() {
    let mut child = spawn_process_tree();
    let job = ProcessTreeJob::assign(child.id()).expect("assign child to job");

    // cmd.exe and the long ping, plus the console host Windows starts for cmd.exe.
    wait_for_active_processes(&job, Duration::from_millis(2500), |count| count >= 2);
    job.terminate().expect("terminate job");

    wait_for_active_processes(&job, Duration::ZERO, |count| count == 0);
    let status = child.wait().expect("wait for child");
    assert_eq!(status.code(), Some(1));
}

#[test]
fn dropping_the_job_leaves_its_processes_running() {
    let mut child = Command::new("cmd.exe")
        .args(["/D", "/C", "ping -n 60 127.0.0.1 >nul"])
        .spawn()
        .expect("spawn cmd.exe");
    // The outer job only cleans up after the test; the inner one is nested in it.
    let cleanup = ProcessTreeJob::assign(child.id()).expect("assign child to cleanup job");
    let job = ProcessTreeJob::assign(child.id()).expect("assign child to job");
    drop(job);

    std::thread::sleep(Duration::from_millis(500));
    let still_running = child.try_wait().expect("poll child").is_none();
    cleanup.terminate().expect("terminate cleanup job");
    let _ = child.wait();
    assert!(still_running, "closing the job should not kill its members");
}
