# IdeaVault

[![Version](https://img.shields.io/github/v/release/bigknoxy/ideavault)](https://github.com/bigknoxy/ideavault/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/bigknoxy/ideavault/actions)

A CLI tool for managing ideas, projects, and tasks in one place.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/bigknoxy/ideavault/main/install.sh | bash
```

## Quick Start

Create a new idea:
```bash
ideavault idea new "Mobile app for pet tracking" -d "GPS tracker for lost pets" -t "mobile app"
```

Create a new project:
```bash
ideavault project new "PetTracker v1" -d "Initial MVP development"
```

Create a task with priority and due date:
```bash
ideavault task new "Design database schema" --project PetTracker --priority high --due 2025-03-01
```

Link a task to a project:
```bash
ideavault project link PetTracker "Design database schema"
```

## Full Documentation

- [CLI Reference](docs/CommandLineHelp.md) - Complete command documentation
- [Usage Guide](docs/USAGE.md) - Workflow examples and best practices
- [Changelog](CHANGELOG.md) - Release history

## Uninstall

To remove IdeaVault and all data:

```bash
curl -fsSL https://raw.githubusercontent.com/bigknoxy/ideavault/main/uninstall.sh | bash
```

See [uninstall.sh](uninstall.sh) for options like `--keep-data` to preserve your files.

## License

MIT
