use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub project_id: Option<Uuid>,
    pub idea_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            due_date: None,
            project_id: None,
            idea_id: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self.updated_at = Utc::now();
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self.updated_at = Utc::now();
        self
    }

    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self.updated_at = Utc::now();
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self.updated_at = Utc::now();
        self
    }

    pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self.updated_at = Utc::now();
        self
    }

    pub fn with_project(mut self, project_id: Uuid) -> Self {
        self.project_id = Some(project_id);
        self.updated_at = Utc::now();
        self
    }

    pub fn with_idea(mut self, idea_id: Uuid) -> Self {
        self.idea_id = Some(idea_id);
        self.updated_at = Utc::now();
        self
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "todo" | "t" => Ok(TaskStatus::Todo),
            "inprogress" | "in-progress" | "progress" | "ip" => Ok(TaskStatus::InProgress),
            "blocked" | "block" | "b" => Ok(TaskStatus::Blocked),
            "done" | "complete" | "d" | "x" => Ok(TaskStatus::Done),
            "cancelled" | "cancel" | "c" => Ok(TaskStatus::Cancelled),
            _ => Err(anyhow::anyhow!(
                "Invalid status. Must be one of: todo, inprogress, blocked, done, cancelled"
            )),
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "Todo"),
            TaskStatus::InProgress => write!(f, "InProgress"),
            TaskStatus::Blocked => write!(f, "Blocked"),
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl std::str::FromStr for TaskPriority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Ok(TaskPriority::Low),
            "medium" | "m" | "med" => Ok(TaskPriority::Medium),
            "high" | "h" => Ok(TaskPriority::High),
            "urgent" | "u" | "crit" | "critical" => Ok(TaskPriority::Urgent),
            _ => Err(anyhow::anyhow!(
                "Invalid priority. Must be one of: low, medium, high, urgent"
            )),
        }
    }
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "Low"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::High => write!(f, "High"),
            TaskPriority::Urgent => write!(f, "Urgent"),
        }
    }
}
