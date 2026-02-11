# IdeaVault Uninstall Implementation

## Goal
Implement a complete uninstall solution for ideavault including uninstall script, documentation updates, and testing.

## Tasks

- [x] Create `/root/ideavault/uninstall.sh` script
- [x] Update `/root/ideavault/install.sh` with installation documentation
- [x] Update `/root/ideavault/README.md` with uninstall section
- [ ] Create git branch for changes
- [ ] Commit all changes
- [ ] Create pull request
- [ ] Merge pull request
- [ ] Test uninstall script with dry-run
- [ ] Verify uninstall script works correctly

## Notes

- Binary location: `/usr/local/bin/ideavault`
- Data location: `~/.local/share/ideavault/`
- Cache location: `~/.cache/ideavault/`
- Uninstall flags: `--dry-run`, `--keep-data`, `--force`
