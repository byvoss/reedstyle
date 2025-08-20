#!/bin/bash

# ReedSTYLE Release Script
# Usage: ./scripts/release.sh [patch|minor|major]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version type is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version type required${NC}"
    echo "Usage: $0 [patch|minor|major]"
    echo "  patch: 0.1.0 -> 0.1.1"
    echo "  minor: 0.1.0 -> 0.2.0"
    echo "  major: 0.1.0 -> 1.0.0"
    exit 1
fi

VERSION_TYPE=$1

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${RED}Error: Releases must be created from main branch${NC}"
    echo "Current branch: $CURRENT_BRANCH"
    echo "Run: git checkout main"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: Uncommitted changes detected${NC}"
    echo "Please commit or stash your changes first"
    exit 1
fi

# Pull latest changes
echo -e "${YELLOW}Pulling latest changes...${NC}"
git pull origin main

# Get current version from src/builder/mod.rs
CURRENT_VERSION=$(grep 'pub const VERSION' src/builder/mod.rs | cut -d'"' -f2)
echo -e "Current version: ${YELLOW}$CURRENT_VERSION${NC}"

# Calculate new version
IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR=${VERSION_PARTS[0]}
MINOR=${VERSION_PARTS[1]}
PATCH=${VERSION_PARTS[2]}

case $VERSION_TYPE in
    patch)
        PATCH=$((PATCH + 1))
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    *)
        echo -e "${RED}Error: Invalid version type '$VERSION_TYPE'${NC}"
        echo "Use: patch, minor, or major"
        exit 1
        ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
echo -e "New version: ${GREEN}$NEW_VERSION${NC}"

# Update version in src/builder/mod.rs
echo -e "${YELLOW}Updating version in source...${NC}"
sed -i.bak "s/pub const VERSION: &str = \"$CURRENT_VERSION\"/pub const VERSION: &str = \"$NEW_VERSION\"/" src/builder/mod.rs
rm src/builder/mod.rs.bak

# Build to verify everything works
echo -e "${YELLOW}Building ReedSTYLE...${NC}"
cargo run --release

# Run tests
echo -e "${YELLOW}Running tests...${NC}"
cargo test

# Verify output files
echo -e "${YELLOW}Verifying distribution files...${NC}"
for file in dist/reedstyle.css dist/reedstyle.min.css dist/reedstyle.js dist/reedstyle.min.js dist/LICENSE; do
    if [ ! -f "$file" ]; then
        echo -e "${RED}Error: Missing file $file${NC}"
        exit 1
    fi
done

# Check file sizes
CSS_SIZE=$(wc -c < dist/reedstyle.min.css)
JS_SIZE=$(wc -c < dist/reedstyle.min.js)

if [ $CSS_SIZE -gt 204800 ]; then
    echo -e "${RED}Error: CSS exceeds 200KB limit ($(($CSS_SIZE / 1024))KB)${NC}"
    exit 1
fi

if [ $JS_SIZE -gt 51200 ]; then
    echo -e "${RED}Error: JS exceeds 50KB limit ($(($JS_SIZE / 1024))KB)${NC}"
    exit 1
fi

echo -e "${GREEN}✓ All files generated successfully${NC}"
echo "  CSS: $(($CSS_SIZE / 1024))KB (limit: 200KB)"
echo "  JS: $(($JS_SIZE / 1024))KB (limit: 50KB)"

# Commit version change
echo -e "${YELLOW}Committing version change...${NC}"
git add src/builder/mod.rs
git commit -m "chore: Bump version to $NEW_VERSION"

# Create tag
echo -e "${YELLOW}Creating tag v$NEW_VERSION...${NC}"
git tag -a "v$NEW_VERSION" -m "Release version $NEW_VERSION"

# Show summary
echo ""
echo -e "${GREEN}🎉 Release prepared successfully!${NC}"
echo ""
echo "Version $CURRENT_VERSION → $NEW_VERSION"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git show HEAD"
echo "  2. Push to GitHub: git push origin main --tags"
echo "  3. GitHub Actions will create the release automatically"
echo ""
echo "To undo this release (before pushing):"
echo "  git tag -d v$NEW_VERSION"
echo "  git reset --hard HEAD~1"