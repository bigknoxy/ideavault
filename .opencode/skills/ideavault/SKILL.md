---
name: ideavault
description: IdeaVault Rust CLI project conventions, build commands, code style, and git workflow
license: MIT
compatibility: opencode
metadata:
  project: ideavault
  language: rust
  framework: clap
---

## Project Overview

- **Name**: IdeaVault
- **Type**: Rust CLI Tool
- **Description**: A CLI tool for managing ideas, projects, and tasks
- **Tech Stack**: Rust 2021, clap 4.4, serde, anyhow, chrono, uuid

---

## Build & Test Commands

```bash
# Build
cargo build --release

# Run all tests
cargo test

# Run a single test (by test name)
cargo test test_name

# Run a single test with exact match
cargo test --exact test_name

# Run tests in a specific file
cargo test --test storage_test

# Run tests matching a pattern
cargo test pattern

# Format code
cargo fmt

# Lint (treat warnings as errors)
cargo clippy -- -D warnings

# Run locally
cargo run -- [args]
```

---

## Git Workflow (MANDATORY)

### All Changes to Main via PR
- **NEVER push directly to main**
- Create feature branch for every change
- Open PR, get review, merge
- Only push tags directly (for releases)

### Branch Naming
- `feature/feature-name`
- `fix/bug-description`
- `docs/documentation-topic`

### Commit Messages
- Use conventional commits: `feat:`, `fix:`, `docs:`, `chore:`
- Keep commits atomic
- Reference issue numbers in PR description

---

## Code Style Guidelines

### Imports Ordering
Group imports in this order, separated by blank lines:
1. External crates (alphabetically)
2. Internal crate modules (`use crate::...`)

```rust
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Idea, Priority, Status};
use crate::storage::Storage;
```

### Naming Conventions
- **Types/Structs/Enums**: PascalCase (e.g., `Idea`, `TaskStatus`)
- **Functions/Methods**: snake_case (e.g., `create_idea`, `get_by_id`)
- **Variables**: snake_case (e.g., `project_id`, `created_at`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `MAX_TITLE_LENGTH`)
- **Module names**: snake_case (e.g., `storage`, `task_commands`)

### Struct Derives
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idea {
    // fields...
}
```

### Error Handling
Use anyhow throughout:
```rust
// Use .context() for error messages
storage.load_ideas().context("Failed to load ideas")?;

// Use anyhow! for custom errors
anyhow::bail!("Idea with ID {} not found", id);

// Convert Option to Result
let idea = ideas.iter().find(|i| i.id == id)
    .ok_or_else(|| anyhow::anyhow!("Idea not found"))?;
```

### Builder Pattern
```rust
impl Idea {
    pub fn new(title: String) -> Self {
        Self { id: Uuid::new_v4(), title, /* defaults */ }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

// Usage:
let idea = Idea::new(title).with_description(desc);
```

### Mutable Update Pattern
Always update `updated_at` when modifying:
```rust
pub fn set_status(&mut self, status: Status) {
    self.status = status;
    self.updated_at = Utc::now();
}
```

### Command Structure
```rust
#[derive(Debug, Args)]
pub struct IdeaCommands {
    #[command(subcommand)]
    pub command: IdeaSubcommand,
}

impl IdeaCommands {
    pub fn execute(&self, storage: &Storage) -> Result<()> {
        self.command.execute(storage)
    }
}
```

### Tests
- Unit tests: in `src/lib.rs` under `#[cfg(test)] mod tests`
- Integration tests: in `tests/` directory
- Use `?` operator in test functions returning `Result<()>`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() -> Result<()> {
        // test code
        Ok(())
    }
}
```

---

## Key Files

| File | Purpose |
|------|---------|
| src/cli.rs | CLI argument definitions |
| src/commands/*.rs | Command implementations |
| src/models/*.rs | Data models |
| src/storage.rs | JSON file persistence |
| install.sh | One-liner install script |
| .github/workflows/*.yml | CI/CD pipelines |

---

## Useful Paths

- Data: `~/.local/share/ideavault/`
- Binary: `/usr/local/bin/ideavault` (after install)
- Config: Uses XDG base directories

---

## When to Use This Skill

Use this skill when:
- Working on the IdeaVault Rust CLI project
- Need build/test/lint commands
- Creating or reviewing code for style compliance
- Setting up PRs

For releases, use the `ideavault-release` skill.
