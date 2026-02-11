use crate::commands::{IdeaCommands, ProjectCommands};
use clap::{Args, Parser};

#[derive(Parser)]
#[command(name = "ideavault")]
#[command(about = "A CLI tool for managing ideas and projects")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    /// Manage ideas
    Idea(IdeaCommands),
    /// Manage projects
    Project(ProjectCommands),
    /// Search across ideas, projects, and tags
    Search(SearchArgs),
}

#[derive(Args)]
pub struct SearchArgs {
    /// Search query string
    #[arg(required = true)]
    pub query: String,

    /// Search in ideas only
    #[arg(short = 'i', long = "ideas")]
    pub ideas: bool,

    /// Search in projects only  
    #[arg(short = 'p', long = "projects")]
    pub projects: bool,

    /// Search in tags only
    #[arg(short = 't', long = "tags")]
    pub tags: bool,

    /// Filter by status
    #[arg(short = 's', long = "status")]
    pub status: Option<String>,

    /// Filter by tags (space-separated, multiple allowed)
    #[arg(long = "with-tags", value_delimiter = ' ')]
    pub with_tags: Vec<String>,

    /// Filter by date from (YYYY-MM-DD format)
    #[arg(long = "from")]
    pub date_from: Option<String>,

    /// Filter by date to (YYYY-MM-DD format)
    #[arg(long = "to")]
    pub date_to: Option<String>,
}
