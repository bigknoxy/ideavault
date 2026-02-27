use ideavault::models::Project;

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
