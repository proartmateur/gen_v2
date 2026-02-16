#!/bin/bash

# GenCLI Release Helper Script
# Makes it easy to create a new release

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   GenCLI Release Helper Script        ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo ""

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo ""
    echo "Usage: ./create_release.sh <version>"
    echo "Example: ./create_release.sh 2.0.0"
    echo ""
    exit 1
fi

VERSION="$1"
TAG="v${VERSION}"

echo -e "${YELLOW}Creating release for version: ${GREEN}${VERSION}${NC}"
echo ""

# Step 1: Check git status
echo -e "${BLUE}[1/8]${NC} Checking git status..."
if [[ -n $(git status -s) ]]; then
    echo -e "${YELLOW}Warning: You have uncommitted changes:${NC}"
    git status -s
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${RED}Aborted.${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}✓ Working directory clean${NC}"
fi
echo ""

# Step 2: Run tests
echo -e "${BLUE}[2/8]${NC} Running tests..."
if cargo test --quiet; then
    echo -e "${GREEN}✓ All tests passed (18/18)${NC}"
else
    echo -e "${RED}✗ Tests failed. Fix them before releasing.${NC}"
    exit 1
fi
echo ""

# Step 3: Build release
echo -e "${BLUE}[3/8]${NC} Building release binary..."
if cargo build --release --quiet; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed. Fix compilation errors.${NC}"
    exit 1
fi
echo ""

# Step 4: Update Cargo.toml version
echo -e "${BLUE}[4/8]${NC} Updating Cargo.toml version..."
if grep -q "version = \"${VERSION}\"" Cargo.toml; then
    echo -e "${GREEN}✓ Version already set to ${VERSION}${NC}"
else
    # Backup
    cp Cargo.toml Cargo.toml.backup
    
    # Update version
    sed -i.tmp "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
    rm Cargo.toml.tmp 2>/dev/null || true
    
    echo -e "${GREEN}✓ Updated version to ${VERSION}${NC}"
    
    # Show diff
    echo -e "${YELLOW}Changes in Cargo.toml:${NC}"
    git diff Cargo.toml | grep "^[+-]version" || true
fi
echo ""

# Step 5: Commit version change
echo -e "${BLUE}[5/8]${NC} Committing version change..."
if git diff --quiet Cargo.toml; then
    echo -e "${YELLOW}⊘ No changes to commit${NC}"
else
    git add Cargo.toml
    git commit -m "chore: bump version to ${VERSION}"
    echo -e "${GREEN}✓ Committed version bump${NC}"
fi
echo ""

# Step 6: Create git tag
echo -e "${BLUE}[6/8]${NC} Creating git tag..."
if git tag -l | grep -q "^${TAG}$"; then
    echo -e "${YELLOW}Warning: Tag ${TAG} already exists${NC}"
    read -p "Delete and recreate? (y/N) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git tag -d "${TAG}"
        git push --delete origin "${TAG}" 2>/dev/null || true
        git tag -a "${TAG}" -m "Release ${VERSION}"
        echo -e "${GREEN}✓ Recreated tag ${TAG}${NC}"
    else
        echo -e "${RED}Aborted.${NC}"
        exit 1
    fi
else
    git tag -a "${TAG}" -m "Release ${VERSION}"
    echo -e "${GREEN}✓ Created tag ${TAG}${NC}"
fi
echo ""

# Step 7: Push changes
echo -e "${BLUE}[7/8]${NC} Pushing to GitHub..."
echo -e "${YELLOW}This will trigger the release build workflow.${NC}"
read -p "Push now? (Y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Nn]$ ]]; then
    echo -e "${YELLOW}Skipped push. Remember to push manually:${NC}"
    echo "  git push origin main"
    echo "  git push origin ${TAG}"
else
    git push origin main
    git push origin "${TAG}"
    echo -e "${GREEN}✓ Pushed to GitHub${NC}"
fi
echo ""

# Step 8: Summary
echo -e "${BLUE}[8/8]${NC} Release summary"
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Release ${VERSION} Ready!                  ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Go to GitHub Actions to monitor the build:"
echo "   https://github.com/YOUR_USERNAME/gen_v2/actions"
echo ""
echo "2. Once builds complete (~5-10 min), check the release:"
echo "   https://github.com/YOUR_USERNAME/gen_v2/releases/tag/${TAG}"
echo ""
echo "3. Downloaded assets will include:"
echo "   • gen-${TAG}-x86_64-pc-windows-msvc.zip (Windows)"
echo "   • gen-${TAG}-x86_64-unknown-linux-gnu.tar.gz (Linux)"
echo "   • gen-${TAG}-x86_64-apple-darwin.tar.gz (macOS Intel)"
echo "   • gen-${TAG}-aarch64-apple-darwin.tar.gz (macOS Apple Silicon)"
echo ""
echo -e "${GREEN}✓ Release process complete!${NC}"
echo ""
