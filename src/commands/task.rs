use crate::models::task::{Task, TaskPriority, TaskStatus};
use crate::storage::Storage;
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use clap::{Args, Parser, Subcommand};
use std::io::{self, Write};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Manage tasks")]
pub struct TaskCommands {
    #[command(subcommand)]
    pub command: TaskSubcommand,
}

#[derive(Subcommand)]
pub enum TaskSubcommand {
    /// Create a new task
    New(NewTaskArgs),
    /// List tasks with optional filtering
    List(ListTaskArgs),
    /// Show full details of a task
    Show(ShowTaskArgs),
    /// Update the status of a task
    Status(StatusTaskArgs),
    /// Update the priority of a task
    Priority(PriorityTaskArgs),
    /// Set due date for a task
    Due(DueTaskArgs),
    /// Link task to a project
    LinkProject(LinkProjectArgs),
    /// Link task to an idea
    LinkIdea(LinkIdeaArgs),
    /// Unlink task from project
    UnlinkProject(UnlinkProjectArgs),
    /// Unlink task from idea
    UnlinkIdea(UnlinkIdeaArgs),
    /// Edit a task in $EDITOR
    Edit(EditTaskArgs),
    /// Delete a task with confirmation
    Delete(DeleteTaskArgs),
}

#[derive(Args)]
pub struct NewTaskArgs {
    /// The title of the task
    title: String,

    /// Optional description for the task
    #[arg(short = 'd', long = "description")]
    description: Option<String>,

    /// Optional priority (low|medium|high|urgent)
    #[arg(short = 'p', long = "priority")]
    priority: Option<TaskPriority>,

    /// Optional due date (YYYY-MM-DD format)
    #[arg(short = 'D', long = "due")]
    due_date: Option<String>,

    /// Optional tags (comma-separated, GTD-style contexts)
    #[arg(short = 't', long = "tags", value_delimiter = ',')]
    tags: Vec<String>,

    /// Optional project ID to link to
    #[arg(long = "project")]
    project_id: Option<Uuid>,

    /// Optional idea ID to link to
    #[arg(long = "idea")]
    idea_id: Option<Uuid>,
}

#[derive(Args)]
pub struct ListTaskArgs {
    /// Filter by status (todo|inprogress|blocked|done|cancelled)
    #[arg(short = 's', long = "status")]
    status: Option<TaskStatus>,

    /// Filter by priority (low|medium|high|urgent)
    #[arg(short = 'p', long = "priority")]
    priority: Option<TaskPriority>,

    /// Filter by tag (GTD-style context)
    #[arg(short = 't', long = "tag")]
    tag: Option<String>,

    /// Filter by project ID
    #[arg(long = "project")]
    project_id: Option<Uuid>,

    /// Filter by idea ID
    #[arg(long = "idea")]
    idea_id: Option<Uuid>,

    /// Show overdue tasks only
    #[arg(long = "overdue")]
    overdue: bool,
}

#[derive(Args)]
pub struct ShowTaskArgs {
    /// The UUID of the task to show
    id: Uuid,
}

#[derive(Args)]
pub struct StatusTaskArgs {
    /// The UUID of the task to update
    id: Uuid,

    /// New status for the task
    status: TaskStatus,
}

#[derive(Args)]
pub struct PriorityTaskArgs {
    /// The UUID of the task to update
    id: Uuid,

    /// New priority for the task
    priority: TaskPriority,
}

#[derive(Args)]
pub struct DueTaskArgs {
    /// The UUID of the task to update
    id: Uuid,

    /// Due date (YYYY-MM-DD format) or "clear" to remove
    due_date: String,
}

#[derive(Args)]
pub struct LinkProjectArgs {
    /// The UUID of the task
    id: Uuid,

    /// The UUID of the project to link
    project_id: Uuid,
}

#[derive(Args)]
pub struct LinkIdeaArgs {
    /// The UUID of the task
    id: Uuid,

    /// The UUID of the idea to link
    idea_id: Uuid,
}

#[derive(Args)]
pub struct UnlinkProjectArgs {
    /// The UUID of the task
    id: Uuid,
}

#[derive(Args)]
pub struct UnlinkIdeaArgs {
    /// The UUID of the task
    id: Uuid,
}

#[derive(Args)]
pub struct EditTaskArgs {
    /// The UUID of the task to edit
    id: Uuid,
}

#[derive(Args)]
pub struct DeleteTaskArgs {
    /// The UUID of the task to delete
    id: Uuid,

    /// Skip confirmation prompt
    #[arg(short, long)]
    force: bool,
}

impl TaskCommands {
    pub fn execute(&self) -> Result<()> {
        let storage = Storage::new().context("Failed to initialize storage")?;

        match &self.command {
            TaskSubcommand::New(args) => Self::new_task(&storage, args),
            TaskSubcommand::List(args) => Self::list_tasks(&storage, args),
            TaskSubcommand::Show(args) => Self::show_task(&storage, args),
            TaskSubcommand::Status(args) => Self::update_status(&storage, args),
            TaskSubcommand::Priority(args) => Self::update_priority(&storage, args),
            TaskSubcommand::Due(args) => Self::update_due_date(&storage, args),
            TaskSubcommand::LinkProject(args) => Self::link_project(&storage, args),
            TaskSubcommand::LinkIdea(args) => Self::link_idea(&storage, args),
            TaskSubcommand::UnlinkProject(args) => Self::unlink_project(&storage, args),
            TaskSubcommand::UnlinkIdea(args) => Self::unlink_idea(&storage, args),
            TaskSubcommand::Edit(args) => Self::edit_task(&storage, args),
            TaskSubcommand::Delete(args) => Self::delete_task(&storage, args),
        }
    }

    fn new_task(storage: &Storage, args: &NewTaskArgs) -> Result<()> {
        let mut task = Task::new(args.title.clone());

        if let Some(description) = &args.description {
            task = task.with_description(description.clone());
        }

        if let Some(priority) = &args.priority {
            task = task.with_priority(priority.clone());
        }

        if !args.tags.is_empty() {
            task = task.with_tags(args.tags.clone());
        }

        if let Some(due_date_str) = &args.due_date {
            let naive_date = NaiveDate::parse_from_str(due_date_str, "%Y-%m-%d")
                .map_err(|_| anyhow::anyhow!("Invalid date format. Use YYYY-MM-DD"))?;
            let due_date = DateTime::<Utc>::from_naive_utc_and_offset(
                naive_date.and_hms_opt(0, 0, 0).unwrap(),
                Utc,
            );
            task = task.with_due_date(due_date);
        }

        if let Some(project_id) = &args.project_id {
            task = task.with_project(*project_id);
        }

        if let Some(idea_id) = &args.idea_id {
            task = task.with_idea(*idea_id);
        }

        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;
        tasks.push(task.clone());
        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Created new task:");
        print_task_summary(&task);
        Ok(())
    }

    fn list_tasks(storage: &Storage, args: &ListTaskArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;

        if let Some(status_filter) = &args.status {
            tasks.retain(|task| &task.status == status_filter);
        }

        if let Some(priority_filter) = &args.priority {
            tasks.retain(|task| &task.priority == priority_filter);
        }

        if let Some(tag_filter) = &args.tag {
            tasks.retain(|task| task.tags.contains(tag_filter));
        }

        if let Some(project_filter) = &args.project_id {
            tasks.retain(|task| task.project_id == Some(*project_filter));
        }

        if let Some(idea_filter) = &args.idea_id {
            tasks.retain(|task| task.idea_id == Some(*idea_filter));
        }

        if args.overdue {
            let now = Utc::now();
            tasks.retain(|task| {
                if let Some(due) = task.due_date {
                    due < now
                        && task.status != TaskStatus::Done
                        && task.status != TaskStatus::Cancelled
                } else {
                    false
                }
            });
        }

        if tasks.is_empty() {
            println!("📋 No tasks found");
            return Ok(());
        }

        println!("📋 Found {} task(s):", tasks.len());
        println!();

        for task in &tasks {
            print_task_summary(task);
            println!();
        }

        Ok(())
    }

    fn show_task(storage: &Storage, args: &ShowTaskArgs) -> Result<()> {
        let tasks = storage.load_tasks().context("Failed to load tasks")?;
        let projects = storage.load_projects().context("Failed to load projects")?;
        let ideas = storage.load_ideas().context("Failed to load ideas")?;

        let task = tasks
            .iter()
            .find(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        print_task_full(task, &projects, &ideas);
        Ok(())
    }

    fn update_status(storage: &Storage, args: &StatusTaskArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;

        let task = tasks
            .iter_mut()
            .find(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        let old_status = task.status.clone();
        task.status = args.status.clone();
        task.updated_at = Utc::now();

        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Updated status for task {}:", args.id);
        println!("   {} → {}", old_status, args.status);
        Ok(())
    }

    fn update_priority(storage: &Storage, args: &PriorityTaskArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;

        let task = tasks
            .iter_mut()
            .find(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        let old_priority = task.priority.clone();
        task.priority = args.priority.clone();
        task.updated_at = Utc::now();

        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Updated priority for task {}:", args.id);
        println!("   {} → {}", old_priority, args.priority);
        Ok(())
    }

    fn update_due_date(storage: &Storage, args: &DueTaskArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;

        let task = tasks
            .iter_mut()
            .find(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        if args.due_date.to_lowercase() == "clear" {
            task.due_date = None;
            println!("✅ Cleared due date for task {}", args.id);
        } else {
            let naive_date = NaiveDate::parse_from_str(&args.due_date, "%Y-%m-%d")
                .map_err(|_| anyhow::anyhow!("Invalid date format. Use YYYY-MM-DD"))?;
            let due_date = DateTime::<Utc>::from_naive_utc_and_offset(
                naive_date.and_hms_opt(0, 0, 0).unwrap(),
                Utc,
            );
            task.due_date = Some(due_date);
            println!("✅ Set due date for task {} to {}", args.id, args.due_date);
        }
        task.updated_at = Utc::now();

        storage.save_tasks(&tasks).context("Failed to save tasks")?;
        Ok(())
    }

    fn link_project(storage: &Storage, args: &LinkProjectArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;
        let projects = storage.load_projects().context("Failed to load projects")?;

        if !projects.iter().any(|p| p.id == args.project_id) {
            return Err(anyhow::anyhow!(
                "Project with ID {} not found",
                args.project_id
            ));
        }

        let task = tasks
            .iter_mut()
            .find(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        task.project_id = Some(args.project_id);
        task.updated_at = Utc::now();

        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Linked task {} to project {}", args.id, args.project_id);
        Ok(())
    }

    fn link_idea(storage: &Storage, args: &LinkIdeaArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;
        let ideas = storage.load_ideas().context("Failed to load ideas")?;

        if !ideas.iter().any(|i| i.id == args.idea_id) {
            return Err(anyhow::anyhow!("Idea with ID {} not found", args.idea_id));
        }

        let task = tasks
            .iter_mut()
            .find(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        task.idea_id = Some(args.idea_id);
        task.updated_at = Utc::now();

        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Linked task {} to idea {}", args.id, args.idea_id);
        Ok(())
    }

    fn unlink_project(storage: &Storage, args: &UnlinkProjectArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;

        let task = tasks
            .iter_mut()
            .find(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        if task.project_id.is_none() {
            println!("⚠️  Task {} is not linked to any project", args.id);
            return Ok(());
        }

        task.project_id = None;
        task.updated_at = Utc::now();

        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Unlinked task {} from project", args.id);
        Ok(())
    }

    fn unlink_idea(storage: &Storage, args: &UnlinkIdeaArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;

        let task = tasks
            .iter_mut()
            .find(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        if task.idea_id.is_none() {
            println!("⚠️  Task {} is not linked to any idea", args.id);
            return Ok(());
        }

        task.idea_id = None;
        task.updated_at = Utc::now();

        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Unlinked task {} from idea", args.id);
        Ok(())
    }

    fn edit_task(storage: &Storage, args: &EditTaskArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;

        let task_index = tasks
            .iter()
            .position(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        let temp_file = format!("{}.md", args.id);
        let content = format!(
            "# {}\n\n{}\n\nPriority: {}\nStatus: {}\nTags: {}\n\n",
            tasks[task_index].title,
            tasks[task_index].description.as_deref().unwrap_or(""),
            tasks[task_index].priority,
            tasks[task_index].status,
            tasks[task_index].tags.join(", ")
        );

        std::fs::write(&temp_file, content).context("Failed to create temp file")?;

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        let status = std::process::Command::new(&editor)
            .arg(&temp_file)
            .status()
            .context("Failed to open editor")?;

        if !status.success() {
            std::fs::remove_file(&temp_file)?;
            return Err(anyhow::anyhow!("Editor exited with non-zero status"));
        }

        let updated_content =
            std::fs::read_to_string(&temp_file).context("Failed to read updated content")?;
        std::fs::remove_file(&temp_file)?;

        let lines: Vec<&str> = updated_content.lines().collect();
        if let Some(title_line) = lines.iter().find(|line| line.starts_with("# ")) {
            tasks[task_index].title = title_line.strip_prefix("# ").unwrap().trim().to_string();
        }

        let mut description_parts = Vec::new();
        let mut in_description = false;

        for line in &lines {
            if line.starts_with("# ") {
                in_description = true;
                continue;
            } else if line.starts_with("Priority:")
                || line.starts_with("Status:")
                || line.starts_with("Tags:")
            {
                in_description = false;
                continue;
            } else if in_description && !line.trim().is_empty() {
                description_parts.push(line.trim());
            }
        }

        if !description_parts.is_empty() {
            tasks[task_index].description = Some(description_parts.join("\n"));
        }

        for line in lines.iter().rev() {
            if line.starts_with("Priority:") {
                let priority_str = line.strip_prefix("Priority:").unwrap().trim();
                if let Ok(parsed_priority) = priority_str.parse() {
                    tasks[task_index].priority = parsed_priority;
                }
            } else if line.starts_with("Status:") {
                let status_str = line.strip_prefix("Status:").unwrap().trim();
                if let Ok(parsed_status) = status_str.parse() {
                    tasks[task_index].status = parsed_status;
                }
            } else if line.starts_with("Tags:") {
                let tags_str = line.strip_prefix("Tags:").unwrap().trim();
                tasks[task_index].tags.clear();
                if !tags_str.is_empty() {
                    tasks[task_index].tags =
                        tags_str.split(',').map(|t| t.trim().to_string()).collect();
                }
            }
        }

        tasks[task_index].updated_at = Utc::now();
        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Updated task {}:", args.id);
        print_task_summary(&tasks[task_index]);
        Ok(())
    }

    fn delete_task(storage: &Storage, args: &DeleteTaskArgs) -> Result<()> {
        let mut tasks = storage.load_tasks().context("Failed to load tasks")?;

        let task_index = tasks
            .iter()
            .position(|task| task.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Task with ID {} not found", args.id))?;

        let task = &tasks[task_index];

        if !args.force {
            print_task_summary(task);
            println!();
            print!("Are you sure you want to delete this task? [y/N]: ");
            io::stdout().flush().context("Failed to flush output")?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .context("Failed to read input")?;

            let response = input.trim().to_lowercase();
            if !matches!(response.as_str(), "y" | "yes") {
                println!("❌ Deletion cancelled");
                return Ok(());
            }
        }

        let deleted_task = tasks.remove(task_index);
        storage.save_tasks(&tasks).context("Failed to save tasks")?;

        println!("✅ Deleted task: {}", deleted_task.title);
        Ok(())
    }
}

fn print_task_summary(task: &Task) {
    let status_emoji = match task.status {
        TaskStatus::Todo => "📋",
        TaskStatus::InProgress => "🔄",
        TaskStatus::Blocked => "🚫",
        TaskStatus::Done => "✅",
        TaskStatus::Cancelled => "❌",
    };

    let priority_emoji = match task.priority {
        TaskPriority::Low => "⬇️",
        TaskPriority::Medium => "➡️",
        TaskPriority::High => "⬆️",
        TaskPriority::Urgent => "🔴",
    };

    println!(
        "{} {} {} [{}]",
        status_emoji, priority_emoji, task.title, task.id
    );

    if let Some(description) = &task.description {
        let desc_preview = if description.len() > 50 {
            format!("{}...", &description[..50])
        } else {
            description.clone()
        };
        println!("   {}", desc_preview);
    }

    if !task.tags.is_empty() {
        println!("   🏷️  {}", task.tags.join(", "));
    }

    if let Some(due_date) = &task.due_date {
        let now = Utc::now();
        let is_overdue = *due_date < now
            && task.status != TaskStatus::Done
            && task.status != TaskStatus::Cancelled;
        if is_overdue {
            println!("   ⏰ Due: {} (OVERDUE)", due_date.format("%Y-%m-%d"));
        } else {
            println!("   ⏰ Due: {}", due_date.format("%Y-%m-%d"));
        }
    }

    if task.project_id.is_some() {
        println!("   📁 Linked to project");
    }
    if task.idea_id.is_some() {
        println!("   💡 Linked to idea");
    }

    println!(
        "   📅 Updated: {}",
        task.updated_at.format("%Y-%m-%d %H:%M")
    );
}

fn print_task_full(
    task: &Task,
    projects: &[crate::models::Project],
    ideas: &[crate::models::Idea],
) {
    let status_emoji = match task.status {
        TaskStatus::Todo => "📋",
        TaskStatus::InProgress => "🔄",
        TaskStatus::Blocked => "🚫",
        TaskStatus::Done => "✅",
        TaskStatus::Cancelled => "❌",
    };

    let priority_emoji = match task.priority {
        TaskPriority::Low => "⬇️",
        TaskPriority::Medium => "➡️",
        TaskPriority::High => "⬆️",
        TaskPriority::Urgent => "🔴",
    };

    println!("{} {} {}", status_emoji, priority_emoji, task.title);
    println!("ID: {}", task.id);
    println!("Status: {}", task.status);
    println!("Priority: {}", task.priority);

    if let Some(due_date) = &task.due_date {
        let now = Utc::now();
        let is_overdue = *due_date < now
            && task.status != TaskStatus::Done
            && task.status != TaskStatus::Cancelled;
        if is_overdue {
            println!(
                "Due Date: {} (OVERDUE)",
                due_date.format("%Y-%m-%d %H:%M UTC")
            );
        } else {
            println!("Due Date: {}", due_date.format("%Y-%m-%d %H:%M UTC"));
        }
    } else {
        println!("Due Date: Not set");
    }

    if !task.tags.is_empty() {
        println!("Tags (Contexts): {}", task.tags.join(", "));
    }

    if let Some(project_id) = &task.project_id {
        let project = projects.iter().find(|p| p.id == *project_id);
        if let Some(p) = project {
            println!("Project: {} [{}]", p.title, p.id);
        } else {
            println!("Project: {} (not found)", project_id);
        }
    } else {
        println!("Project: Not linked");
    }

    if let Some(idea_id) = &task.idea_id {
        let idea = ideas.iter().find(|i| i.id == *idea_id);
        if let Some(i) = idea {
            println!("Idea: {} [{}]", i.title, i.id);
        } else {
            println!("Idea: {} (not found)", idea_id);
        }
    } else {
        println!("Idea: Not linked");
    }

    println!(
        "Created: {}",
        task.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "Updated: {}",
        task.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!();

    if let Some(description) = &task.description {
        println!("Description:");
        println!("{}", description);
    } else {
        println!("No description");
    }
}
