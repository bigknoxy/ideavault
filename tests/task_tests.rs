use ideavault::commands::task::{TaskCommands, TaskUpdateArgs};
use ideavault::models::task::{Task, TaskPriority, TaskStatus};
use ideavault::storage::Storage;

#[test]
fn task_update_title() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Original Title".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: Some("New Title".to_string()),
        description: None,
        priority: None,
        due_date: None,
        status: None,
        tags: None,
        clear: vec![],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert_eq!(updated.title, "New Title");
}

#[test]
fn task_update_description() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: Some("New description".to_string()),
        priority: None,
        due_date: None,
        status: None,
        tags: None,
        clear: vec![],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert_eq!(updated.description, Some("New description".to_string()));
}

#[test]
fn task_update_priority() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: Some(TaskPriority::High),
        due_date: None,
        status: None,
        tags: None,
        clear: vec![],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert_eq!(updated.priority, TaskPriority::High);
}

#[test]
fn task_update_due_date() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: Some("2024-12-31".to_string()),
        status: None,
        tags: None,
        clear: vec![],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert!(updated.due_date.is_some());
    assert_eq!(
        updated.due_date.unwrap().format("%Y-%m-%d").to_string(),
        "2024-12-31"
    );
}

#[test]
fn task_update_status() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: None,
        status: Some(TaskStatus::InProgress),
        tags: None,
        clear: vec![],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert_eq!(updated.status, TaskStatus::InProgress);
}

#[test]
fn task_update_tags() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task =
        Task::new("Test".to_string()).with_tags(vec!["old1".to_string(), "old2".to_string()]);
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: None,
        status: None,
        tags: Some(vec!["new1".to_string(), "new2".to_string()]),
        clear: vec![],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert_eq!(updated.tags, vec!["new1", "new2"]);
}

#[test]
fn task_update_multiple_fields() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Original".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: Some("New Title".to_string()),
        description: Some("New description".to_string()),
        priority: Some(TaskPriority::Urgent),
        due_date: None,
        status: Some(TaskStatus::Done),
        tags: None,
        clear: vec![],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert_eq!(updated.title, "New Title");
    assert_eq!(updated.description, Some("New description".to_string()));
    assert_eq!(updated.priority, TaskPriority::Urgent);
    assert_eq!(updated.status, TaskStatus::Done);
}

#[test]
fn task_update_clear_due_date() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    // First set a due date
    let args_set_due = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: Some("2024-12-31".to_string()),
        status: None,
        tags: None,
        clear: vec![],
    };
    TaskCommands::update_task(&storage, &args_set_due).unwrap();

    // Now clear it
    let args_clear_due = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: None,
        status: None,
        tags: None,
        clear: vec!["due_date".to_string()],
    };
    TaskCommands::update_task(&storage, &args_clear_due).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert!(updated.due_date.is_none());
}

#[test]
fn task_update_clear_description() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string()).with_description("Has description".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: None,
        status: None,
        tags: None,
        clear: vec!["description".to_string()],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert!(updated.description.is_none());
}

#[test]
fn task_update_clear_tags() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task =
        Task::new("Test".to_string()).with_tags(vec!["tag1".to_string(), "tag2".to_string()]);
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: None,
        status: None,
        tags: None,
        clear: vec!["tags".to_string()],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert!(updated.tags.is_empty());
}

#[test]
fn task_update_clear_due_date_via_value() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: Some("clear".to_string()),
        status: None,
        tags: None,
        clear: vec![],
    };

    TaskCommands::update_task(&storage, &args).unwrap();

    let tasks = storage.load_tasks().unwrap();
    let updated = tasks.iter().find(|t| t.id == id).unwrap();
    assert!(updated.due_date.is_none());
}

#[test]
fn task_update_no_changes() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: None,
        status: None,
        tags: None,
        clear: vec![],
    };

    // Should succeed but print warning
    TaskCommands::update_task(&storage, &args).unwrap();
}

#[test]
fn task_update_invalid_id() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let args = TaskUpdateArgs {
        id: uuid::Uuid::new_v4(),
        title: Some("New Title".to_string()),
        description: None,
        priority: None,
        due_date: None,
        status: None,
        tags: None,
        clear: vec![],
    };

    let result = TaskCommands::update_task(&storage, &args);
    assert!(result.is_err());
}

#[test]
fn task_update_invalid_clear_field() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: None,
        status: None,
        tags: None,
        clear: vec!["invalid_field".to_string()],
    };

    let result = TaskCommands::update_task(&storage, &args);
    assert!(result.is_err());
}

#[test]
fn task_update_invalid_date_format() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new_with_path(temp_dir.path().to_path_buf()).unwrap();

    let task = Task::new("Test".to_string());
    let id = task.id;
    storage.save_tasks(&[task]).unwrap();

    let args = TaskUpdateArgs {
        id,
        title: None,
        description: None,
        priority: None,
        due_date: Some("invalid-date".to_string()),
        status: None,
        tags: None,
        clear: vec![],
    };

    let result = TaskCommands::update_task(&storage, &args);
    assert!(result.is_err());
}
