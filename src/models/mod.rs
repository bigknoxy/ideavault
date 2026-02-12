//! Data models for the IdeaVault CLI application
//!
//! This module contains all the core data structures used throughout the application:
//! - Ideas: Individual thoughts, concepts, or notes
//! - Projects: Collections of ideas organized toward a goal
//! - Tags: Labels for categorizing ideas
//! - Config: Application configuration settings

pub mod config;
pub mod idea;
pub mod project;
pub mod tag;
pub mod task;

// Re-export the main types for convenience
pub use config::{BackupConfig, Config, OutputFormat};
pub use idea::{Idea, IdeaStatus};
pub use project::{Project, ProjectStatus};
pub use tag::Tag;
pub use task::{Task, TaskPriority, TaskStatus};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common trait for entities that have timestamps
pub trait Timestamped {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn touch(&mut self);
}

/// Common trait for entities that can be identified by UUID
pub trait Identifiable {
    fn id(&self) -> Uuid;
}

/// Common trait for entities that have a status
pub trait Statusful {
    type Status;
    fn status(&self) -> &Self::Status;
    fn set_status(&mut self, status: Self::Status);
}

impl Timestamped for Idea {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    fn touch(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
}

impl Identifiable for Idea {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Statusful for Idea {
    type Status = IdeaStatus;

    fn status(&self) -> &Self::Status {
        &self.status
    }

    fn set_status(&mut self, status: Self::Status) {
        self.set_status(status);
    }
}

impl Timestamped for Project {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    fn touch(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
}

impl Identifiable for Project {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Statusful for Project {
    type Status = ProjectStatus;

    fn status(&self) -> &Self::Status {
        &self.status
    }

    fn set_status(&mut self, status: Self::Status) {
        self.set_status(status);
    }
}

/// Result type for model operations
pub type ModelResult<T> = Result<T, ModelError>;

/// Errors that can occur in model operations
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ModelError {
    #[error("Entity not found: {id}")]
    NotFound { id: Uuid },

    #[error("Duplicate entity: {id}")]
    Duplicate { id: Uuid },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("IO error: {message}")]
    Io { message: String },

    #[error("Serialization error: {message}")]
    Serialization { message: String },
}

impl From<std::io::Error> for ModelError {
    fn from(err: std::io::Error) -> Self {
        Self::Io {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for ModelError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization {
            message: err.to_string(),
        }
    }
}
