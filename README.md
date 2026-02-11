# IdeaVault

A CLI tool for managing ideas and projects.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/bigknoxy/ideavault/main/install.sh | bash
```

## Usage

### Ideas
```bash
ideavault idea new "My startup idea" -d "Description" -t "startup rust"
ideavault idea list
ideavault idea show <id>
ideavault idea tag <id> <tags>
```

### Projects
```bash
ideavault project new "My Project"
ideavault project link <project-id> <idea-id>
ideavault project ideas <project-id>
```

### Search
```bash
ideavault search "startup"
ideavault search "rust" --ideas
ideavault search "cli" --with-tags rust
```

## Building from Source

```bash
git clone https://github.com/bigknoxy/ideavault.git
cd ideavault
cargo build --release
./target/release/ideavault --help
```

## Uninstallation

To uninstall IdeaVault:

```bash
curl -fsSL https://raw.githubusercontent.com/bigknoxy/ideavault/main/uninstall.sh | bash
```

### Options

- `--dry-run` Preview what will be removed without actually removing anything
- `--keep-data` Keep the data directory (~/.local/share/ideavault/)
- `--force` Skip confirmation prompts

### What gets removed

- Binary: `/usr/local/bin/ideavault`
- Data: `~/.local/share/ideavault/`
- Cache: `~/.cache/ideavault/`

## License

MIT
