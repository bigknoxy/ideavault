# IdeaVault - AI Agent Guidelines

Guidelines for AI coding agents working on the IdeaVault project.

---

## Project Overview

- **Name**: IdeaVault
- **Type**: Rust CLI Tool
- **Version**: 0.2.0 (see Cargo.toml)
- **Description**: A CLI tool for managing ideas, projects, and tasks

---

## Technology Stack

- **Language**: Rust 2021 edition
- **CLI Framework**: clap 4.4 (derive macros)
- **Serialization**: serde + serde_json
- **UUID**: uuid v4
- **DateTime**: chrono
- **Paths**: directories crate
- **Error Handling**: anyhow + thiserror

---

## Build & Test Commands

```bash
# Build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
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

## Release Process

1. Merge all changes to main via PR
2. Update version in Cargo.toml
3. Push tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
4. GitHub Actions automatically:
   - Builds binary (musl static)
   - Generates docs/CommandLineHelp.md
   - Creates GitHub release

---

## Documentation

- **README.md**: Quick start + essential examples only
- **docs/USAGE.md**: Comprehensive workflow guide
- **docs/CommandLineHelp.md**: Auto-generated via `cargo run -- --markdown-help`
- **CHANGELOG.md**: Release history

### Updating CLI Docs
```bash
cargo run --release -- --markdown-help > docs/CommandLineHelp.md
```

---

## Code Conventions

1. Follow existing patterns in src/commands/ and src/models/
2. Use anyhow for error handling
3. Serialize with serde_derive
4. Add unit tests for new functionality
5. Run `cargo fmt` before committing

---

## Verification Checklist

Before marking complete:
- [ ] `cargo build --release` succeeds
- [ ] `cargo test` passes
- [ ] `cargo fmt` applied
- [ ] `cargo clippy` clean
- [ ] PR created and merged
- [ ] Tag pushed for release
- [ ] Release verified on GitHub
- [ ] Install command tested: `curl -fsSL https://raw.githubusercontent.com/bigknoxy/ideavault/main/install.sh | bash`

---

## Key Files

| File | Purpose |
|------|---------|
| src/cli.rs | CLI argument definitions |
| src/commands/*.rs | Command implementations |
| src/models/*.rs | Data models |
| src/storage.rs | JSON file persistence |
| install.sh | One-liner install script |
| uninstall.sh | Uninstall script |
| .github/workflows/*.yml | CI/CD pipelines |

---

## Useful Paths

- Data: `~/.local/share/ideavault/`
- Binary: `/usr/local/bin/ideavault` (after install)
- Config: Uses XDG base directories

---

## Lessons Learned

See tasks/lessons.md for recorded mistakes and prevention rules.
