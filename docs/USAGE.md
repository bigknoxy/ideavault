# IdeaVault Usage Guide

Welcome to **IdeaVault** — your command-line companion for capturing ideas, managing projects, and staying organized.

---

## Getting Started

### First-Time Setup

Before you start, make sure IdeaVault is installed and ready to go:

```bash
# Verify installation
ideavault --help

# Set up your preferred editor (optional but recommended)
export EDITOR="vim"        # or nano, code, emacs, etc.
```

**Pro tip:** Add the `EDITOR` export to your shell config file (`~/.bashrc`, `~/.zshrc`, etc.) so it's always available.

### Creating Your First Idea

Let's create your first idea:

```bash
# Create a simple idea
ideavault idea new "Build a personal knowledge base"

# Create an idea with a description
ideavault idea new "Learn Rust programming" --description "Focus on async and web development"

# Create and tag it immediately
ideavault idea new "Design a smart garden system" --tags "iot,hardware,garden"
```

### Basic Concepts

Understanding these three core concepts is key to using IdeaVault effectively:

| Concept | Description | Use Case |
|---------|-------------|----------|
| **Ideas** | Fleeting thoughts, concepts, or inspiration | Capture things that excite you before you forget |
| **Projects** | Organized efforts with goals and milestones | Turn ideas into actionable work |
| **Tasks** | Concrete, actionable items | The smallest unit of work to move projects forward |

**Flow:** Ideas → Projects → Tasks

---

## Workflow Examples

### Pattern 1: Idea Development

Capture raw inspiration and refine it into actionable work:

```bash
# 1. Brainstorm freely
ideavault idea new "Write a blog post about productivity" --tags "writing,productivity"

# 2. Refine with tags
ideavault idea tag <id> writing productivity blog 2024

# 3. When ready to execute, mark Active
ideavault idea status <id> Active

# 4. Convert to a project when it grows
ideavault project new "Productivity Blog Series" --description "Weekly posts on productivity techniques"

# 5. Link the idea to the project
ideavault project link <project-id> <idea-id>
```

**When to use:** Capturing inspiration, incubating concepts, deciding what to work on.

---

### Pattern 2: Project Management

Structure complex work with milestones and linked ideas:

```bash
# 1. Create a project with URL and repo
ideavault project new "Launch Personal Website" \
  --description "Portfolio and blog site" \
  --url "https://mywebsite.com" \
  --repo "https://github.com/user/website"

# 2. Set a milestone
ideavault project update <project-id> --milestone "Design Phase"

# 3. Link relevant ideas
ideavault project link <project-id> <idea-id-1>
ideavault project link <project-id> <idea-id-2>

# 4. Break down into tasks
ideavault task new "Choose domain name" --project <project-id> --priority high
ideavault task new "Set up hosting" --project <project-id> --priority high
ideavault task new "Design homepage mockup" --project <project-id> --priority medium

# 5. Track progress
ideavault project show <project-id>
ideavault task list --project <project-id>
```

**When to use:** Multi-step initiatives, team coordination, deadline-driven work.

---

### Pattern 3: GTD-Style Task Management

Apply Getting Things Done principles to your daily workflow:

#### Daily Review Workflow

```bash
# Morning review - check blocked items
ideavault task list --status blocked

# Review high priority items
ideavault task list --priority high

# Check overdue tasks
ideavault task list --overdue
```

#### Context-Based Filtering

Use tags as contexts for where/when you can do work:

```bash
# @computer - tasks you can only do at your desk
ideavault task list --tag @computer

# @phone - calls and quick tasks
ideavault task list --tag @phone

# @home - household chores
ideavault task list --tag @home

# @errand - things to do while out
ideavault task list --tag @errand
```

#### Priority Management

```bash
# Create with priority
ideavault task new "Fix critical bug" --priority urgent
ideavault task new "Update documentation" --priority high
ideavault task new "Refactor old code" --priority medium
ideavault task new "Research new tools" --priority low

# Change priority as needed
ideavault task priority <task-id> urgent
```

#### Status Workflow

```bash
# Start working on a task
ideavault task status <task-id> inprogress

# Mark complete
ideavault task status <task-id> done

# Mark blocked
ideavault task status <task-id> blocked
```

**When to use:** Daily productivity, context switching, prioritization.

---

### Pattern 4: Linking Everything Together

See the full power of interconnected workflows:

```bash
# 1. Capture an idea during brainstorming
ideavault idea new "Create a CLI tool for notes" \
  --tags "programming,cli,rust" \
  --description "Could use SQLite for storage"

# 2. When committed, mark Active
ideavault idea status <idea-id> Active

# 3. Create a project
ideavault project new "IdeaVault CLI" \
  --description "Note-taking tool with projects and tasks" \
  --repo "https://github.com/user/ideavault"

# 4. Link the idea to the project
ideavault project link <project-id> <idea-id>

# 5. Add tasks to the project
ideavault task new "Design database schema" --project <project-id> --priority high
ideavault task new "Implement idea commands" --project <project-id> --priority high
ideavault task new "Write documentation" --project <project-id> --priority medium

# 6. View everything together
ideavault project show <project-id>    # See project + linked ideas
ideavault idea show <idea-id>           # See idea details
```

**When to use:** Complex initiatives, long-term planning, maintaining context.

---

## Status Reference Table

Here's the complete reference for all valid statuses:

| Entity | Valid Statuses | Description |
|--------|----------------|-------------|
| **Ideas** | `Brainstorming` | Initial capture, not yet committed |
| | `Active` | Currently being developed |
| | `Completed` | Idea implemented or done |
| | `Archived` | No longer relevant, kept for reference |
| **Projects** | `Planning` | In design/requirements phase |
| | `InProgress` | Actively being worked on |
| | `Completed` | All milestones achieved |
| | `OnHold` | Paused, may resume later |
| **Tasks** | `todo` | Ready to be worked on |
| | `inprogress` | Currently being worked on |
| | `blocked` | Cannot proceed (external dependency) |
| | `done` | Completed |
| | `cancelled` | No longer needed |
| **Task Priorities** | `urgent` | Drop everything and do this |
| | `high` | Important, do soon |
| | `medium` | Normal priority |
| | `low` | Do when convenient |

---

## Tips & Tricks

### Tagging Strategies

**Hierarchical Tags:**
```bash
# Use dot notation for categories
ideavault idea new "New idea" --tags "work.project-a,tech.backend"
ideavault idea new "Another idea" --tags "work.project-b,tech.frontend"

# Search by prefix
ideavault idea list --tag work    # All work items
```

**Status + Context Tags:**
```bash
# Combine status and context
ideavault task new "Call dentist" --tags "@phone,health,urgent"
ideavault task new "Buy groceries" --tags "@errand,home"
```

**Date-Based Tags:**
```bash
# Tag with timeframes
ideavault idea new "Plan vacation" --tags "2024,q2,travel"
# Makes it easy to find: ideavault idea list --tag 2024
```

### Effective Searching

```bash
# Search across all entities
ideavault search "blog"

# Search only ideas
ideavault search "blog" --ideas

# Search only projects
ideavault search "website" --projects

# Search with status filter
ideavault search "api" --status Active

# Search with tag filter
ideavault search "rust" --with-tags programming
```

### Using Due Dates Effectively

```bash
# Set a due date when creating a task
ideavault task new "Submit report" --due 2024-02-15

# Set due date on existing task
ideavault task due <task-id> 2024-02-15

# Clear a due date
ideavault task due <task-id> clear

# Find overdue items
ideavault task list --overdue
```

### Editor Integration

When using commands that open an editor:

```bash
# Set your preferred editor
export EDITOR="vim"           # Terminal-based
export EDITOR="nano"          # Simple terminal editor
export EDITOR="code --wait"   # VS Code (wait flag is important!)
export EDITOR="emacs"         # Emacs
```

**Important:** For VS Code and similar GUI editors, use the `--wait` flag so IdeaVault waits for you to close the file before continuing.

---

## Common Commands Reference

### Ideas

| Command | Description |
|---------|-------------|
| `ideavault idea new "title"` | Create a new idea |
| `ideavault idea list` | List all ideas |
| `ideavault idea list --status Active` | List ideas by status |
| `ideavault idea list --tag <tag>` | List ideas by tag |
| `ideavault idea show <id>` | Show idea details |
| `ideavault idea status <id> <status>` | Update idea status |
| `ideavault idea tag <id> <tags...>` | Update idea tags |
| `ideavault idea edit <id>` | Edit idea in $EDITOR |
| `ideavault idea delete <id>` | Delete an idea |

### Projects

| Command | Description |
|---------|-------------|
| `ideavault project new "name"` | Create a new project |
| `ideavault project new "name" --url <url> --repo <repo>` | Create with URLs |
| `ideavault project list` | List all projects |
| `ideavault project show <id>` | Show project with linked ideas |
| `ideavault project update <id> [flags]` | Update project fields |
| `ideavault project status <id> <status>` | Quick status update |
| `ideavault project link <project-id> <idea-id>` | Link idea to project |
| `ideavault project unlink <project-id> <idea-id>` | Unlink idea from project |
| `ideavault project ideas <id>` | List linked ideas |
| `ideavault project delete <id>` | Delete a project |

#### Updating Projects

Update one or more fields of an existing project:

```bash
# Update title
ideavault project update <id> --title "New Title"

# Update multiple fields at once
ideavault project update <id> --title "New" --url "https://example.com" --milestone "v1.0"

# Clear optional fields
ideavault project update <id> --clear url --clear milestone

# Mix updates and clears
ideavault project update <id> --title "Updated" --clear description

# Update status
ideavault project update <id> --status InProgress
```

**Available flags:**
- `--title` - Project title
- `--description` - Project description
- `--milestone` - Current milestone
- `--url` - Project URL
- `--repo` - Repository URL
- `--status` - Project status
- `--clear <field>` - Clear an optional field (description, milestone, url, repo)

**Quick status update:**
```bash
ideavault project status <id> InProgress
```

### Tasks

| Command | Description |
|---------|-------------|
| `ideavault task new "title"` | Create a new task |
| `ideavault task new "title" --project <id>` | Create task linked to project |
| `ideavault task new "title" --priority high --due 2024-02-15` | Create with priority and due date |
| `ideavault task list` | List all tasks |
| `ideavault task list --project <id>` | List project tasks |
| `ideavault task list --status todo` | List tasks by status |
| `ideavault task list --priority high` | List tasks by priority |
| `ideavault task list --overdue` | List overdue tasks |
| `ideavault task show <id>` | Show task details |
| `ideavault task status <id> <status>` | Update task status |
| `ideavault task priority <id> <priority>` | Update task priority |
| `ideavault task due <id> <date>` | Set due date |
| `ideavault task update <id> [flags]` | Update task fields |
| `ideavault task link-project <task-id> <project-id>` | Link task to project |
| `ideavault task link-idea <task-id> <idea-id>` | Link task to idea |
| `ideavault task edit <id>` | Edit task in $EDITOR |
| `ideavault task delete <id>` | Delete a task |

#### Updating Tasks

Update one or more fields of an existing task:

```bash
# Update title
ideavault task update <id> --title "New Title"

# Update multiple fields at once
ideavault task update <id> --title "Updated" --priority high --status inprogress

# Update due date
ideavault task update <id> --due 2024-02-15

# Clear due date
ideavault task update <id> --due clear

# Clear optional fields
ideavault task update <id> --clear description --clear tags

# Update tags (replaces existing tags)
ideavault task update <id> --tags "work,urgent"

# Mix updates and clears
ideavault task update <id> --title "Updated" --clear due_date
```

**Available flags:**
- `--title` - Task title
- `--description` - Task description
- `--priority` - Task priority (low|medium|high|urgent)
- `--due` - Due date (YYYY-MM-DD format) or "clear" to remove
- `--status` - Task status (todo|inprogress|blocked|done|cancelled)
- `--tags` - Tags (comma-separated, replaces existing tags)
- `--clear <field>` - Clear an optional field (description, due_date, tags)

### Search

| Command | Description |
|---------|-------------|
| `ideavault search "query"` | Search across all entities |
| `ideavault search "query" --ideas` | Search only ideas |
| `ideavault search "query" --projects` | Search only projects |
| `ideavault search "query" --tags` | Search only tags |
| `ideavault search "query" --status Active` | Filter by status |
| `ideavault search "query" --with-tags tag1 tag2` | Filter by tags |

---

## Quick Start Checklist

- [ ] Set your `EDITOR` environment variable
- [ ] Create your first idea with `ideavault idea new`
- [ ] Add tags to organize your ideas
- [ ] Create a project to work on
- [ ] Link an idea to your project
- [ ] Create a few tasks
- [ ] Set a task to `inprogress` when you start working
- [ ] Mark a task `done` when complete
- [ ] Try the GTD-style daily review

---

**Happy organizing!**

For more details on specific commands, run `ideavault <command> --help`.
