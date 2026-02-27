# Command Line Help

## project new

Create a new project.

Usage:

ideavault project new "PROJECT TITLE" [options]

Options:
- --description "TEXT"     Set project description
- --ideas <IDs>            Link existing idea IDs (comma/space separated)
- --url <URL>              (new) Set the project website or homepage URL
- --repo <GIT_REPO_URL>    (new) Set the canonical git repository URL for the project

Examples:

ideavault project new "JeetSocial" --description "Platform to encourage small acts of kindness" --url https://jeetsocial.com --repo https://github.com/bigknoxy/jeetSocial2


