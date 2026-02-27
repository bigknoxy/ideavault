---
name: ideavault-release
description: IdeaVault release process - version bumping, tagging, and monitoring
license: MIT
compatibility: opencode
metadata:
  project: ideavault
  language: rust
  type: release
---

## Release Process

1. Merge all changes to main via PR
2. Update version in `Cargo.toml`
3. Push tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
4. GitHub Actions automatically:
   - Builds binary (musl static)
   - Generates `docs/CommandLineHelp.md`
   - Creates GitHub release with binary attached

---

## Pre-Release Checklist

Before creating a release:

- [ ] Verify Cargo.toml version is correct (check against existing tags)
- [ ] `cargo build --release` succeeds
- [ ] `cargo test` passes
- [ ] `cargo fmt` applied
- [ ] `cargo clippy -- -D warnings` clean

### Verify Version

```bash
# Check current Cargo.toml version
grep '^version' Cargo.toml | head -1

# Check latest git tag
git tag --sort=-v:refname | head -5

# Ensure Cargo.toml version > latest tag
```

---

## Release Commands

```bash
# 1. Merge PR with version bump to main
gh pr merge <number> --squash --delete-branch

# 2. Pull latest and create tag
git checkout main && git pull
git tag vX.Y.Z && git push origin vX.Y.Z

# 3. Monitor release
gh run list --limit 3
gh release view vX.Y.Z
```

---

## Monitor Release

```bash
# Check workflow status
gh run list --limit 3

# View release details
gh release view vX.Y.Z

# Watch workflow run
gh run watch
```

---

## Troubleshooting

### Version Mismatch
If Cargo.toml version is behind tags:
1. Create PR to bump version in Cargo.toml
2. Merge PR
3. Then proceed with tagging

### Failed Release
1. Check workflow logs: `gh run view`
2. Fix issues in new PR
3. Delete failed tag if needed: `git push --delete origin vX.Y.Z`
4. Re-tag after fix: `git tag -d vX.Y.Z && git tag vX.Y.Z && git push origin vX.Y.Z`

---

## When to Use This Skill

Use this skill when:
- Preparing a new release
- Bumping version numbers
- Creating and pushing git tags
- Monitoring GitHub Actions release workflow
- Troubleshooting release failures
