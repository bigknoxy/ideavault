use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    OnHold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub milestone: Option<String>,
    pub status: ProjectStatus,
    pub idea_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            milestone: None,
            status: ProjectStatus::Planning,
            idea_ids: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self.updated_at = Utc::now();
        self
    }

    pub fn with_milestone(mut self, milestone: String) -> Self {
        self.milestone = Some(milestone);
        self.updated_at = Utc::now();
        self
    }

    pub fn with_ideas(mut self, idea_ids: Vec<Uuid>) -> Self {
        self.idea_ids = idea_ids;
        self.updated_at = Utc::now();
        self
    }

    pub fn with_status(mut self, status: ProjectStatus) -> Self {
        self.status = status;
        self.updated_at = Utc::now();
        self
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    pub fn update_milestone(&mut self, milestone: Option<String>) {
        self.milestone = milestone;
        self.updated_at = Utc::now();
    }

    pub fn add_idea(&mut self, idea_id: Uuid) {
        if !self.idea_ids.contains(&idea_id) {
            self.idea_ids.push(idea_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_idea(&mut self, idea_id: &Uuid) {
        if let Some(pos) = self.idea_ids.iter().position(|id| id == idea_id) {
            self.idea_ids.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    pub fn set_status(&mut self, status: ProjectStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn get_idea_count(&self) -> usize {
        self.idea_ids.len()
    }
}
