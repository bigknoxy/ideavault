use ideavault::commands::project::UpdateProjectArgs;
use ideavault::commands::ProjectCommands;
use ideavault::models::project::ProjectStatus;
use ideavault::models::Project;
use ideavault::storage::Storage;
use uuid::Uuid;

#[test]
fn project_url_and_repo_roundtrip() {
    let project = Project::new("Test Project".to_string())
        .with_url("https://example.com".to_string())
        .with_repo("https://github.com/user/repo".to_string());

    assert_eq!(project.url, Some("https://example.com".to_string()));
    assert_eq!(
        project.repo,
        Some("https://github.com/user/repo".to_string())
    );
}

#[test]
fn project_new_without_url_repo() {
    let project = Project::new("Test".to_string());

    assert_eq!(project.url, None);
    assert_eq!(project.repo, None);
}

#[test]
fn project_set_url() {
    let mut project = Project::new("Test".to_string());
    project.set_url(Some("https://example.com".to_string()));

    assert_eq!(project.url, Some("https://example.com".to_string()));
}

#[test]
fn project_set_repo() {
    let mut project = Project::new("Test".to_string());
    project.set_repo(Some("https://github.com/user/repo".to_string()));

    assert_eq!(
        project.repo,
        Some("https://github.com/user/repo".to_string())
    );
}

#[test]
fn project_clear_url() {
    let mut project = Project::new("Test".to_string()).with_url("https://example.com".to_string());

    project.set_url(None);

    assert_eq!(project.url, None);
}

#[test]
fn project_clear_repo() {
    let mut project =
        Project::new("Test".to_string()).with_repo("https://github.com/user/repo".to_string());

    project.set_repo(None);

    assert_eq!(project.repo, None);
}

#[test]
fn project_update_title() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Original Title".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: Some("New Title".to_string()),
        description: None,
        milestone: None,
        url: None,
        repo: None,
        status: None,
        clear: vec![],
    };

    ProjectCommands::update_project(&storage, &args).unwrap();

    let projects = storage.load_projects().unwrap();
    let updated = projects.iter().find(|p| p.id == id).unwrap();
    assert_eq!(updated.title, "New Title");
}

#[test]
fn project_update_url() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Test".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: None,
        description: None,
        milestone: None,
        url: Some("https://example.com".to_string()),
        repo: None,
        status: None,
        clear: vec![],
    };

    ProjectCommands::update_project(&storage, &args).unwrap();

    let projects = storage.load_projects().unwrap();
    let updated = projects.iter().find(|p| p.id == id).unwrap();
    assert_eq!(updated.url, Some("https://example.com".to_string()));
}

#[test]
fn project_update_multiple_fields() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Original".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: Some("New Title".to_string()),
        description: Some("New description".to_string()),
        milestone: Some("v1.0".to_string()),
        url: Some("https://example.com".to_string()),
        repo: Some("https://github.com/user/repo".to_string()),
        status: None,
        clear: vec![],
    };

    ProjectCommands::update_project(&storage, &args).unwrap();

    let projects = storage.load_projects().unwrap();
    let updated = projects.iter().find(|p| p.id == id).unwrap();
    assert_eq!(updated.title, "New Title");
    assert_eq!(updated.description, Some("New description".to_string()));
    assert_eq!(updated.milestone, Some("v1.0".to_string()));
    assert_eq!(updated.url, Some("https://example.com".to_string()));
    assert_eq!(
        updated.repo,
        Some("https://github.com/user/repo".to_string())
    );
}

#[test]
fn project_update_clear_url() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Test".to_string()).with_url("https://example.com".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: None,
        description: None,
        milestone: None,
        url: None,
        repo: None,
        status: None,
        clear: vec!["url".to_string()],
    };

    ProjectCommands::update_project(&storage, &args).unwrap();

    let projects = storage.load_projects().unwrap();
    let updated = projects.iter().find(|p| p.id == id).unwrap();
    assert_eq!(updated.url, None);
}

#[test]
fn project_update_clear_multiple() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Test".to_string())
        .with_url("https://example.com".to_string())
        .with_repo("https://github.com/user/repo".to_string())
        .with_milestone("v1.0".to_string())
        .with_description("Description".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: None,
        description: None,
        milestone: None,
        url: None,
        repo: None,
        status: None,
        clear: vec!["url".to_string(), "repo".to_string()],
    };

    ProjectCommands::update_project(&storage, &args).unwrap();

    let projects = storage.load_projects().unwrap();
    let updated = projects.iter().find(|p| p.id == id).unwrap();
    assert_eq!(updated.url, None);
    assert_eq!(updated.repo, None);
    assert_eq!(updated.milestone, Some("v1.0".to_string())); // Not cleared
}

#[test]
fn project_update_no_changes() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Test".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: None,
        description: None,
        milestone: None,
        url: None,
        repo: None,
        status: None,
        clear: vec![],
    };

    // Should succeed but print warning
    ProjectCommands::update_project(&storage, &args).unwrap();
}

#[test]
fn project_update_invalid_id() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let args = UpdateProjectArgs {
        id: Uuid::new_v4(),
        title: Some("New Title".to_string()),
        description: None,
        milestone: None,
        url: None,
        repo: None,
        status: None,
        clear: vec![],
    };

    let result = ProjectCommands::update_project(&storage, &args);
    assert!(result.is_err());
}

#[test]
fn project_update_invalid_clear_field() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Test".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: None,
        description: None,
        milestone: None,
        url: None,
        repo: None,
        status: None,
        clear: vec!["invalid_field".to_string()],
    };

    let result = ProjectCommands::update_project(&storage, &args);
    assert!(result.is_err());
}

#[test]
fn project_update_status() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Test".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: None,
        description: None,
        milestone: None,
        url: None,
        repo: None,
        status: Some(ProjectStatus::InProgress),
        clear: vec![],
    };

    ProjectCommands::update_project(&storage, &args).unwrap();

    let projects = storage.load_projects().unwrap();
    let updated = projects.iter().find(|p| p.id == id).unwrap();
    assert_eq!(updated.status, ProjectStatus::InProgress);
}

#[test]
fn project_update_mix_set_and_clear() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let project = Project::new("Test".to_string())
        .with_url("https://old.com".to_string())
        .with_description("Old description".to_string());
    let id = project.id;
    storage.save_projects(&[project]).unwrap();

    let args = UpdateProjectArgs {
        id,
        title: Some("New Title".to_string()),
        description: None,
        milestone: None,
        url: Some("https://new.com".to_string()),
        repo: None,
        status: None,
        clear: vec!["description".to_string()],
    };

    ProjectCommands::update_project(&storage, &args).unwrap();

    let projects = storage.load_projects().unwrap();
    let updated = projects.iter().find(|p| p.id == id).unwrap();
    assert_eq!(updated.title, "New Title");
    assert_eq!(updated.url, Some("https://new.com".to_string()));
    assert_eq!(updated.description, None);
}
