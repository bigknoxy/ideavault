use anyhow::Result;
use clap::Args;
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_API_URL: &str = "https://api.github.com/repos/bigknoxy/ideavault/releases/latest";

#[derive(Debug, Args)]
pub struct VersionArgs {
    /// Check for updates
    #[arg(short, long)]
    pub check: bool,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
}

pub fn execute(args: VersionArgs) -> Result<()> {
    println!("IdeaVault v{VERSION}");

    if args.check {
        println!("Checking for updates...");
        match check_latest_version() {
            Ok(latest) => {
                let latest_version = latest.trim_start_matches('v');
                if latest_version != VERSION {
                    println!("Latest version: v{latest_version}");
                    println!("Update available! Run: curl -fsSL https://raw.githubusercontent.com/bigknoxy/ideavault/main/install.sh | bash");
                } else {
                    println!("You're running the latest version.");
                }
            }
            Err(e) => {
                println!("Warning: Could not check for updates: {e}");
            }
        }
    }

    Ok(())
}

fn check_latest_version() -> Result<String> {
    let response = ureq::get(GITHUB_API_URL)
        .set("User-Agent", &format!("IdeaVault/{VERSION}"))
        .call()?
        .into_json::<Release>()?;
    Ok(response.tag_name)
}
