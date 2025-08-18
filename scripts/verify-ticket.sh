#!/bin/bash

# ReedSTYLE Ticket Verification Script
# Usage: ./scripts/verify-ticket.sh <ticket-number>

set -e

TICKET_NUMBER=$1
if [ -z "$TICKET_NUMBER" ]; then
    echo "❌ Usage: $0 <ticket-number>"
    exit 1
fi

echo "═══════════════════════════════════════════════════════"
echo "   ReedSTYLE Ticket #$TICKET_NUMBER Verification"
echo "═══════════════════════════════════════════════════════"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Verification counters
PASSED=0
FAILED=0
WARNINGS=0

# Function to check condition
check() {
    local description=$1
    local command=$2
    
    echo -n "Checking: $description... "
    
    if eval $command > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}✗${NC}"
        ((FAILED++))
        return 1
    fi
}

# Function to warn
warn() {
    local description=$1
    local command=$2
    
    echo -n "Warning: $description... "
    
    if eval $command > /dev/null 2>&1; then
        echo -e "${GREEN}OK${NC}"
    else
        echo -e "${YELLOW}⚠${NC}"
        ((WARNINGS++))
    fi
}

echo ""
echo "1️⃣  BUILD VERIFICATION"
echo "───────────────────────"

# Rust build checks
check "Rust compilation" "cargo build --release"
check "Rust tests pass" "cargo test"
check "Clippy warnings" "cargo clippy -- -D warnings"

# TypeScript checks
check "TypeScript types" "npm run type-check || npx tsc --noEmit"

echo ""
echo "2️⃣  OUTPUT VERIFICATION"
echo "───────────────────────"

# Build the project
echo "Building ReedSTYLE..."
cargo run --release > /dev/null 2>&1

# Check output files exist
check "CSS file exists" "test -f dist/reedstyle.css"
check "Minified CSS exists" "test -f dist/reedstyle.min.css"
check "JS file exists" "test -f dist/reedstyle.js"
check "Minified JS exists" "test -f dist/reedstyle.min.js"

# Check file sizes
CSS_SIZE=$(wc -c < dist/reedstyle.css 2>/dev/null || echo 999999)
MIN_CSS_SIZE=$(wc -c < dist/reedstyle.min.css 2>/dev/null || echo 999999)
JS_SIZE=$(wc -c < dist/reedstyle.js 2>/dev/null || echo 999999)
MIN_JS_SIZE=$(wc -c < dist/reedstyle.min.js 2>/dev/null || echo 999999)

check "CSS size < 350KB" "[ $CSS_SIZE -lt 358400 ]"
check "Min CSS < 180KB" "[ $MIN_CSS_SIZE -lt 184320 ]"
check "JS size < 100KB" "[ $JS_SIZE -lt 102400 ]"
check "Min JS < 50KB" "[ $MIN_JS_SIZE -lt 51200 ]"

echo ""
echo "3️⃣  CSS STRUCTURE VERIFICATION"
echo "───────────────────────────────"

# Check CSS contains required patterns
check "Has layer system" "grep -q '@layer settings, bridge, theme, free' dist/reedstyle.css"
check "Uses reed selectors" "grep -q 'reed\[' dist/reedstyle.css"
check "No old data-r- attrs" "! grep -q 'data-r-' dist/reedstyle.css"
check "Has OKLCH colors" "grep -q 'oklch(' dist/reedstyle.css"
check "Has CSS variables" "grep -q '--' dist/reedstyle.css"

# Check responsive
check "Has media queries" "grep -q '@media' dist/reedstyle.css"
check "Has tablet breakpoint" "grep -q 'min-width: 560px' dist/reedstyle.css"
check "Has screen breakpoint" "grep -q 'min-width: 960px' dist/reedstyle.css"

echo ""
echo "4️⃣  JAVASCRIPT VERIFICATION"
echo "───────────────────────────"

# Check JavaScript patterns
check "Defines customElements" "grep -q 'customElements.define' dist/reedstyle.js"
check "Has ReedStyle global" "grep -q 'window.ReedStyle' dist/reedstyle.js"
check "No console.log" "! grep -q 'console.log' dist/reedstyle.min.js"

echo ""
echo "5️⃣  DOCUMENTATION CHECK"
echo "───────────────────────"

# Check if ticket documentation exists
TICKET_DOC="docs/develop/$TICKET_NUMBER-*.md"
check "Ticket doc exists" "ls $TICKET_DOC 2>/dev/null | grep -q ."

if ls $TICKET_DOC 2>/dev/null | grep -q .; then
    DOC_FILE=$(ls $TICKET_DOC | head -1)
    check "Has status field" "grep -q '^## Status:' $DOC_FILE"
    check "Has requirements" "grep -q '^## Requirements' $DOC_FILE"
    check "Has acceptance criteria" "grep -q '^## Acceptance Criteria' $DOC_FILE"
    warn "Has testing section" "grep -q '^## Testing' $DOC_FILE"
fi

echo ""
echo "6️⃣  PERFORMANCE CHECK"
echo "───────────────────────"

# Measure build time
echo -n "Build time: "
BUILD_TIME=$( { time cargo run --release > /dev/null 2>&1; } 2>&1 | grep real | awk '{print $2}' )
echo "$BUILD_TIME"

# Convert to milliseconds for comparison (rough)
if [[ "$BUILD_TIME" < "0m0.500s" ]]; then
    echo -e "Performance: ${GREEN}✓ Under 500ms${NC}"
    ((PASSED++))
else
    echo -e "Performance: ${YELLOW}⚠ Over 500ms${NC}"
    ((WARNINGS++))
fi

echo ""
echo "7️⃣  GIT STATUS CHECK"
echo "───────────────────────"

# Check git status
warn "No uncommitted changes" "[ -z \"\$(git status --porcelain)\" ]"
check "On feature branch" "git branch --show-current | grep -q 'ticket-$TICKET_NUMBER'"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "                    VERIFICATION SUMMARY"
echo "═══════════════════════════════════════════════════════"
echo ""
echo -e "  Passed:   ${GREEN}$PASSED${NC} checks"
echo -e "  Failed:   ${RED}$FAILED${NC} checks"
echo -e "  Warnings: ${YELLOW}$WARNINGS${NC} checks"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ VERIFICATION PASSED!${NC}"
    echo ""
    echo "Ready to commit with:"
    echo "  git add -A"
    echo "  git commit -m \"feat: Implement [feature] (#$TICKET_NUMBER)\""
    echo "  git push origin ticket-$TICKET_NUMBER-[feature]"
    exit 0
else
    echo -e "${RED}❌ VERIFICATION FAILED!${NC}"
    echo ""
    echo "Please fix the failing checks before committing."
    exit 1
fi