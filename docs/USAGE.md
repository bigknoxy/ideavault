# 🚀 IdeaVault Usage Guide

Welcome to **IdeaVault** — your command-line companion for capturing ideas, managing projects, and staying organized.

---

## 📋 Getting Started

### First-Time Setup

Before you start, make sure IdeaVault is installed and ready to go:

```bash
# Verify installation
ideavault --help

# Set up your preferred editor (optional but recommended)
export EDITOR="vim"        # or nano, code, emacs, etc.
```

💡 **Pro tip:** Add the `EDITOR` export to your shell config file (`~/.bashrc`, `~/.zshrc`, etc.) so it's always available.

### Creating Your First Idea

Let's create your first idea:

```bash
# Create a simple idea
ideavault idea create "Build a personal knowledge base"

# Create an idea with a description
ideavault idea create "Learn Rust programming" --description "Focus on async and web development"

# Create and tag it immediately
ideavault idea create "Design a smart garden system" --tags "iot,hardware,garden"
```

### Basic Concepts

Understanding these three core concepts is key to using IdeaVault effectively:

| Concept | Description | Use Case |
|---------|-------------|----------|
| **💡 Ideas** | Fleeting thoughts, concepts, or inspiration | Capture things that excite you before you forget |
| **📁 Projects** | Organized efforts with goals and milestones | Turn ideas into actionable work |
| **✅ Tasks** | Concrete, actionable items | The smallest unit of work to move projects forward |

**Flow:** Ideas → Projects → Tasks

---

## 🔄 Workflow Examples

### Pattern 1: Idea Development

Capture raw inspiration and refine it into actionable work:

```bash
# 1. Brainstorm freely
ideavault idea create "Write a blog post about productivity" --tags "writing,productivity"

# 2. Refine with tags
ideavault idea update 1 --tags "writing,productivity,blog,2024"

# 3. When ready to execute, mark Active
ideavault idea update 1 --status Active

# 4. Convert to a project when it grows
ideavault project create "Productivity Blog Series" --description "Weekly posts on productivity techniques"

# 5. Link the idea to the project
ideavault idea update 1 --project-id 1
```

**When to use:** Capturing inspiration, incubating concepts, deciding what to work on.

---

### Pattern 2: Project Management

Structure complex work with milestones and linked ideas:

```bash
# 1. Create a project
ideavault project create "Launch Personal Website" --description "Portfolio and blog site"

# 2. Add a milestone
ideavault milestone create 1 "Design Phase" --target-date 2024-02-28

# 3. Link relevant ideas
ideavault idea update 5 --project-id 1
ideavault idea update 12 --project-id 1

# 4. Break down into tasks
ideavault task create 1 "Choose domain name" --priority high --due 2024-02-15
ideavault task create 1 "Set up hosting" --priority high --due 2024-02-17
ideavault task create 1 "Design homepage mockup" --priority medium --due 2024-02-20

# 5. Track progress
ideavault project status 1
ideavault task list --project 1
```

**When to use:** Multi-step initiatives, team coordination, deadline-driven work.

---

### Pattern 3: GTD-Style Task Management

Apply Getting Things Done principles to your daily workflow:

#### Daily Review Workflow

```bash
# Morning review - see what's due today
ideavault task list --due today

# Check blocked items
ideavault task list --status blocked

# Review high priority items
ideavault task list --priority urgent,high
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
ideavault task create 1 "Fix critical bug" --priority urgent
ideavault task create 1 "Update documentation" --priority high
ideavault task create 1 "Refactor old code" --priority medium
ideavault task create 1 "Research new tools" --priority low

# Change priority as needed
ideavault task update 5 --priority urgent
```

#### Status Workflow

```bash
# Start working on a task
ideavault task update 5 --status inprogress

# Mark complete
ideavault task update 5 --status done

# Mark blocked (add a note why)
ideavault task update 5 --status blocked --description "Waiting for API access"

# Daily sweep - what's done?
ideavault task list --status done --since yesterday
```

**When to use:** Daily productivity, context switching, prioritization.

---

### Pattern 4: Linking Everything Together

See the full power of interconnected workflows:

```bash
# 1. Capture an idea during brainstorming
ideavault idea create "Create a CLI tool for notes" \
  --tags "programming,cli,golang" \
  --description "Could use SQLite for storage"

# 2. When committed, mark Active and create project
ideavault idea update 42 --status Active
ideavault project create "IdeaVault CLI" \
  --description "Note-taking tool with projects and tasks"

# 3. Link the idea to the project
ideavault idea update 42 --project-id 7

# 4. Add tasks to the project
ideavault task create 7 "Design database schema" --priority high
ideavault task create 7 "Implement idea commands" --priority high
ideavault task create 7 "Write documentation" --priority medium

# 5. View everything together
ideavault project show 7          # See project + linked ideas + tasks
ideavault idea show 42             # See idea + linked project
```

**When to use:** Complex initiatives, long-term planning, maintaining context.

---

## 📊 Status Reference Table

Here's the complete reference for all valid statuses:

| Entity | Valid Statuses | Description |
|--------|----------------|-------------|
| **💡 Ideas** | `Brainstorming` | Initial capture, not yet committed |
| | `Active` | Currently being developed |
| | `Completed` | Idea implemented or done |
| | `Archived` | No longer relevant, kept for reference |
| **📁 Projects** | `Planning` | In design/requirements phase |
| | `InProgress` | Actively being worked on |
| | `Completed` | All milestones achieved |
| | `OnHold` | Paused, may resume later |
| **✅ Tasks** | `todo` | Ready to be worked on |
| | `inprogress` | Currently being worked on |
| | `blocked` | Cannot proceed (external dependency) |
| | `done` | Completed |
| | `cancelled` | No longer needed |
| **Task Priorities** | `urgent` | Drop everything and do this |
| | `high` | Important, do soon |
| | `medium` | Normal priority |
| | `low` | Do when convenient |

---

## 💡 Tips & Tricks

### Tagging Strategies

**Hierarchical Tags:**
```bash
# Use dot notation for categories
ideavault idea create "New idea" --tags "work.project-a,tech.backend"
ideavault idea create "Another idea" --tags "work.project-b,tech.frontend"

# Search by prefix
ideavault idea list --tag "work."    # All work items
```

**Status + Context Tags:**
```bash
# Combine status and context
ideavault task create 1 "Call dentist" --tags "@phone,health,urgent"
ideavault task create 1 "Buy groceries" --tags "@errand,home"
```

**Date-Based Tags:**
```bash
# Tag with timeframes
ideavault idea create "Plan vacation" --tags "2024,q2,travel"
# Makes it easy to find: ideavault idea list --tag 2024
```

### Effective Searching

```bash
# Search titles and descriptions
ideavault idea search "blog"

# Filter by multiple criteria
ideavault task list --project 1 --status todo --priority high

# Find abandoned ideas
ideavault idea list --status Brainstorming --older-than 30d

# Recent activity
ideavault idea list --since 2024-02-01
```

### Using Due Dates Effectively

```bash
# Set specific dates
ideavault task create 1 "Submit report" --due 2024-02-15

# Use relative dates
ideavault task create 1 "Weekly review" --due friday
ideavault task create 1 "Tomorrow's task" --due tomorrow

# Find overdue items
ideavault task list --overdue

# Upcoming deadlines
ideavault task list --due-this-week
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

💡 **Important:** For VS Code and similar GUI editors, use the `--wait` flag so IdeaVault waits for you to close the file before continuing.

---

## 📝 Common Commands Reference

### Ideas

| Command | Description |
|---------|-------------|
| `ideavault idea create "title"` | Create a new idea |
| `ideavault idea list` | List all ideas |
| `ideavault idea show <id>` | Show idea details |
| `ideavault idea update <id> --status Active` | Update idea status |
| `ideavault idea search "keyword"` | Search ideas |
| `ideavault idea delete <id>` | Delete an idea |

### Projects

| Command | Description |
|---------|-------------|
| `ideavault project create "name"` | Create a new project |
| `ideavault project list` | List all projects |
| `ideavault project show <id>` | Show project with ideas & tasks |
| `ideavault project update <id> --status InProgress` | Update project |
| `ideavault project status <id>` | Get project progress summary |
| `ideavault project delete <id>` | Delete a project |

### Tasks

| Command | Description |
|---------|-------------|
| `ideavault task create <project-id> "title"` | Create a task |
| `ideavault task list` | List all tasks |
| `ideavault task list --project <id>` | List project tasks |
| `ideavault task update <id> --status done` | Update task status |
| `ideavault task delete <id>` | Delete a task |

### Milestones

| Command | Description |
|---------|-------------|
| `ideavault milestone create <project-id> "name"` | Add milestone |
| `ideavault milestone list <project-id>` | List milestones |
| `ideavault milestone complete <id>` | Mark milestone complete |

---

## 🎯 Quick Start Checklist

- [ ] Set your `EDITOR` environment variable
- [ ] Create your first idea with `ideavault idea create`
- [ ] Add tags to organize your ideas
- [ ] Create a project to work on
- [ ] Link an idea to your project
- [ ] Create a few tasks
- [ ] Set a task to `inprogress` when you start working
- [ ] Mark a task `done` when complete
- [ ] Try the GTD-style daily review

---

**Happy organizing!** 🎉

For more details on specific commands, run `ideavault <command> --help`.
