use ideavault::commands::idea::{IdeaCommands, IdeaUpdateArgs};
use ideavault::models::idea::{Idea, IdeaStatus};
use ideavault::storage::Storage;
use uuid::Uuid;

#[test]
fn idea_update_title() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let idea = Idea::new("Original Title".to_string());
    let id = idea.id;
    storage.save_ideas(&[idea]).unwrap();

    let args = IdeaUpdateArgs {
        id,
        title: Some("New Title".to_string()),
        description: None,
        status: None,
        clear: vec![],
    };

    IdeaCommands::update_idea(&storage, &args).unwrap();

    let ideas = storage.load_ideas().unwrap();
    let updated = ideas.iter().find(|i| i.id == id).unwrap();
    assert_eq!(updated.title, "New Title");
}

#[test]
fn idea_update_description() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let idea = Idea::new("Test".to_string());
    let id = idea.id;
    storage.save_ideas(&[idea]).unwrap();

    let args = IdeaUpdateArgs {
        id,
        title: None,
        description: Some("New description".to_string()),
        status: None,
        clear: vec![],
    };

    IdeaCommands::update_idea(&storage, &args).unwrap();

    let ideas = storage.load_ideas().unwrap();
    let updated = ideas.iter().find(|i| i.id == id).unwrap();
    assert_eq!(updated.description, Some("New description".to_string()));
}

#[test]
fn idea_update_multiple_fields() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let idea = Idea::new("Original".to_string());
    let id = idea.id;
    storage.save_ideas(&[idea]).unwrap();

    let args = IdeaUpdateArgs {
        id,
        title: Some("New Title".to_string()),
        description: Some("New description".to_string()),
        status: Some(IdeaStatus::Active),
        clear: vec![],
    };

    IdeaCommands::update_idea(&storage, &args).unwrap();

    let ideas = storage.load_ideas().unwrap();
    let updated = ideas.iter().find(|i| i.id == id).unwrap();
    assert_eq!(updated.title, "New Title");
    assert_eq!(updated.description, Some("New description".to_string()));
    assert_eq!(updated.status, IdeaStatus::Active);
}

#[test]
fn idea_update_clear_description() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let idea = Idea::new("Test".to_string()).with_description("Original description".to_string());
    let id = idea.id;
    storage.save_ideas(&[idea]).unwrap();

    let args = IdeaUpdateArgs {
        id,
        title: None,
        description: None,
        status: None,
        clear: vec!["description".to_string()],
    };

    IdeaCommands::update_idea(&storage, &args).unwrap();

    let ideas = storage.load_ideas().unwrap();
    let updated = ideas.iter().find(|i| i.id == id).unwrap();
    assert_eq!(updated.description, None);
}

#[test]
fn idea_update_no_changes() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let idea = Idea::new("Test".to_string());
    let id = idea.id;
    storage.save_ideas(&[idea]).unwrap();

    let args = IdeaUpdateArgs {
        id,
        title: None,
        description: None,
        status: None,
        clear: vec![],
    };

    // Should succeed but print warning
    IdeaCommands::update_idea(&storage, &args).unwrap();
}

#[test]
fn idea_update_invalid_id() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let args = IdeaUpdateArgs {
        id: Uuid::new_v4(),
        title: Some("New Title".to_string()),
        description: None,
        status: None,
        clear: vec![],
    };

    let result = IdeaCommands::update_idea(&storage, &args);
    assert!(result.is_err());
}

#[test]
fn idea_update_invalid_clear_field() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let idea = Idea::new("Test".to_string());
    let id = idea.id;
    storage.save_ideas(&[idea]).unwrap();

    let args = IdeaUpdateArgs {
        id,
        title: None,
        description: None,
        status: None,
        clear: vec!["invalid_field".to_string()],
    };

    let result = IdeaCommands::update_idea(&storage, &args);
    assert!(result.is_err());
}

#[test]
fn idea_update_status() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let idea = Idea::new("Test".to_string());
    let id = idea.id;
    storage.save_ideas(&[idea]).unwrap();

    let args = IdeaUpdateArgs {
        id,
        title: None,
        description: None,
        status: Some(IdeaStatus::Completed),
        clear: vec![],
    };

    IdeaCommands::update_idea(&storage, &args).unwrap();

    let ideas = storage.load_ideas().unwrap();
    let updated = ideas.iter().find(|i| i.id == id).unwrap();
    assert_eq!(updated.status, IdeaStatus::Completed);
}

#[test]
fn idea_update_mix_set_and_clear() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let idea = Idea::new("Test".to_string()).with_description("Old description".to_string());
    let id = idea.id;
    storage.save_ideas(&[idea]).unwrap();

    let args = IdeaUpdateArgs {
        id,
        title: Some("New Title".to_string()),
        description: None,
        status: None,
        clear: vec!["description".to_string()],
    };

    IdeaCommands::update_idea(&storage, &args).unwrap();

    let ideas = storage.load_ideas().unwrap();
    let updated = ideas.iter().find(|i| i.id == id).unwrap();
    assert_eq!(updated.title, "New Title");
    assert_eq!(updated.description, None);
}
