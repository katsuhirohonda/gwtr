# Crates.io Publishing Guide

This guide explains how to automatically publish gwtr to crates.io using GitHub Actions.

## Setup

### 1. Get your crates.io API Token

1. Log in to [crates.io](https://crates.io/)
2. Go to Account Settings → API Tokens
3. Click "New Token"
4. Give it a name like "gwtr-github-actions"
5. Copy the generated token

### 2. Add Token to GitHub Secrets

1. Go to https://github.com/katsuhirohonda/gwtr/settings/secrets/actions
2. Click "New repository secret"
3. Name: `CARGO_REGISTRY_TOKEN`
4. Secret: Paste your crates.io API token
5. Click "Add secret"

## Publishing Process

### Automatic Publishing (via GitHub Release)

When you create a GitHub release, the workflow automatically:
1. Verifies the version matches the git tag
2. Runs all tests
3. Checks if the version is already published
4. Publishes to crates.io

### Manual Publishing (via Workflow Dispatch)

1. Go to Actions → "Publish to crates.io"
2. Click "Run workflow"
3. Choose dry run option:
   - `true`: Only simulate publishing (recommended for testing)
   - `false`: Actually publish to crates.io

## Version Management

Before creating a release:

```bash
# 1. Update version in Cargo.toml
# 2. Update Cargo.lock
cargo update

# 3. Commit changes
git add Cargo.toml Cargo.lock
git commit -m "chore: Bump version to x.y.z"

# 4. Create and push tag
git tag -a vx.y.z -m "Release version x.y.z"
git push origin main
git push origin vx.y.z

# 5. Create GitHub release
# The workflow will automatically publish to crates.io
```

## Troubleshooting

- **"Version already published"**: Each version can only be published once to crates.io
- **"Version mismatch"**: Ensure Cargo.toml version matches the git tag (without 'v' prefix)
- **"Authentication failed"**: Check that CARGO_REGISTRY_TOKEN is correctly set

## Testing

Use the dry run option to test the publishing process without actually publishing:

```bash
# Via GitHub Actions UI
# Select "Run workflow" → dry_run: true
```