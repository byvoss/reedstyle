# Ticket #950: Git Repository Setup

## Status: 🚧 In Progress
## Priority: 🔴 Critical

## Description
Establish clean Git repository with ticket-based commit history from the beginning. Archive old repository and create fresh start with proper development workflow.

## Requirements

### Repository Migration
1. Archive current repository to `reedstyle-archive`
2. Create new clean `reedstyle` repository  
3. Initialize with ticket #001 as first commit
4. Establish ticket-based commit convention

### Commit Convention
Every commit MUST reference a ticket:
```
<type>(<scope>): <description> (#<ticket-number>)

<optional body>

Closes #<ticket-number>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, no code change
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance tasks
- `build`: Build system changes

## Implementation Steps

### Step 1: Archive Current Repository
```bash
# Rename current repo on GitHub
gh repo rename reedstyle-archive

# Update description
gh repo edit --description "ARCHIVED: Old ReedSTYLE implementation (pre-v1.0)"

# Archive the repository
gh repo archive byvoss/reedstyle-archive --yes
```

### Step 2: Create New Repository
```bash
# Create new public repository
gh repo create reedstyle --public \
  --description "HTML-first styling system with reed element" \
  --homepage "https://reedstyle.dev"

# Add topics
gh repo edit --add-topic "css-framework"
gh repo edit --add-topic "html-first"  
gh repo edit --add-topic "rust"
gh repo edit --add-topic "typescript"
```

### Step 3: Initialize Local Repository
```bash
# Remove old git
rm -rf .git

# Initialize new repository
git init
git branch -M main

# Create initial commit structure
git add .gitignore LICENSE
git commit -m "chore: Initialize repository (#950)

- Setup project structure
- Add Apache 2.0 license
- Configure .gitignore

Closes #950"

# Add remote
git remote add origin https://github.com/byvoss/reedstyle.git

# Push first commit
git push -u origin main
```

### Step 4: Setup Branch Protection
```bash
# Configure branch protection rules
gh api repos/byvoss/reedstyle/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["build"]}' \
  --field enforce_admins=false \
  --field required_pull_request_reviews='{"required_approving_review_count":1}' \
  --field restrictions=null
```

### Step 5: Create Initial Issues
```bash
# Create milestone
gh api repos/byvoss/reedstyle/milestones \
  --method POST \
  --field title="v1.0.0" \
  --field description="Initial release with reed element system"

# Create first tickets
gh issue create --title "Ticket #950: Git Repository Setup" \
  --body "Setup clean repository with ticket-based workflow" \
  --label "chore"

gh issue create --title "Ticket #951: Project Structure" \
  --body "Create base project structure" \
  --label "build"

gh issue create --title "Ticket #906: Rust Build System" \
  --body "Implement core Rust build system" \
  --label "feat"
```

## Git Workflow Rules

### 1. No Direct Commits to Main
All changes through pull requests:
```bash
git checkout -b ticket-002-project-structure
# Make changes
git commit -m "feat: Add project structure (#002)"
git push origin ticket-002-project-structure
gh pr create
```

### 2. Commit Message Examples

**Feature:**
```
feat(build): Add Rust CSS generator (#003)

- Implement namespace modules
- Add Lightning CSS integration
- Create layer system

Closes #003
```

**Documentation:**
```
docs(readme): Add installation instructions (#004)

Closes #004
```

**Fix:**
```
fix(css): Correct reed selector specificity (#005)

The selector was too specific causing override issues.

Closes #005
```

### 3. PR Template
```markdown
## Ticket
Closes #XXX

## Changes
- 
- 
- 

## Testing
- [ ] Tests pass
- [ ] Manual testing complete

## Checklist
- [ ] Follows commit convention
- [ ] Documentation updated
- [ ] No breaking changes
```

## Verification

- [ ] Old repo archived as `reedstyle-archive`
- [ ] New repo created as `reedstyle`
- [ ] First commit references ticket #001
- [ ] Branch protection enabled
- [ ] Initial tickets created
- [ ] Team notified of change

## Documentation

This document serves as:
1. **Ticket #001** implementation guide
2. **Permanent reference** for Git workflow
3. **Template** for repository setup

## Commit Message for This Document
```
docs: Add Git repository setup documentation (#001)

- Define repository migration process
- Establish commit conventions
- Document workflow rules

Part of #001
```

## Notes

- Every commit from now on MUST reference a ticket
- No exceptions to the ticket rule
- This ensures complete traceability
- Makes changelog generation automatic
- Enables precise project tracking