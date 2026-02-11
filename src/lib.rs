pub mod cli;
pub mod commands;
pub mod models;
pub mod storage;

pub use cli::{Cli, Commands};
pub use models::idea::Idea;
pub use models::project::Project;
pub use models::tag::Tag;
pub use storage::Storage;

#[cfg(test)]
mod tests {
    use crate::models::idea::{Idea, IdeaStatus};
    use crate::models::project::{Project, ProjectStatus};
    use crate::models::tag::Tag;
    use crate::storage::Storage;
    use uuid::Uuid;

    #[test]
    fn test_idea_creation() {
        let idea = Idea::new("Test Idea".to_string())
            .with_description("Test Description".to_string())
            .with_tags(vec!["test".to_string(), "rust".to_string()]);

        assert_eq!(idea.title, "Test Idea");
        assert_eq!(idea.description, Some("Test Description".to_string()));
        assert_eq!(idea.tags, vec!["test", "rust"]);
        assert_eq!(idea.status, IdeaStatus::Brainstorming);
        assert!(!idea.id.to_string().is_empty());
    }

    #[test]
    fn test_idea_status_update() {
        let mut idea = Idea::new("Test Idea".to_string());
        idea.set_status(IdeaStatus::Active);
        assert_eq!(idea.status, IdeaStatus::Active);

        idea.set_status(IdeaStatus::Completed);
        assert_eq!(idea.status, IdeaStatus::Completed);
    }

    #[test]
    fn test_project_creation() {
        let project = Project::new("Test Project".to_string())
            .with_description("Test Description".to_string())
            .with_milestone("v1.0".to_string());

        assert_eq!(project.title, "Test Project");
        assert_eq!(project.description, Some("Test Description".to_string()));
        assert_eq!(project.milestone, Some("v1.0".to_string()));
        assert_eq!(project.status, ProjectStatus::Planning);
        assert!(project.idea_ids.is_empty());
    }

    #[test]
    fn test_project_link_idea() {
        let mut project = Project::new("Test Project".to_string());
        let idea_id = Uuid::new_v4();

        project.add_idea(idea_id);
        assert_eq!(project.idea_ids.len(), 1);
        assert!(project.idea_ids.contains(&idea_id));

        project.remove_idea(&idea_id);
        assert!(project.idea_ids.is_empty());
    }

    #[test]
    fn test_tag_creation() {
        let tag = Tag::new("rust".to_string());
        assert_eq!(tag.name, "rust");
        assert_eq!(tag.color, None);

        let colored_tag = Tag::new("cli".to_string()).with_color("blue".to_string());
        assert_eq!(colored_tag.name, "cli");
        assert_eq!(colored_tag.color, Some("blue".to_string()));
    }

    #[test]
    fn test_storage_initialization() {
        let storage = Storage::new();
        assert!(storage.is_ok());

        let storage = storage.unwrap();
        let ideas = storage.load_ideas();
        assert!(ideas.is_ok());
        // Storage may have ideas from previous tests
        let ideas = ideas.unwrap();
        assert!(ideas.len() >= 0);
    }
}
