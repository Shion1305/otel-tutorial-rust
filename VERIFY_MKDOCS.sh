#!/bin/bash

echo "═══════════════════════════════════════════════════════════════════"
echo "    MkDocs Documentation Website - Verification Report"
echo "═══════════════════════════════════════════════════════════════════"
echo ""

echo "📋 Configuration Files"
echo "───────────────────────────────────────────────────────────────────"
test -f mkdocs.yml && echo "✓ mkdocs.yml - MkDocs configuration" || echo "✗ mkdocs.yml"
test -f pyproject.toml && echo "✓ pyproject.toml - Python project config" || echo "✗ pyproject.toml"
test -f .python-version && echo "✓ .python-version - Python 3.12" || echo "✗ .python-version"
test -f .github/workflows/deploy-docs.yml && echo "✓ .github/workflows/deploy-docs.yml - GitHub Actions" || echo "✗ GitHub Actions"
echo ""

echo "📚 Documentation Structure"
echo "───────────────────────────────────────────────────────────────────"
echo "Getting Started:"
test -f docs/getting-started/index.md && echo "  ✓ Welcome guide" || echo "  ✗ Welcome guide"
test -f docs/getting-started/quickstart.md && echo "  ✓ 5-minute quickstart" || echo "  ✗ Quickstart"
echo ""

echo "Learning Guides:"
test -f docs/guides/architecture.md && echo "  ✓ Architecture reference" || echo "  ✗ Architecture"
test -f docs/guides/tech-stack.md && echo "  ✓ Technology stack" || echo "  ✗ Tech stack"
test -f docs/guides/onboarding.md && echo "  ✓ Onboarding path" || echo "  ✗ Onboarding"
test -f docs/guides/k6-testing.md && echo "  ✓ K6 load testing" || echo "  ✗ K6 testing"
test -f docs/guides/k6-summary.md && echo "  ✓ K6 integration summary" || echo "  ✗ K6 summary"
test -f docs/guides/mkdocs-setup.md && echo "  ✓ MkDocs setup guide" || echo "  ✗ MkDocs setup"
echo ""

echo "Reference:"
test -f docs/reference/commands.md && echo "  ✓ Commands & URLs" || echo "  ✗ Commands"
test -f docs/reference/project-contents.md && echo "  ✓ Project contents" || echo "  ✗ Project contents"
test -f docs/reference/troubleshooting.md && echo "  ✓ Troubleshooting" || echo "  ✗ Troubleshooting"
echo ""

echo "API Documentation:"
test -f docs/api/endpoints.md && echo "  ✓ API endpoints" || echo "  ✗ Endpoints"
test -f docs/api/examples.md && echo "  ✓ API examples" || echo "  ✗ Examples"
echo ""

echo "📊 Site Generation"
echo "───────────────────────────────────────────────────────────────────"
if [ -d "site/" ]; then
    FILE_COUNT=$(find site -type f | wc -l)
    SIZE=$(du -sh site | cut -f1)
    echo "✓ Site directory exists"
    echo "  Files: $FILE_COUNT"
    echo "  Size: $SIZE"
else
    echo "✗ Site directory not found (run 'mkdocs build')"
fi
echo ""

echo "🔧 Dependencies"
echo "───────────────────────────────────────────────────────────────────"
if [ -d "venv/" ]; then
    echo "✓ Virtual environment exists"
    MKDOCS_VERSION=$(./venv/bin/mkdocs --version 2>/dev/null)
    if [ -n "$MKDOCS_VERSION" ]; then
        echo "  $MKDOCS_VERSION"
    fi
else
    echo "ℹ Virtual environment not found (create with: python3 -m venv venv)"
fi
echo ""

echo "📝 Setup Guides"
echo "───────────────────────────────────────────────────────────────────"
test -f DOCS_SETUP.md && echo "✓ DOCS_SETUP.md - Documentation setup guide" || echo "✗ DOCS_SETUP.md"
test -f MKDOCS_SUMMARY.md && echo "✓ MKDOCS_SUMMARY.md - Complete summary" || echo "✗ MKDOCS_SUMMARY.md"
echo ""

echo "═══════════════════════════════════════════════════════════════════"
echo "✨ Quick Start Commands"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "1. Serve documentation locally:"
echo "   mkdocs serve"
echo "   Then visit: http://localhost:8000"
echo ""
echo "2. Build static site:"
echo "   mkdocs build"
echo ""
echo "3. Deploy to GitHub Pages:"
echo "   git add ."
echo "   git commit -m 'Add MkDocs documentation'"
echo "   git push origin main"
echo ""
echo "4. View documentation:"
echo "   https://yourusername.github.io/otel-tutorial-rust/"
echo ""
echo "═══════════════════════════════════════════════════════════════════"

