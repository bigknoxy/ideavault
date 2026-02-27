# Command-Line Help for `ideavault`

This document contains the help content for the `ideavault` command-line program.

**Command Overview:**

* [`ideavault`↴](#ideavault)
* [`ideavault idea`↴](#ideavault-idea)
* [`ideavault idea new`↴](#ideavault-idea-new)
* [`ideavault idea list`↴](#ideavault-idea-list)
* [`ideavault idea show`↴](#ideavault-idea-show)
* [`ideavault idea tag`↴](#ideavault-idea-tag)
* [`ideavault idea status`↴](#ideavault-idea-status)
* [`ideavault idea edit`↴](#ideavault-idea-edit)
* [`ideavault idea delete`↴](#ideavault-idea-delete)
* [`ideavault idea update`↴](#ideavault-idea-update)
* [`ideavault project`↴](#ideavault-project)
* [`ideavault project new`↴](#ideavault-project-new)
* [`ideavault project list`↴](#ideavault-project-list)
* [`ideavault project show`↴](#ideavault-project-show)
* [`ideavault project link`↴](#ideavault-project-link)
* [`ideavault project unlink`↴](#ideavault-project-unlink)
* [`ideavault project ideas`↴](#ideavault-project-ideas)
* [`ideavault project status`↴](#ideavault-project-status)
* [`ideavault project delete`↴](#ideavault-project-delete)
* [`ideavault project update`↴](#ideavault-project-update)
* [`ideavault task`↴](#ideavault-task)
* [`ideavault task new`↴](#ideavault-task-new)
* [`ideavault task list`↴](#ideavault-task-list)
* [`ideavault task show`↴](#ideavault-task-show)
* [`ideavault task status`↴](#ideavault-task-status)
* [`ideavault task priority`↴](#ideavault-task-priority)
* [`ideavault task due`↴](#ideavault-task-due)
* [`ideavault task link-project`↴](#ideavault-task-link-project)
* [`ideavault task link-idea`↴](#ideavault-task-link-idea)
* [`ideavault task unlink-project`↴](#ideavault-task-unlink-project)
* [`ideavault task unlink-idea`↴](#ideavault-task-unlink-idea)
* [`ideavault task edit`↴](#ideavault-task-edit)
* [`ideavault task delete`↴](#ideavault-task-delete)
* [`ideavault search`↴](#ideavault-search)
* [`ideavault version`↴](#ideavault-version)

## `ideavault`

A CLI tool for managing ideas and projects

**Usage:** `ideavault <COMMAND>`

###### **Subcommands:**

* `idea` — Manage ideas
* `project` — Manage projects
* `task` — Manage tasks
* `search` — Search across ideas, projects, and tags
* `version` — Show version information



## `ideavault idea`

Manage ideas

**Usage:** `ideavault idea <COMMAND>`

###### **Subcommands:**

* `new` — Create a new idea
* `list` — List ideas with optional filtering
* `show` — Show full details of an idea
* `tag` — Add or replace tags on an idea
* `status` — Update the status of an idea
* `edit` — Edit an idea in $EDITOR
* `delete` — Delete an idea with confirmation
* `update` — Update idea fields (title, description, status)



## `ideavault idea new`

Create a new idea

**Usage:** `ideavault idea new [OPTIONS] <TITLE>`

###### **Arguments:**

* `<TITLE>` — The title of the idea

###### **Options:**

* `-d`, `--description <DESCRIPTION>` — Optional description for the idea
* `-t`, `--tags <TAGS>` — Optional tags (comma-separated)



## `ideavault idea list`

List ideas with optional filtering

**Usage:** `ideavault idea list [OPTIONS]`

###### **Options:**

* `-s`, `--status <STATUS>` — Filter by status (Brainstorming|Active|Completed|Archived)
* `-t`, `--tag <TAG>` — Filter by tag



## `ideavault idea show`

Show full details of an idea

**Usage:** `ideavault idea show <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the idea to show



## `ideavault idea tag`

Add or replace tags on an idea

**Usage:** `ideavault idea tag <ID> [TAGS]...`

###### **Arguments:**

* `<ID>` — The UUID of the idea to tag
* `<TAGS>` — Tags to add/replace (space-separated)



## `ideavault idea status`

Update the status of an idea

**Usage:** `ideavault idea status <ID> <STATUS>`

###### **Arguments:**

* `<ID>` — The UUID of the idea to update
* `<STATUS>` — New status for the idea



## `ideavault idea edit`

Edit an idea in $EDITOR

**Usage:** `ideavault idea edit <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the idea to edit



## `ideavault idea delete`

Delete an idea with confirmation

**Usage:** `ideavault idea delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the idea to delete

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt



## `ideavault idea update`

Update idea fields (title, description, status)

**Usage:** `ideavault idea update [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — Idea ID to update

###### **Options:**

* `-t`, `--title <TITLE>` — New title
* `-d`, `--description <DESCRIPTION>` — New description
* `-s`, `--status <STATUS>` — New status
* `--clear <FIELD>` — Clear one or more optional fields (description)



## `ideavault project`

Manage projects

**Usage:** `ideavault project <COMMAND>`

###### **Subcommands:**

* `new` — Create a new project
* `list` — List projects with optional filtering
* `show` — Show full details of a project
* `link` — Link an idea to a project
* `unlink` — Remove idea link from project
* `ideas` — List all ideas linked to a project
* `status` — Update the status of a project
* `delete` — Delete a project with confirmation
* `update` — Update project fields (title, description, milestone, url, repo, status)



## `ideavault project new`

Create a new project

**Usage:** `ideavault project new [OPTIONS] <TITLE>`

###### **Arguments:**

* `<TITLE>` — The title of the project

###### **Options:**

* `-d`, `--description <DESCRIPTION>` — Optional description for the project
* `-m`, `--milestone <MILESTONE>` — Optional milestone for the project
* `--url <URL>` — Optional URL for the project
* `--repo <REPO>` — Optional repository for the project



## `ideavault project list`

List projects with optional filtering

**Usage:** `ideavault project list [OPTIONS]`

###### **Options:**

* `-s`, `--status <STATUS>` — Filter by status (Planning|InProgress|Completed|OnHold)



## `ideavault project show`

Show full details of a project

**Usage:** `ideavault project show <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the project to show



## `ideavault project link`

Link an idea to a project

**Usage:** `ideavault project link <project-id> <idea-id>`

###### **Arguments:**

* `<project-id>` — The UUID of the project
* `<idea-id>` — The UUID of the idea to link



## `ideavault project unlink`

Remove idea link from project

**Usage:** `ideavault project unlink <project-id> <idea-id>`

###### **Arguments:**

* `<project-id>` — The UUID of the project
* `<idea-id>` — The UUID of the idea to unlink



## `ideavault project ideas`

List all ideas linked to a project

**Usage:** `ideavault project ideas <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the project



## `ideavault project status`

Update the status of a project

**Usage:** `ideavault project status <ID> <STATUS>`

###### **Arguments:**

* `<ID>` — The UUID of the project to update
* `<STATUS>` — New status for the project



## `ideavault project delete`

Delete a project with confirmation

**Usage:** `ideavault project delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the project to delete

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt



## `ideavault project update`

Update project fields (title, description, milestone, url, repo, status)

**Usage:** `ideavault project update [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — Project ID to update

###### **Options:**

* `-t`, `--title <TITLE>` — New title
* `-d`, `--description <DESCRIPTION>` — New description
* `-m`, `--milestone <MILESTONE>` — New milestone
* `--url <URL>` — New URL
* `--repo <REPO>` — New repository URL
* `-s`, `--status <STATUS>` — New status
* `--clear <FIELD>` — Clear one or more optional fields (description, milestone, url, repo)



## `ideavault task`

Manage tasks

**Usage:** `ideavault task <COMMAND>`

###### **Subcommands:**

* `new` — Create a new task
* `list` — List tasks with optional filtering
* `show` — Show full details of a task
* `status` — Update the status of a task
* `priority` — Update the priority of a task
* `due` — Set due date for a task
* `link-project` — Link task to a project
* `link-idea` — Link task to an idea
* `unlink-project` — Unlink task from project
* `unlink-idea` — Unlink task from idea
* `edit` — Edit a task in $EDITOR
* `delete` — Delete a task with confirmation



## `ideavault task new`

Create a new task

**Usage:** `ideavault task new [OPTIONS] <TITLE>`

###### **Arguments:**

* `<TITLE>` — The title of the task

###### **Options:**

* `-d`, `--description <DESCRIPTION>` — Optional description for the task
* `-p`, `--priority <PRIORITY>` — Optional priority (low|medium|high|urgent)
* `-D`, `--due <DUE_DATE>` — Optional due date (YYYY-MM-DD format)
* `-t`, `--tags <TAGS>` — Optional tags (comma-separated, GTD-style contexts)
* `--project <PROJECT_ID>` — Optional project ID to link to
* `--idea <IDEA_ID>` — Optional idea ID to link to



## `ideavault task list`

List tasks with optional filtering

**Usage:** `ideavault task list [OPTIONS]`

###### **Options:**

* `-s`, `--status <STATUS>` — Filter by status (todo|inprogress|blocked|done|cancelled)
* `-p`, `--priority <PRIORITY>` — Filter by priority (low|medium|high|urgent)
* `-t`, `--tag <TAG>` — Filter by tag (GTD-style context)
* `--project <PROJECT_ID>` — Filter by project ID
* `--idea <IDEA_ID>` — Filter by idea ID
* `--overdue` — Show overdue tasks only



## `ideavault task show`

Show full details of a task

**Usage:** `ideavault task show <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the task to show



## `ideavault task status`

Update the status of a task

**Usage:** `ideavault task status <ID> <STATUS>`

###### **Arguments:**

* `<ID>` — The UUID of the task to update
* `<STATUS>` — New status for the task



## `ideavault task priority`

Update the priority of a task

**Usage:** `ideavault task priority <ID> <PRIORITY>`

###### **Arguments:**

* `<ID>` — The UUID of the task to update
* `<PRIORITY>` — New priority for the task



## `ideavault task due`

Set due date for a task

**Usage:** `ideavault task due <ID> <DUE_DATE>`

###### **Arguments:**

* `<ID>` — The UUID of the task to update
* `<DUE_DATE>` — Due date (YYYY-MM-DD format) or "clear" to remove



## `ideavault task link-project`

Link task to a project

**Usage:** `ideavault task link-project <ID> <PROJECT_ID>`

###### **Arguments:**

* `<ID>` — The UUID of the task
* `<PROJECT_ID>` — The UUID of the project to link



## `ideavault task link-idea`

Link task to an idea

**Usage:** `ideavault task link-idea <ID> <IDEA_ID>`

###### **Arguments:**

* `<ID>` — The UUID of the task
* `<IDEA_ID>` — The UUID of the idea to link



## `ideavault task unlink-project`

Unlink task from project

**Usage:** `ideavault task unlink-project <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the task



## `ideavault task unlink-idea`

Unlink task from idea

**Usage:** `ideavault task unlink-idea <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the task



## `ideavault task edit`

Edit a task in $EDITOR

**Usage:** `ideavault task edit <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the task to edit



## `ideavault task delete`

Delete a task with confirmation

**Usage:** `ideavault task delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — The UUID of the task to delete

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt



## `ideavault search`

Search across ideas, projects, and tags

**Usage:** `ideavault search [OPTIONS] <QUERY>`

###### **Arguments:**

* `<QUERY>` — Search query string

###### **Options:**

* `-i`, `--ideas` — Search in ideas only
* `-p`, `--projects` — Search in projects only
* `-t`, `--tags` — Search in tags only
* `-s`, `--status <STATUS>` — Filter by status
* `--with-tags <WITH_TAGS>` — Filter by tags (space-separated, multiple allowed)
* `--from <DATE_FROM>` — Filter by date from (YYYY-MM-DD format)
* `--to <DATE_TO>` — Filter by date to (YYYY-MM-DD format)



## `ideavault version`

Show version information

**Usage:** `ideavault version [OPTIONS]`

###### **Options:**

* `-c`, `--check` — Check for updates



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

