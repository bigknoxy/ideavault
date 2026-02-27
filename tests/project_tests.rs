use ideavault::models::project::Project;

#[test]
fn project_url_and_repo_roundtrip() {
    let p = Project::new("Test Project".to_string())
        .with_url("https://example.com".to_string())
        .with_repo("https://github.com/example/repo".to_string());

    assert_eq!(p.url.unwrap(), "https://example.com");
    assert_eq!(p.repo.unwrap(), "https://github.com/example/repo");
}
