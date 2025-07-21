# npm Publishing Guide for gwtr

This guide explains how to publish gwtr to npm.

## Prerequisites

1. npm account with publishing rights
2. `NPM_TOKEN` secret set in GitHub repository settings

## Publishing Process

### Automated Publishing (Recommended)

The GitHub Actions workflow handles the entire publishing process:

1. **Create a GitHub Release**
   - Go to the Releases page
   - Click "Create a new release"
   - Tag version should match npm version (e.g., v0.1.2)
   - The workflow will automatically:
     - Build binaries for all platforms
     - Publish platform-specific packages
     - Publish the main package

2. **Manual Workflow Dispatch**
   - Go to Actions → "Publish to npm" workflow
   - Click "Run workflow"
   - Enter the version number
   - The workflow will build and publish all packages

### Local Testing

Before publishing, test the package locally:

```bash
# Build and test locally
cd npm
./test-local.sh

# Link the package globally
npm link

# Test the CLI
gwtr --help
```

### Manual Publishing (Emergency Only)

If automated publishing fails:

1. Build binaries for all platforms locally or download from releases
2. Place binaries in correct directories:
   ```
   npm/platforms/darwin-x64/gwtr
   npm/platforms/darwin-arm64/gwtr
   npm/platforms/linux-x64/gwtr
   npm/platforms/linux-arm64/gwtr
   npm/platforms/win32-x64/gwtr.exe
   ```

3. Publish platform packages first:
   ```bash
   cd npm
   for platform in platforms/*; do
     cd $platform && npm publish --access public && cd ../..
   done
   ```

4. Publish main package:
   ```bash
   npm publish --access public
   ```

## Version Management

- Keep npm version in sync with Cargo.toml version
- Update all package.json files when bumping version
- Use semantic versioning (MAJOR.MINOR.PATCH)

## Troubleshooting

- **Binary not found**: Ensure all platform binaries are built and placed correctly
- **Permission denied**: Check npm authentication and package access rights
- **Version conflict**: Ensure version hasn't been published already