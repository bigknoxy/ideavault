use crate::models::idea::Idea;
use crate::models::project::Project;
use crate::models::tag::Tag;
use crate::models::task::Task;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde_json;
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct Storage {
    data_dir: PathBuf,
    ideas_file: PathBuf,
    projects_file: PathBuf,
    tags_file: PathBuf,
    tasks_file: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "ideavault", "ideavault")
            .context("Failed to get project directories")?;

        let data_dir = proj_dirs.data_dir().to_path_buf();
        Self::new_with_path(data_dir)
    }

    /// Create storage with a custom data directory path.
    /// Useful for testing with temporary directories.
    pub fn new_with_path(data_dir: PathBuf) -> Result<Self> {
        let ideas_file = data_dir.join("ideas.json");
        let projects_file = data_dir.join("projects.json");
        let tags_file = data_dir.join("tags.json");
        let tasks_file = data_dir.join("tasks.json");

        // Ensure data directory exists
        fs::create_dir_all(&data_dir)
            .with_context(|| format!("Failed to create data directory: {:?}", data_dir))?;

        Ok(Self {
            data_dir,
            ideas_file,
            projects_file,
            tags_file,
            tasks_file,
        })
    }

    pub fn load_ideas(&self) -> Result<Vec<Idea>> {
        if !self.ideas_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.ideas_file)
            .with_context(|| format!("Failed to read ideas file: {:?}", self.ideas_file))?;

        let ideas: Vec<Idea> =
            serde_json::from_str(&content).with_context(|| "Failed to parse ideas JSON")?;

        Ok(ideas)
    }

    pub fn save_ideas(&self, ideas: &[Idea]) -> Result<()> {
        let content = serde_json::to_string_pretty(ideas)
            .with_context(|| "Failed to serialize ideas to JSON")?;

        fs::write(&self.ideas_file, content)
            .with_context(|| format!("Failed to write ideas file: {:?}", self.ideas_file))?;

        Ok(())
    }

    pub fn load_projects(&self) -> Result<Vec<Project>> {
        if !self.projects_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.projects_file)
            .with_context(|| format!("Failed to read projects file: {:?}", self.projects_file))?;

        let projects: Vec<Project> =
            serde_json::from_str(&content).with_context(|| "Failed to parse projects JSON")?;

        Ok(projects)
    }

    pub fn save_projects(&self, projects: &[Project]) -> Result<()> {
        let content = serde_json::to_string_pretty(projects)
            .with_context(|| "Failed to serialize projects to JSON")?;

        fs::write(&self.projects_file, content)
            .with_context(|| format!("Failed to write projects file: {:?}", self.projects_file))?;

        Ok(())
    }

    pub fn load_tags(&self) -> Result<Vec<Tag>> {
        if !self.tags_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.tags_file)
            .with_context(|| format!("Failed to read tags file: {:?}", self.tags_file))?;

        let tags: Vec<Tag> =
            serde_json::from_str(&content).with_context(|| "Failed to parse tags JSON")?;

        Ok(tags)
    }

    pub fn save_tags(&self, tags: &[Tag]) -> Result<()> {
        let content = serde_json::to_string_pretty(tags)
            .with_context(|| "Failed to serialize tags to JSON")?;

        fs::write(&self.tags_file, content)
            .with_context(|| format!("Failed to write tags file: {:?}", self.tags_file))?;

        Ok(())
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>> {
        if !self.tasks_file.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&self.tasks_file)
            .with_context(|| format!("Failed to read tasks file: {:?}", self.tasks_file))?;
        let tasks: Vec<Task> =
            serde_json::from_str(&content).with_context(|| "Failed to parse tasks JSON")?;
        Ok(tasks)
    }

    pub fn save_tasks(&self, tasks: &[Task]) -> Result<()> {
        let content = serde_json::to_string_pretty(tasks)
            .with_context(|| "Failed to serialize tasks to JSON")?;
        fs::write(&self.tasks_file, content)
            .with_context(|| format!("Failed to write tasks file: {:?}", self.tasks_file))?;
        Ok(())
    }
}
