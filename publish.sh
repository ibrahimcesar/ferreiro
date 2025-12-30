#!/bin/bash
# Ferreiro v0.0.1 Publishing Script
# Publishes all crates in dependency order

set -e  # Exit on error

echo "🔨 Ferreiro v0.0.1 - Publishing Script"
echo "======================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Crates in dependency order
CRATES=(
    "ferreiro_domain"
    "ferreiro_adapters_session"
    "ferreiro_adapters_templates"
    "ferreiro_adapters_admin"
    "ferreiro_application"
    "ferreiro_adapters_db"
    "ferreiro_adapters_http"
    "ferreiro_cli"
    "ferreiro"
)

echo "Will publish ${#CRATES[@]} crates in this order:"
for crate in "${CRATES[@]}"; do
    echo "  - $crate"
done
echo ""

read -p "Continue? (y/N) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Publish each crate
for i in "${!CRATES[@]}"; do
    crate="${CRATES[$i]}"
    num=$((i+1))

    echo ""
    echo -e "${GREEN}[$num/${#CRATES[@]}] Publishing $crate...${NC}"
    echo "----------------------------------------"

    cd "$crate"

    # Try to publish
    if cargo publish; then
        echo -e "${GREEN}✓ $crate published successfully!${NC}"
    else
        echo -e "${YELLOW}⚠ $crate publish failed or already exists${NC}"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Aborted."
            exit 1
        fi
    fi

    cd ..

    # Wait between publishes (except for the last one)
    if [ $i -lt $((${#CRATES[@]}-1)) ]; then
        echo ""
        echo -e "${YELLOW}Waiting 90 seconds for crates.io to index...${NC}"
        sleep 90
    fi
done

echo ""
echo "======================================"
echo -e "${GREEN}🎉 All crates published!${NC}"
echo ""
echo "Next steps:"
echo "1. Check https://crates.io/crates/ferreiro"
echo "2. Wait for https://docs.rs/ferreiro to build"
echo "3. Create GitHub release"
echo "4. Announce on Reddit, Twitter, etc."
echo ""
echo "See PUBLISH_NOW.md for details."
