use anyhow::Result;
use clap::Parser;
use ideavault::cli::Cli;
use ideavault::commands::execute_search;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        ideavault::cli::Commands::Idea(idea_cmd) => {
            idea_cmd.execute()?;
        }
        ideavault::cli::Commands::Project(project_cmd) => {
            project_cmd.execute()?;
        }
        ideavault::cli::Commands::Task(task_cmd) => {
            task_cmd.execute()?;
        }
        ideavault::cli::Commands::Search(search_args) => {
            execute_search(search_args)?;
        }
    }

    Ok(())
}
