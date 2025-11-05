# GitHub Repository Rename Instructions

## Overview

This document provides step-by-step instructions for renaming the GitHub repository from `neon-seren-migrator` to `neon-seren-replicator`.

## Prerequisites

- Repository admin access
- All code changes committed and pushed

## Rename Steps

### 1. Rename the GitHub Repository

1. Navigate to the repository settings:
   ```
   https://github.com/serenorg/neon-seren-migrator/settings
   ```

2. Scroll down to the "Repository name" section

3. Enter the new repository name:
   ```
   neon-seren-replicator
   ```

4. Click the **"Rename"** button

5. Confirm the rename when prompted

### 2. GitHub Automatic Redirects

GitHub will automatically:
- Set up redirects from the old URL to the new URL
- Update all references in pull requests, issues, and wikis
- Redirect git operations (clone, fetch, push) from old URL to new URL

### 3. Update Local Clones

For team members with local clones, update the remote URL:

```bash
# Navigate to your local repository
cd path/to/seren-neon-migrator

# Update the remote URL
git remote set-url origin https://github.com/serenorg/neon-seren-replicator.git

# Verify the change
git remote -v
```

Expected output:
```
origin  https://github.com/serenorg/neon-seren-replicator.git (fetch)
origin  https://github.com/serenorg/neon-seren-replicator.git (push)
```

### 4. Update CI/CD Integration (if applicable)

If you have external CI/CD services (not GitHub Actions), update:
- CircleCI configuration
- Travis CI configuration
- Jenkins jobs
- Other external services referencing the repository

**Note**: GitHub Actions is already updated in the codebase.

### 5. Update Documentation Links

Update any external documentation that references the old repository:
- Company wiki pages
- Blog posts
- External documentation sites
- README files in other repositories

### 6. Update Crates.io (for Rust projects)

If published to crates.io:

1. Update the `repository` field in Cargo.toml (already done)
2. Publish a new version:
   ```bash
   cargo publish
   ```

### 7. Notify Users

Consider notifying users through:
- GitHub release notes
- README update
- Announcement in discussions/issues
- Social media/blog post

## Verification

After renaming, verify:

1. **Old URL redirects properly**:
   ```bash
   curl -I https://github.com/serenorg/neon-seren-migrator
   ```
   Should return `HTTP/1.1 301 Moved Permanently`

2. **Git operations work**:
   ```bash
   git fetch
   git pull
   ```

3. **CI/CD pipeline runs successfully** on the new repository

4. **Release artifacts have correct names**

## Rollback Plan

If issues arise:
1. GitHub allows renaming back to the original name
2. Re-run the rename process in reverse
3. Notify team to revert remote URLs

## Important Notes

- **Redirects are temporary**: While GitHub maintains redirects, it's best practice to update all references promptly
- **Old name availability**: The old repository name (`neon-seren-migrator`) becomes available for others to claim immediately after rename
- **Badge URLs**: Update badge URLs in README (already done)
- **Crate name**: Consider reserving the old crate name on crates.io to prevent confusion

## Timeline

- **Code changes**: Completed
- **Repository rename**: Pending (requires admin access)
- **Team notification**: After rename
- **Documentation updates**: Within 24 hours

## Contact

If you encounter any issues during the rename process, contact:
- Repository administrators
- DevOps team
- Open a GitHub issue in the new repository
