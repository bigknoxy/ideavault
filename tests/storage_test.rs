use anyhow::Result;
use ideavault::{Project, Storage, Tag};

#[test]
fn test_projects_and_tags_storage() -> Result<()> {
    let storage = Storage::new()?;

    // Test projects storage
    let projects = storage.load_projects()?;
    // Storage may have projects from previous tests
    let initial_count = projects.len();

    let project =
        Project::new("Test Project".to_string()).with_description("A test project".to_string());
    let mut all_projects = storage.load_projects()?;
    all_projects.push(project);
    storage.save_projects(&all_projects)?;

    let loaded_projects = storage.load_projects()?;
    assert_eq!(loaded_projects.len(), initial_count + 1);
    assert!(loaded_projects.iter().any(|p| p.title == "Test Project"));

    // Test tags storage
    let tags = storage.load_tags()?;
    let initial_tags_count = tags.len();

    let tag = Tag::new("test-tag".to_string()).with_color("#ff0000".to_string());
    let mut all_tags = storage.load_tags()?;
    all_tags.push(tag);
    storage.save_tags(&all_tags)?;

    let loaded_tags = storage.load_tags()?;
    assert_eq!(loaded_tags.len(), initial_tags_count + 1);
    assert!(loaded_tags.iter().any(|t| t.name == "test-tag"));

    Ok(())
}
