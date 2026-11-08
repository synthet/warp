use std::sync::Arc;

use crate::context_chips::context_chip::GeneratorContext;
use crate::terminal::model::block::BlockMetadata;
use crate::terminal::model::session::command_executor::testing::TestCommandExecutor;
use crate::terminal::model::session::{BootstrapSessionType, Session, SessionInfo};

#[test]
fn test_working_directory() {
    let session = Session::test();
    // SessionInfo forces the home directory in tests.
    let home_dir = session.home_dir().expect("Home dir is set in tests");

    let block_in_cwd = BlockMetadata::new(Some(session.id()), Some(format!("{home_dir}/projects")));

    assert_eq!(
        super::working_directory(&GeneratorContext {
            active_block_metadata: &block_in_cwd,
            active_session: Some(&session),
            current_environment: &Default::default(),
        })
        .as_ref()
        .and_then(|v| v.as_text()),
        Some("~/projects")
    );

    let block_outside_cwd = BlockMetadata::new(Some(session.id()), Some("/etc".to_string()));

    assert_eq!(
        super::working_directory(&GeneratorContext {
            active_block_metadata: &block_outside_cwd,
            active_session: Some(&session),
            current_environment: &Default::default(),
        })
        .as_ref()
        .and_then(|v| v.as_text()),
        Some("/etc")
    );
}

#[test]
fn test_remote_sessions() {
    let local_session = Session::test();
    let remote_session = Session::new(
        SessionInfo::new_for_test()
            .with_session_type(BootstrapSessionType::WarpifiedRemote)
            .with_hostname("remote-host".to_string())
            .with_user("remote-user".to_string()),
        Arc::new(TestCommandExecutor {}),
    );

    let local_ctx = GeneratorContext {
        active_block_metadata: &BlockMetadata::new(Some(local_session.id()), None),
        active_session: Some(&local_session),
        current_environment: &Default::default(),
    };

    let remote_ctx = GeneratorContext {
        active_block_metadata: &BlockMetadata::new(Some(remote_session.id()), None),
        active_session: Some(&remote_session),
        current_environment: &Default::default(),
    };

    // The Username and Hostname chips are always present.
    assert_eq!(
        super::username(&local_ctx)
            .as_ref()
            .and_then(|v| v.as_text()),
        Some("local:user")
    );
    assert_eq!(
        super::username(&remote_ctx)
            .as_ref()
            .and_then(|v| v.as_text()),
        Some("remote-user")
    );
    assert_eq!(
        super::hostname(&local_ctx)
            .as_ref()
            .and_then(|v| v.as_text()),
        Some("local:host")
    );
    assert_eq!(
        super::hostname(&remote_ctx)
            .as_ref()
            .and_then(|v| v.as_text()),
        Some("remote-host")
    );

    // The SSH chip is only shown for remote sessions.
    assert_eq!(super::ssh_session(&local_ctx), None);
    assert_eq!(
        super::ssh_session(&remote_ctx)
            .as_ref()
            .and_then(|v| v.as_text()),
        Some("remote-user@remote-host")
    );
}

#[test]
fn test_node_version() {
    use crate::context_chips::context_chip::Environment;
    use crate::terminal::model::block::BlockMetadata;
    use crate::terminal::model::session::Session;

    let session = Session::test();
    let block_metadata = BlockMetadata::new(Some(session.id()), None);

    // Test with no node version
    let environment_no_node = Environment::default();
    let ctx_no_node = GeneratorContext {
        active_block_metadata: &block_metadata,
        active_session: Some(&session),
        current_environment: &environment_no_node,
    };
    assert_eq!(super::node_version(&ctx_no_node), None);

    // Test with node version - create environment with node version
    let environment_with_node = Environment::new(
        None,                        // virtual_env
        None,                        // conda_env
        Some("v18.0.0".to_string()), // node_version
    );
    let ctx_with_node = GeneratorContext {
        active_block_metadata: &block_metadata,
        active_session: Some(&session),
        current_environment: &environment_with_node,
    };
    assert_eq!(
        super::node_version(&ctx_with_node)
            .as_ref()
            .and_then(|v| v.as_text()),
        Some("v18.0.0")
    );
}

/// Runs each generator's `cmd.exe` command the way a local PowerShell session on Windows does.
#[cfg(windows)]
mod cmd_exe {
    use std::path::Path;

    use command::blocking::Command;
    use warp_completer::completer::CommandOutput;

    use crate::terminal::model::session::{ExecuteCommandOptions, LocalCommandExecutor};
    use crate::terminal::shell::ShellType;

    fn git(repo: &Path, args: &[&str]) {
        let status = Command::new("git")
            .args(args)
            .current_dir(repo)
            .status()
            .expect("run git");
        assert!(status.success(), "git {args:?} failed");
    }

    fn init_repo(dir: &Path) {
        git(dir, &["init", "-q", "-b", "main"]);
        std::fs::write(dir.join("file.txt"), "one\n").expect("write file");
        git(dir, &["add", "file.txt"]);
        git(
            dir,
            &[
                "-c",
                "user.name=test",
                "-c",
                "user.email=test@example.com",
                "commit",
                "-q",
                "-m",
                "init",
            ],
        );
    }

    fn run(generator: super::super::ShellCommandGenerator, dir: &Path) -> CommandOutput {
        let command = generator
            .cmd_exe_command()
            .expect("generator should have a cmd.exe command")
            .to_owned();
        futures_lite::future::block_on(
            LocalCommandExecutor::new(None, ShellType::PowerShell).execute_local_command(
                &command,
                dir.to_str(),
                None,
                ExecuteCommandOptions {
                    run_command_in_same_shell_as_session: false,
                },
            ),
        )
        .expect("execute cmd.exe command")
    }

    fn stdout(output: &CommandOutput) -> String {
        output.to_string().expect("utf-8 stdout")
    }

    #[test]
    fn git_branch_prints_the_current_branch() {
        let repo = tempfile::tempdir().expect("create temp dir");
        init_repo(repo.path());

        let output = run(super::super::shell_git_branch(), repo.path());
        assert!(output.success(), "{output:?}");
        assert_eq!(stdout(&output).trim(), "main");
    }

    #[test]
    fn git_branch_fails_outside_a_repository() {
        let dir = tempfile::tempdir().expect("create temp dir");

        let output = run(super::super::shell_git_branch(), dir.path());
        assert!(!output.success(), "{output:?}");
        assert!(stdout(&output).trim().is_empty());
    }

    #[test]
    fn git_line_changes_reports_the_diff() {
        let repo = tempfile::tempdir().expect("create temp dir");
        init_repo(repo.path());
        std::fs::write(repo.path().join("file.txt"), "one\ntwo\n").expect("modify file");

        let output = run(super::super::shell_git_line_changes(), repo.path());
        assert!(output.success(), "{output:?}");
        assert_eq!(stdout(&output).trim(), "1 file changed, 1 insertion(+)");
    }

    #[test]
    fn other_git_branches_separates_branches_from_worktrees() {
        let repo = tempfile::tempdir().expect("create temp dir");
        init_repo(repo.path());

        let output = run(super::super::shell_other_git_branches(), repo.path());
        assert!(output.success(), "{output:?}");
        let stdout = stdout(&output);
        let lines: Vec<&str> = stdout.lines().map(str::trim).collect();
        let separator = lines
            .iter()
            .position(|line| *line == "\u{1e}")
            .unwrap_or_else(|| panic!("no record separator line in {stdout:?}"));
        assert_eq!(lines[..separator], ["* main"]);
        assert!(lines[separator + 1].starts_with("worktree "), "{stdout:?}");
    }
}
