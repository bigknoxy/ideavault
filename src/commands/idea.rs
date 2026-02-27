use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::env;
use std::io::{self, Write};
use std::process::Command;
use uuid::Uuid;

use crate::models::idea::{Idea, IdeaStatus};
use crate::storage::Storage;

#[derive(Parser)]
#[command(name = "idea")]
#[command(about = "Manage ideas")]
pub struct IdeaCommands {
    #[command(subcommand)]
    pub command: IdeaSubcommand,
}

#[derive(Subcommand)]
pub enum IdeaSubcommand {
    /// Create a new idea
    New(NewIdeaArgs),
    /// List ideas with optional filtering
    List(ListIdeaArgs),
    /// Show full details of an idea
    Show(ShowIdeaArgs),
    /// Add or replace tags on an idea
    Tag(TagIdeaArgs),
    /// Update the status of an idea
    Status(StatusIdeaArgs),
    /// Edit an idea in $EDITOR
    Edit(EditIdeaArgs),
    /// Delete an idea with confirmation
    Delete(DeleteIdeaArgs),
    /// Update idea fields (title, description, status)
    Update(IdeaUpdateArgs),
}

#[derive(Args)]
pub struct NewIdeaArgs {
    /// The title of the idea
    title: String,

    /// Optional description for the idea
    #[arg(short = 'd', long = "description")]
    description: Option<String>,

    /// Optional tags (comma-separated)
    #[arg(short = 't', long = "tags", value_delimiter = ',')]
    tags: Vec<String>,
}

#[derive(Args)]
pub struct ListIdeaArgs {
    /// Filter by status (Brainstorming|Active|Completed|Archived)
    #[arg(short = 's', long = "status")]
    status: Option<IdeaStatus>,

    /// Filter by tag
    #[arg(short = 't', long = "tag")]
    tag: Option<String>,
}

#[derive(Args)]
pub struct ShowIdeaArgs {
    /// The UUID of the idea to show
    id: Uuid,
}

#[derive(Args)]
pub struct TagIdeaArgs {
    /// The UUID of the idea to tag
    id: Uuid,

    /// Tags to add/replace (space-separated)
    tags: Vec<String>,
}

#[derive(Args)]
pub struct StatusIdeaArgs {
    /// The UUID of the idea to update
    id: Uuid,

    /// New status for the idea
    status: IdeaStatus,
}

#[derive(Args)]
pub struct EditIdeaArgs {
    /// The UUID of the idea to edit
    id: Uuid,
}

#[derive(Args)]
pub struct DeleteIdeaArgs {
    /// The UUID of the idea to delete
    id: Uuid,

    /// Skip confirmation prompt
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
pub struct IdeaUpdateArgs {
    /// Idea ID to update
    pub id: Uuid,

    /// New title
    #[arg(short = 't', long = "title")]
    pub title: Option<String>,

    /// New description
    #[arg(short = 'd', long = "description")]
    pub description: Option<String>,

    /// New status
    #[arg(short = 's', long = "status")]
    pub status: Option<IdeaStatus>,

    /// Clear one or more optional fields (description)
    #[arg(long = "clear", value_name = "FIELD")]
    pub clear: Vec<String>,
}

impl IdeaCommands {
    pub fn execute(&self) -> Result<()> {
        let storage = Storage::new().context("Failed to initialize storage")?;

        match &self.command {
            IdeaSubcommand::New(args) => Self::new_idea(&storage, args),
            IdeaSubcommand::List(args) => Self::list_ideas(&storage, args),
            IdeaSubcommand::Show(args) => Self::show_idea(&storage, args),
            IdeaSubcommand::Tag(args) => Self::tag_idea(&storage, args),
            IdeaSubcommand::Status(args) => Self::update_status(&storage, args),
            IdeaSubcommand::Edit(args) => Self::edit_idea(&storage, args),
            IdeaSubcommand::Delete(args) => Self::delete_idea(&storage, args),
            IdeaSubcommand::Update(args) => Self::update_idea(&storage, args),
        }
    }

    fn new_idea(storage: &Storage, args: &NewIdeaArgs) -> Result<()> {
        let mut idea = Idea::new(args.title.clone());

        if let Some(description) = &args.description {
            idea = idea.with_description(description.clone());
        }

        if !args.tags.is_empty() {
            idea = idea.with_tags(args.tags.clone());
        }

        let mut ideas = storage.load_ideas().context("Failed to load ideas")?;
        ideas.push(idea.clone());
        storage.save_ideas(&ideas).context("Failed to save ideas")?;

        println!("✅ Created new idea:");
        print_idea_summary(&idea);
        Ok(())
    }

    fn list_ideas(storage: &Storage, args: &ListIdeaArgs) -> Result<()> {
        let mut ideas = storage.load_ideas().context("Failed to load ideas")?;

        // Apply filters
        if let Some(status_filter) = &args.status {
            ideas.retain(|idea| &idea.status == status_filter);
        }

        if let Some(tag_filter) = &args.tag {
            ideas.retain(|idea| idea.tags.contains(tag_filter));
        }

        if ideas.is_empty() {
            println!("📝 No ideas found");
            return Ok(());
        }

        println!("📝 Found {} idea(s):", ideas.len());
        println!();

        for idea in &ideas {
            print_idea_summary(idea);
            println!();
        }

        Ok(())
    }

    fn show_idea(storage: &Storage, args: &ShowIdeaArgs) -> Result<()> {
        let ideas = storage.load_ideas().context("Failed to load ideas")?;

        let idea = ideas
            .iter()
            .find(|idea| idea.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Idea with ID {} not found", args.id))?;

        print_idea_full(idea);
        Ok(())
    }

    fn tag_idea(storage: &Storage, args: &TagIdeaArgs) -> Result<()> {
        let mut ideas = storage.load_ideas().context("Failed to load ideas")?;

        let idea_index = ideas
            .iter()
            .position(|idea| idea.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Idea with ID {} not found", args.id))?;

        // Replace all tags with the new ones
        ideas[idea_index].tags.clear();
        for tag in &args.tags {
            if !ideas[idea_index].tags.contains(tag) {
                ideas[idea_index].tags.push(tag.clone());
            }
        }
        ideas[idea_index].updated_at = chrono::Utc::now();

        storage.save_ideas(&ideas).context("Failed to save ideas")?;

        println!("✅ Updated tags for idea {}:", args.id);
        println!("   Tags: {}", ideas[idea_index].tags.join(", "));
        Ok(())
    }

    fn update_status(storage: &Storage, args: &StatusIdeaArgs) -> Result<()> {
        let mut ideas = storage.load_ideas().context("Failed to load ideas")?;

        let idea = ideas
            .iter_mut()
            .find(|idea| idea.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Idea with ID {} not found", args.id))?;

        let old_status = idea.status.clone();
        idea.set_status(args.status.clone());

        storage.save_ideas(&ideas).context("Failed to save ideas")?;

        println!("✅ Updated status for idea {}:", args.id);
        println!("   {} → {}", old_status, args.status);
        Ok(())
    }

    fn edit_idea(storage: &Storage, args: &EditIdeaArgs) -> Result<()> {
        let mut ideas = storage.load_ideas().context("Failed to load ideas")?;

        let idea_index = ideas
            .iter()
            .position(|idea| idea.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Idea with ID {} not found", args.id))?;

        // Create temporary file with current content
        let temp_file = format!("{}.md", args.id);
        let content = format!(
            "# {}\n\n{}\n\nTags: {}\n\nStatus: {}\n\n",
            ideas[idea_index].title,
            ideas[idea_index].description.as_deref().unwrap_or(""),
            ideas[idea_index].tags.join(", "),
            ideas[idea_index].status
        );

        std::fs::write(&temp_file, content).context("Failed to create temp file")?;

        // Open editor
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        let status = Command::new(&editor)
            .arg(&temp_file)
            .status()
            .context("Failed to open editor")?;

        if !status.success() {
            std::fs::remove_file(&temp_file)?;
            return Err(anyhow::anyhow!("Editor exited with non-zero status"));
        }

        // Read updated content
        let updated_content =
            std::fs::read_to_string(&temp_file).context("Failed to read updated content")?;
        std::fs::remove_file(&temp_file)?;

        // Parse updated content (simple parsing for demo)
        let lines: Vec<&str> = updated_content.lines().collect();
        if let Some(title_line) = lines.iter().find(|line| line.starts_with("# ")) {
            ideas[idea_index].title = title_line.strip_prefix("# ").unwrap().trim().to_string();
        }

        // Find description (content between title and tags/status)
        let mut description_parts = Vec::new();
        let mut in_description = false;

        for line in &lines {
            if line.starts_with("# ") {
                in_description = true;
                continue;
            } else if line.starts_with("Tags:") || line.starts_with("Status:") {
                in_description = false;
                continue;
            } else if in_description && !line.trim().is_empty() {
                description_parts.push(line.trim());
            }
        }

        if !description_parts.is_empty() {
            ideas[idea_index].description = Some(description_parts.join("\n"));
        }

        // Parse tags and status from the end of file
        for line in lines.iter().rev() {
            if line.starts_with("Tags:") {
                let tags_str = line.strip_prefix("Tags:").unwrap().trim();
                ideas[idea_index].tags.clear();
                if !tags_str.is_empty() {
                    ideas[idea_index].tags =
                        tags_str.split(',').map(|t| t.trim().to_string()).collect();
                }
            } else if line.starts_with("Status:") {
                let status_str = line.strip_prefix("Status:").unwrap().trim();
                if let Ok(parsed_status) = status_str.parse() {
                    ideas[idea_index].set_status(parsed_status);
                }
            }
        }

        storage.save_ideas(&ideas).context("Failed to save ideas")?;

        println!("✅ Updated idea {}:", args.id);
        print_idea_summary(&ideas[idea_index]);
        Ok(())
    }

    fn delete_idea(storage: &Storage, args: &DeleteIdeaArgs) -> Result<()> {
        let mut ideas = storage.load_ideas().context("Failed to load ideas")?;

        let idea_index = ideas
            .iter()
            .position(|idea| idea.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Idea with ID {} not found", args.id))?;

        let idea = &ideas[idea_index];

        if !args.force {
            print!(
                "Are you sure you want to delete the idea '{}'? [y/N]: ",
                idea.title
            );
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

        let deleted_idea = ideas.remove(idea_index);
        storage.save_ideas(&ideas).context("Failed to save ideas")?;

        println!("✅ Deleted idea: {}", deleted_idea.title);
        Ok(())
    }

    pub fn update_idea(storage: &Storage, args: &IdeaUpdateArgs) -> Result<()> {
        const CLEARABLE_FIELDS: [&str; 1] = ["description"];

        // Validate clear fields
        for field in &args.clear {
            if !CLEARABLE_FIELDS.contains(&field.as_str()) {
                anyhow::bail!(
                    "Cannot clear '{}'. Valid fields: {}",
                    field,
                    CLEARABLE_FIELDS.join(", ")
                );
            }
        }

        let mut ideas = storage.load_ideas().context("Failed to load ideas")?;

        let idea = ideas
            .iter_mut()
            .find(|i| i.id == args.id)
            .ok_or_else(|| anyhow::anyhow!("Idea with ID {} not found", args.id))?;

        let mut changes: Vec<String> = Vec::new();

        // Update title
        if let Some(title) = &args.title {
            let old = idea.title.clone();
            idea.update_title(title.clone());
            changes.push(format!("title: \"{}\" → \"{}\"", old, title));
        }

        // Update description
        if let Some(desc) = &args.description {
            let old = idea.description.clone().unwrap_or_default();
            idea.update_description(Some(desc.clone()));
            changes.push(format!("description: \"{}\" → \"{}\"", old, desc));
        }

        // Update status
        if let Some(status) = &args.status {
            let old = idea.status.clone();
            idea.set_status(status.clone());
            changes.push(format!("status: {} → {}", old, status));
        }

        // Clear fields
        for field in &args.clear {
            match field.as_str() {
                "description" => {
                    idea.update_description(None);
                    changes.push("description: cleared".to_string());
                }
                _ => unreachable!(),
            }
        }

        if changes.is_empty() {
            println!("No changes specified for idea {}", args.id);
            println!("Use --help to see available options.");
            return Ok(());
        }

        storage.save_ideas(&ideas).context("Failed to save ideas")?;

        println!("✅ Updated idea {}:", args.id);
        for change in &changes {
            println!("   {}", change);
        }

        Ok(())
    }
}

fn print_idea_summary(idea: &Idea) {
    let status_emoji = match idea.status {
        IdeaStatus::Brainstorming => "🧠",
        IdeaStatus::Active => "🚀",
        IdeaStatus::Completed => "✅",
        IdeaStatus::Archived => "📦",
    };

    println!("{} {} [{}]", status_emoji, idea.title, idea.id);
    if let Some(description) = &idea.description {
        let desc_preview = if description.len() > 50 {
            format!("{}...", &description[..50])
        } else {
            description.clone()
        };
        println!("   {}", desc_preview);
    }
    if !idea.tags.is_empty() {
        println!("   🏷️  {}", idea.tags.join(", "));
    }
    println!(
        "   📅 Updated: {}",
        idea.updated_at.format("%Y-%m-%d %H:%M")
    );
}

fn print_idea_full(idea: &Idea) {
    let status_emoji = match idea.status {
        IdeaStatus::Brainstorming => "🧠",
        IdeaStatus::Active => "🚀",
        IdeaStatus::Completed => "✅",
        IdeaStatus::Archived => "📦",
    };

    println!("{} {}", status_emoji, idea.title);
    println!("ID: {}", idea.id);
    println!("Status: {}", idea.status);

    if !idea.tags.is_empty() {
        println!("Tags: {}", idea.tags.join(", "));
    }

    println!(
        "Created: {}",
        idea.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "Updated: {}",
        idea.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!();

    if let Some(description) = &idea.description {
        println!("Description:");
        println!("{}", description);
    } else {
        println!("No description");
    }
}

// Implement FromStr for IdeaStatus for CLI parsing
impl std::str::FromStr for IdeaStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "brainstorming" => Ok(IdeaStatus::Brainstorming),
            "active" => Ok(IdeaStatus::Active),
            "completed" => Ok(IdeaStatus::Completed),
            "archived" => Ok(IdeaStatus::Archived),
            _ => Err(anyhow::anyhow!(
                "Invalid status. Must be one of: Brainstorming, Active, Completed, Archived"
            )),
        }
    }
}

impl std::fmt::Display for IdeaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdeaStatus::Brainstorming => write!(f, "Brainstorming"),
            IdeaStatus::Active => write!(f, "Active"),
            IdeaStatus::Completed => write!(f, "Completed"),
            IdeaStatus::Archived => write!(f, "Archived"),
        }
    }
}
