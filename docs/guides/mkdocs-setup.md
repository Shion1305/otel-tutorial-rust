# MkDocs Website Setup Guide

This documentation site is built with MkDocs and the Material theme, managed with `uv` for Python dependencies.

## Overview

- **Documentation Framework:** MkDocs
- **Theme:** Material for MkDocs
- **Dependency Manager:** uv (fast Python package manager)
- **Deployment:** GitHub Pages (automatic with GitHub Actions)
- **Python Version:** 3.12

## Quick Start

### 1. Prerequisites

- Python 3.8+ installed
- `uv` package manager installed ([install uv](https://docs.astral.sh/uv/getting-started/installation/))

### 2. Install Dependencies with uv

```bash
# Install uv (if not already installed)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install documentation dependencies
uv sync

# Or install specific packages
uv pip install mkdocs mkdocs-material pymdown-extensions
```

### 3. Serve Documentation Locally

```bash
# Build and serve documentation
mkdocs serve

# Then visit http://localhost:8000
```

### 4. Build Documentation

```bash
# Generate static HTML
mkdocs build

# Output is in the 'site/' directory
```

## File Structure

```
otel-tutorial-rust/
├── docs/                          # Documentation source
│   ├── index.md                   # Home page
│   ├── assets/                    # Images, stylesheets
│   │   ├── logo.png
│   │   ├── favicon.ico
│   │   └── stylesheets/
│   │       └── extra.css
│   ├── getting-started/           # Getting Started section
│   │   ├── index.md
│   │   └── quickstart.md
│   ├── guides/                    # Learning guides
│   │   ├── architecture.md
│   │   ├── tech-stack.md
│   │   ├── onboarding.md
│   │   ├── k6-testing.md
│   │   ├── k6-summary.md
│   │   └── mkdocs-setup.md        # This file
│   ├── reference/                 # Quick reference
│   │   ├── commands.md
│   │   ├── project-contents.md
│   │   └── troubleshooting.md
│   └── api/                       # API documentation
│       ├── endpoints.md
│       └── examples.md
├── mkdocs.yml                     # MkDocs configuration
├── pyproject.toml                 # Python project configuration
├── .python-version                # Python version specification
└── site/                          # Generated website (don't edit)
    ├── index.html
    ├── css/
    ├── js/
    └── ...
```

## Configuration Files

### mkdocs.yml

Main configuration file for the documentation:

```yaml
site_name: OpenTelemetry + Rust + K6 Tutorial
site_description: A comprehensive, production-ready tutorial...
site_author: Your Name
site_url: https://yourusername.github.io/otel-tutorial-rust/

theme:
  name: material
  features:
    - navigation.instant      # Instant loading of pages
    - navigation.tracking     # URL updates on scroll
    - navigation.sections     # Collapsible sections
    - toc.integrate          # TOC in sidebar

plugins:
  - search                    # Full-text search

markdown_extensions:
  - admonition               # !!! note blocks
  - pymdownx.details         # Details/summary blocks
  - pymdownx.superfences     # Code blocks
  - pymdownx.highlight       # Syntax highlighting
  - tables                   # Table support

nav:
  - Home: getting-started/index.md
  - Getting Started:
    - Welcome: getting-started/index.md
    - Quick Start: getting-started/quickstart.md
  - Guides:
    - Architecture: guides/architecture.md
    # ... more guides
  - API Documentation:
    - Endpoints: api/endpoints.md
    - Examples: api/examples.md
```

### pyproject.toml

Declares project dependencies in a standardized format:

```toml
[project]
name = "otel-tutorial-docs"
version = "1.0.0"
description = "OpenTelemetry + Rust + K6 Tutorial Documentation"
requires-python = ">=3.8"

dependencies = [
    "mkdocs>=1.4.0",
    "mkdocs-material>=9.0.0",
    "pymdown-extensions>=10.0.0",
]

[project.optional-dependencies]
dev = [
    "mkdocs-awesome-pages-plugin>=2.9.0",
]
```

### .python-version

Specifies the Python version to use:

```
3.12.0
```

## Using uv for Dependency Management

### Installing Dependencies

```bash
# Install from pyproject.toml
uv sync

# Install all dependencies including dev
uv sync --all-extras
```

### Adding New Dependencies

```bash
# Add a dependency
uv add mkdocs-plugin-name

# Add a dev dependency
uv add --dev mkdocs-plugin-name
```

### Updating Dependencies

```bash
# Update all dependencies
uv sync --upgrade

# Update specific package
uv pip install --upgrade mkdocs-material
```

### Creating Virtual Environment

```bash
# Create virtual environment
uv venv

# Activate (on macOS/Linux)
source .venv/bin/activate

# Activate (on Windows)
.venv\Scripts\activate
```

## Material Theme Features

### Navigation Features

- **Instant Loading:** Pages load instantly without full page refresh
- **Section Navigation:** Collapsible sections in the sidebar
- **TOC Integration:** Table of contents integrated into the sidebar
- **Breadcrumbs:** Shows current location in navigation

### Content Features

- **Code Highlighting:** Syntax highlighting for multiple languages
- **Code Copy Button:** One-click code block copying
- **Annotations:** Highlight specific code lines
- **Tables:** Enhanced table support with sorting
- **Admonitions:** Note, warning, danger blocks

### Color Scheme

- **Light Mode:** Default light theme
- **Dark Mode:** Automatic dark theme toggle
- **Custom Colors:** Indigo primary color, indigo accent

## Markdown Extensions

### Admonitions (Information Blocks)

```markdown
!!! note "Title"
    This is a note block.

!!! warning "Warning"
    This is a warning block.

!!! danger "Danger"
    This is a danger block.
```

### Details/Summary (Collapsible Blocks)

```markdown
??? note "Click to expand"
    Hidden content here.
```

### Code Blocks

```python
def hello():
    print("Hello, World!")
```

### Tables

```markdown
| Column 1 | Column 2 |
|----------|----------|
| Data 1   | Data 2   |
```

## Customization

### Custom CSS

Create `docs/assets/stylesheets/extra.css`:

```css
:root {
  --md-primary-fg-color: #1976d2;
  --md-accent-fg-color: #00bcd4;
}
```

Then add to `mkdocs.yml`:

```yaml
extra_css:
  - assets/stylesheets/extra.css
```

### Custom JavaScript

Create `docs/assets/javascripts/extra.js`:

```javascript
document.addEventListener('DOMContentLoaded', () => {
  console.log('Documentation loaded!');
});
```

Then add to `mkdocs.yml`:

```yaml
extra_javascript:
  - assets/javascripts/extra.js
```

### Logo and Favicon

Place your images in `docs/assets/`:

```yaml
theme:
  logo: assets/logo.png
  favicon: assets/favicon.ico
```

## Building and Serving

### Local Development

```bash
# Serve with auto-reload
mkdocs serve

# Serves at http://localhost:8000
# Hot-reloads on file changes
```

### Production Build

```bash
# Build static site
mkdocs build

# Output in 'site/' directory
# Ready to deploy to any static hosting
```

### Building with Specific Site URL

```bash
# Build for custom domain
mkdocs build --site-url "https://docs.yourdomain.com/"

# Build for GitHub Pages subdirectory
mkdocs build --site-url "https://username.github.io/repo/"
```

## Deployment Options

### GitHub Pages (Recommended)

```bash
# 1. Push documentation to GitHub
git add docs/ mkdocs.yml .github/
git commit -m "Update documentation"
git push origin main

# 2. GitHub Actions automatically builds and deploys
# 3. Site available at https://username.github.io/otel-tutorial-rust/
```

### Manual Deployment

```bash
# 1. Build locally
mkdocs build

# 2. Deploy 'site/' directory to any static host
# - GitHub Pages
# - Netlify
# - Vercel
# - AWS S3
# - Any web server
```

### Docker Deployment

```dockerfile
FROM python:3.12-slim

WORKDIR /app

COPY pyproject.toml .python-version ./
RUN pip install -e .

COPY mkdocs.yml ./
COPY docs/ ./docs/

CMD ["mkdocs", "serve", "--dev-addr=0.0.0.0:8000"]
```

## Best Practices

### Navigation Structure

- Keep navigation hierarchy 2-3 levels deep
- Use clear, descriptive titles
- Group related pages together
- Order pages logically (Getting Started → Guides → Reference)

### Content Organization

```
docs/
├── getting-started/     # Quick setup for newcomers
├── guides/             # Deep dives and tutorials
├── reference/          # Quick lookup and API docs
├── api/                # API reference
└── assets/             # Images and stylesheets
```

### Documentation Quality

- Start each page with a clear summary
- Use headings hierarchically (H1 > H2 > H3)
- Add code examples for technical content
- Include links to related pages
- Keep sentences short and clear
- Update documentation with code changes

### File Naming

- Use lowercase filenames
- Use hyphens, not underscores (`my-guide.md`, not `my_guide.md`)
- Match nav structure in mkdocs.yml
- Use descriptive names (`api-endpoints.md`, not `api.md`)

## Troubleshooting

### Port Already in Use

```bash
# Use a different port
mkdocs serve --dev-addr=127.0.0.1:8001
```

### Search Not Working

```bash
# Rebuild search index
mkdocs build
```

### Styling Not Applied

```bash
# Clear cache and rebuild
rm -rf site/
mkdocs build
```

### Dependencies Conflict

```bash
# Recreate virtual environment
rm -rf .venv
uv venv
uv sync
```

### Navigation Broken

Check `mkdocs.yml`:
- All paths must be relative to `docs/`
- Files must exist in specified locations
- YAML indentation must be correct

## Performance Tips

### Build Time

```bash
# Build is fast (usually < 1 second)
mkdocs build

# Serve has instant reload on file changes
mkdocs serve
```

### Search Performance

- Material theme uses client-side search
- Works offline
- Fast, no external service needed

### File Size

- Generated `site/` is compact (~5-10MB for documentation)
- Suitable for any hosting
- Easily fit in GitHub Pages (1GB limit)

## Maintenance

### Regular Tasks

```bash
# Weekly: Rebuild and test locally
mkdocs serve

# Monthly: Update dependencies
uv sync --upgrade

# Per release: Update version
# Update pyproject.toml version
# Update .python-version if needed
```

### Backup

```bash
# Version control everything
git add docs/ mkdocs.yml pyproject.toml .github/
git commit -m "Documentation update"
git push origin main

# GitHub keeps full history
```

## Advanced Topics

### Multiple Languages

```yaml
plugins:
  - i18n:
      default_language: en
      languages:
        en: English
        es: Español
        fr: Français
```

### Versioning Documentation

```bash
# Keep different versions
docs/
├── latest/    # Latest version
├── v1.0/      # Version 1.0
└── v0.9/      # Version 0.9
```

### Custom Plugins

Create `docs_plugins/my_plugin.py` and configure in `mkdocs.yml`.

## Quick Commands

```bash
# Install dependencies
uv sync

# Serve locally
mkdocs serve

# Build static site
mkdocs build

# Deploy to GitHub Pages
git push origin main
# (GitHub Actions handles the rest)

# Add new dependency
uv add package-name

# Update all dependencies
uv sync --upgrade

# Remove all generated files
rm -rf site/ .venv
```

## Resources

- [MkDocs Documentation](https://www.mkdocs.org/)
- [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/)
- [uv Documentation](https://docs.astral.sh/uv/)
- [Markdown Guide](https://www.markdownguide.org/)

## Next Steps

1. [View the live documentation](https://yourusername.github.io/otel-tutorial-rust/)
2. [Customize the theme](#customization)
3. [Add custom content](#markdown-extensions)

---

Your documentation website is ready! Start with `mkdocs serve` to preview locally, then deploy with git.

