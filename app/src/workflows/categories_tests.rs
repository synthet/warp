use std::sync::Arc;

use crate::workflows::WorkflowType;
use crate::workflows::categories::{
    CategoriesView, WorkflowMatchType, WorkflowViewType, sidebar_workflow_types,
};
use crate::workflows::workflow::Workflow;

#[test]
fn test_workflow_matches() {
    let workflow = Arc::new(WorkflowType::Local(Workflow::Command {
        name: "g workflow_name it ".into(),
        command: "command_name git".to_string(),
        tags: vec!["foo".into(), "bar".into()],
        description: None,
        arguments: vec![],
        source_url: None,
        author: None,
        author_url: None,
        shells: vec![],
        environment_variables: None,
    }));

    assert_eq!(
        CategoriesView::matches_workflow(&workflow, "foo"),
        WorkflowMatchType::Tag
    );
    assert_eq!(
        CategoriesView::matches_workflow(&workflow, "bar"),
        WorkflowMatchType::Tag
    );

    // The Workflow name has higher precedence than the command.
    assert!(matches!(
        CategoriesView::matches_workflow(&workflow, "name"),
        WorkflowMatchType::Name { .. }
    ));

    // Git matches both the name and the command, but fuzzy matches command with a higher score.
    assert!(matches!(
        CategoriesView::matches_workflow(&workflow, "git"),
        WorkflowMatchType::Command { .. }
    ));

    assert!(matches!(
        CategoriesView::matches_workflow(&workflow, "command"),
        WorkflowMatchType::Command { .. }
    ));

    assert!(matches!(
        CategoriesView::matches_workflow(&workflow, "command"),
        WorkflowMatchType::Command { .. }
    ));

    assert_eq!(
        CategoriesView::matches_workflow(&workflow, "gibberish"),
        WorkflowMatchType::Unmatched
    );
}

/// Without a cloud backend (the OSS build) the sidebar lists no Team Workflows entry, while the
/// bundled catalog ("All"), My Workflows and Repository Workflows remain.
#[test]
fn sidebar_omits_team_workflows_without_cloud() {
    assert_eq!(
        sidebar_workflow_types(false),
        vec![
            WorkflowViewType::All,
            WorkflowViewType::LocalPersonal,
            WorkflowViewType::Project,
        ]
    );
}

/// With a cloud backend configured, Team Workflows sits between My and Repository Workflows.
#[test]
fn sidebar_lists_team_workflows_with_cloud() {
    assert_eq!(
        sidebar_workflow_types(true),
        vec![
            WorkflowViewType::All,
            WorkflowViewType::LocalPersonal,
            WorkflowViewType::Team,
            WorkflowViewType::Project,
        ]
    );
}
