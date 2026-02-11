use crate::models::idea::Idea;
use crate::models::project::{Project, ProjectStatus};
use crate::storage::Storage;
use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::io::{self, Write};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "project")]
#[command(about = "Manage projects")]
pub struct ProjectCommands {
    #[command(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Subcommand)]
pub enum ProjectSubcommand {
    /// Create a new project
    New(NewProjectArgs),
    /// List projects with optional filtering
    List(ListProjectArgs),
    /// Show full details of a project
    Show(ShowProjectArgs),
    /// Link an idea to a project
    Link(LinkArgs),
    /// Remove idea link from project
    Unlink(UnlinkArgs),
    /// List all ideas linked to a project
    Ideas(IdeasArgs),
    /// Update the status of a project
    Status(StatusArgs),
    /// Delete a project with confirmation
    Delete(DeleteProjectArgs),
}

#[derive(Args)]
pub struct NewProjectArgs {
    /// The title of the project
    title: String,

    /// Optional description for the project
    #[arg(short = 'd', long = "description")]
    description: Option<String>,

    /// Optional milestone for the project
    #[arg(short = 'm', long = "milestone")]
    milestone: Option<String>,
}

#[derive(Args)]
pub struct ListProjectArgs {
    /// Filter by status (Planning|InProgress|Completed|OnHold)
    #[arg(short = 's', long = "status")]
    status: Option<ProjectStatus>,
}

#[derive(Args)]
pub struct ShowProjectArgs {
    /// The UUID of the project to show
    id: Uuid,
}

#[derive(Args)]
pub struct LinkArgs {
    /// The UUID of the project
    #[arg(name = "project-id")]
    project_id: Uuid,

    /// The UUID of the idea to link
    #[arg(name = "idea-id")]
    idea_id: Uuid,
}

#[derive(Args)]
pub struct UnlinkArgs {
    /// The UUID of the project
    #[arg(name = "project-id")]
    project_id: Uuid,

    /// The UUID of the idea to unlink
    #[arg(name = "idea-id")]
    idea_id: Uuid,
}

#[derive(Args)]
pub struct IdeasArgs {
    /// The UUID of the project
    id: Uuid,
}

#[derive(Args)]
pub struct StatusArgs {
    /// The UUID of the project to update
    id: Uuid,

    /// New status for the project
    status: ProjectStatus,
}

#[derive(Args)]
pub struct DeleteProjectArgs {
    /// The UUID of the project to delete
    id: Uuid,

    /// Skip confirmation prompt
    #[arg(short, long)]
    force: bool,
}

impl ProjectCommands {
    pub fn execute(&self) -> Result<()> {
        let storage = Storage::new().context("Failed to initialize storage")?;

        match &self.command {
            ProjectSubcommand::New(args) => Self::new_project(&storage, args),
            ProjectSubcommand::List(args) => Self::list_projects(&storage, args),
            ProjectSubcommand::Show(args) => Self::show_project(&storage, args),
            ProjectSubcommand::Link(args) => Self::link_idea(&storage, args),
            ProjectSubcommand::Unlink(args) => Self::unlink_idea(&storage, args),
            ProjectSubcommand::Ideas(args) => Self::list_project_ideas(&storage, args),
            ProjectSubcommand::Status(args) => Self::update_status(&storage, args),
            ProjectSubcommand::Delete(args) => Self::delete_project(&storage, args),
        }
    }

    fn new_project(storage: &Storage, args: &NewProjectArgs) -> Result<()> {
        let mut project = Project::new(args.title.clone());

        if let Some(description) = &args.description {
            project = project.with_description(description.clone());
        }

        if let Some(milestone) = &args.milestone {
            project = project.with_milestone(milestone.clone());
        }

        let mut projects = storage.load_projects().context("Failed to load projects")?;
        projects.push(project.clone());
        storage
            .save_projects(&projects)
            .context("Failed to save projects")?;

        println!("✅ Created new project:");
        print_project_summary(&project);
        Ok(())
    }

    fn list_projects(storage: &Storage, args: &ListProjectArgs) -> Result<()> {
        let mut projects = storage.load_projects().context("Failed to load projects")?;

        // Apply filters
        if let Some(status_filter) = &args.status {
            projects.retain(|project| &project.status == status_filter);
        }

        if projects.is_empty() {
            println!("📋 No projects found");
            return Ok(());
        }

        println!("📋 Found {} project(s):", projects.len());
        println!();

        for project in &projects {
            print_project_summary(project);
            println!();
        }

        Ok(())
    }

    fn show_project(storage: &Storage, args: &ShowProjectArgs) -> Result<()> {
        let projects = storage.load_projects().context("Failed to load projects")?;
        let ideas = storage.load_ideas().context("Failed to load ideas")?;

        let project = projects
            .iter()
            .find(|project| project.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID {} not found", args.id))?;

        print_project_full(project, &ideas);
        Ok(())
    }

    fn link_idea(storage: &Storage, args: &LinkArgs) -> Result<()> {
        let mut projects = storage.load_projects().context("Failed to load projects")?;
        let ideas = storage.load_ideas().context("Failed to load ideas")?;

        // Verify idea exists
        if !ideas.iter().any(|idea| idea.id == args.idea_id) {
            return Err(anyhow::anyhow!("Idea with ID {} not found", args.idea_id));
        }

        let project = projects
            .iter_mut()
            .find(|project| project.id == args.project_id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID {} not found", args.project_id))?;

        if project.idea_ids.contains(&args.idea_id) {
            println!(
                "⚠️  Idea {} is already linked to project {}",
                args.idea_id, args.project_id
            );
            return Ok(());
        }

        project.add_idea(args.idea_id);
        storage
            .save_projects(&projects)
            .context("Failed to save projects")?;

        println!(
            "✅ Linked idea {} to project {}",
            args.idea_id, args.project_id
        );
        Ok(())
    }

    fn unlink_idea(storage: &Storage, args: &UnlinkArgs) -> Result<()> {
        let mut projects = storage.load_projects().context("Failed to load projects")?;

        let project = projects
            .iter_mut()
            .find(|project| project.id == args.project_id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID {} not found", args.project_id))?;

        if !project.idea_ids.contains(&args.idea_id) {
            println!(
                "⚠️  Idea {} is not linked to project {}",
                args.idea_id, args.project_id
            );
            return Ok(());
        }

        project.remove_idea(&args.idea_id);
        storage
            .save_projects(&projects)
            .context("Failed to save projects")?;

        println!(
            "✅ Unlinked idea {} from project {}",
            args.idea_id, args.project_id
        );
        Ok(())
    }

    fn list_project_ideas(storage: &Storage, args: &IdeasArgs) -> Result<()> {
        let projects = storage.load_projects().context("Failed to load projects")?;
        let ideas = storage.load_ideas().context("Failed to load ideas")?;

        let project = projects
            .iter()
            .find(|project| project.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID {} not found", args.id))?;

        if project.idea_ids.is_empty() {
            println!("📋 No ideas linked to project {}", args.id);
            return Ok(());
        }

        println!("💡 Ideas linked to project {}:", project.title);
        println!("   Total: {} ideas", project.idea_ids.len());
        println!();

        for idea_id in &project.idea_ids {
            if let Some(idea) = ideas.iter().find(|idea| idea.id == *idea_id) {
                print_idea_in_project(idea);
                println!();
            } else {
                println!("⚠️  Idea {} not found in storage", idea_id);
            }
        }

        Ok(())
    }

    fn update_status(storage: &Storage, args: &StatusArgs) -> Result<()> {
        let mut projects = storage.load_projects().context("Failed to load projects")?;

        let project = projects
            .iter_mut()
            .find(|project| project.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID {} not found", args.id))?;

        let old_status = project.status.clone();
        project.set_status(args.status.clone());

        storage
            .save_projects(&projects)
            .context("Failed to save projects")?;

        println!("✅ Updated status for project {}:", args.id);
        println!("   {} → {}", old_status, args.status);
        Ok(())
    }

    fn delete_project(storage: &Storage, args: &DeleteProjectArgs) -> Result<()> {
        let mut projects = storage.load_projects().context("Failed to load projects")?;

        let project_index = projects
            .iter()
            .position(|project| project.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID {} not found", args.id))?;

        let project = &projects[project_index];

        if !args.force {
            println!("📋 Project to delete:");
            print_project_summary(project);

            if !project.idea_ids.is_empty() {
                println!(
                    "⚠️  This project has {} linked ideas. They will not be deleted.",
                    project.idea_ids.len()
                );
            }

            print!("Are you sure you want to delete this project? [y/N]: ");
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

        let deleted_project = projects.remove(project_index);
        storage
            .save_projects(&projects)
            .context("Failed to save projects")?;

        println!("✅ Deleted project: {}", deleted_project.title);
        Ok(())
    }
}

fn print_project_summary(project: &Project) {
    let status_emoji = match project.status {
        ProjectStatus::Planning => "📋",
        ProjectStatus::InProgress => "🚀",
        ProjectStatus::Completed => "✅",
        ProjectStatus::OnHold => "⏸️",
    };

    println!("{} {} [{}]", status_emoji, project.title, project.id);
    if let Some(description) = &project.description {
        let desc_preview = if description.len() > 50 {
            format!("{}...", &description[..50])
        } else {
            description.clone()
        };
        println!("   {}", desc_preview);
    }
    if let Some(milestone) = &project.milestone {
        println!("   🎯 {}", milestone);
    }
    if !project.idea_ids.is_empty() {
        println!("   💡 {} idea(s)", project.idea_ids.len());
    }
    println!(
        "   📅 Updated: {}",
        project.updated_at.format("%Y-%m-%d %H:%M")
    );
}

fn print_project_full(project: &Project, ideas: &[Idea]) {
    let status_emoji = match project.status {
        ProjectStatus::Planning => "📋",
        ProjectStatus::InProgress => "🚀",
        ProjectStatus::Completed => "✅",
        ProjectStatus::OnHold => "⏸️",
    };

    println!("{} {}", status_emoji, project.title);
    println!("ID: {}", project.id);
    println!("Status: {}", project.status);

    if let Some(milestone) = &project.milestone {
        println!("Milestone: {}", milestone);
    }

    println!("Ideas: {} linked", project.idea_ids.len());
    println!(
        "Created: {}",
        project.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "Updated: {}",
        project.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!();

    if let Some(description) = &project.description {
        println!("Description:");
        println!("{}", description);
    } else {
        println!("No description");
    }

    // Show linked ideas
    if !project.idea_ids.is_empty() {
        println!();
        println!("💡 Linked Ideas:");
        for idea_id in &project.idea_ids {
            if let Some(idea) = ideas.iter().find(|i| i.id == *idea_id) {
                print_idea_in_project(idea);
            } else {
                println!("  - {} (not found)", idea_id);
            }
        }
    }
}

fn print_idea_in_project(idea: &Idea) {
    let status_emoji = match idea.status {
        crate::models::idea::IdeaStatus::Brainstorming => "🧠",
        crate::models::idea::IdeaStatus::Active => "🚀",
        crate::models::idea::IdeaStatus::Completed => "✅",
        crate::models::idea::IdeaStatus::Archived => "📦",
    };

    println!("  {} {} [{}]", status_emoji, idea.title, idea.id);
    if let Some(description) = &idea.description {
        let desc_preview = if description.len() > 80 {
            format!("{}...", &description[..80])
        } else {
            description.clone()
        };
        println!("     {}", desc_preview);
    }
    if !idea.tags.is_empty() {
        println!("     🏷️  {}", idea.tags.join(", "));
    }
    println!("     📅 {}", idea.updated_at.format("%Y-%m-%d %H:%M"));
}

// Implement FromStr for ProjectStatus for CLI parsing
impl std::str::FromStr for ProjectStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "planning" => Ok(ProjectStatus::Planning),
            "inprogress" | "in-progress" | "progress" => Ok(ProjectStatus::InProgress),
            "completed" | "complete" | "done" => Ok(ProjectStatus::Completed),
            "onhold" | "on-hold" | "hold" => Ok(ProjectStatus::OnHold),
            _ => Err(anyhow::anyhow!(
                "Invalid status. Must be one of: Planning, InProgress, Completed, OnHold"
            )),
        }
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "Planning"),
            ProjectStatus::InProgress => write!(f, "InProgress"),
            ProjectStatus::Completed => write!(f, "Completed"),
            ProjectStatus::OnHold => write!(f, "OnHold"),
        }
    }
}
