use anyhow::Result;
use clap::Parser;
use ideavault::cli::Cli;
use ideavault::commands::execute_search;
use std::env;

fn main() -> Result<()> {
    // Check for markdown-help flag before parsing
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--markdown-help".to_string()) {
        clap_markdown::print_help_markdown::<Cli>();
        return Ok(());
    }

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
        ideavault::cli::Commands::Version(version_args) => {
            ideavault::commands::version::execute(version_args)?;
        }
    }

    Ok(())
}
