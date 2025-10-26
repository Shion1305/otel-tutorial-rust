# MkDocs Documentation Website

This project includes a complete MkDocs website with comprehensive documentation for the OpenTelemetry + Rust + K6 tutorial.

## Quick Start

### Option 1: View Online
The documentation website is automatically deployed to GitHub Pages. Visit:
```
https://yourusername.github.io/otel-tutorial-rust/
```

### Option 2: Serve Locally
```bash
# Install dependencies with uv
uv sync

# Or with pip
pip install mkdocs mkdocs-material pymdown-extensions

# Serve documentation locally
mkdocs serve

# Then visit http://localhost:8000
```

## Build Documentation

```bash
# Build static HTML files
mkdocs build

# Output is in the 'site/' directory
```

## Project Structure

```
otel-tutorial-rust/
├── docs/                          # Documentation source files
│   ├── index.md                   # Home page
│   ├── getting-started/
│   │   ├── index.md              # Getting started guide
│   │   └── quickstart.md          # 5-minute quick start
│   ├── guides/                    # Detailed guides
│   │   ├── architecture.md
│   │   ├── tech-stack.md
│   │   ├── onboarding.md
│   │   ├── k6-testing.md
│   │   ├── k6-summary.md
│   │   └── mkdocs-setup.md        # This documentation
│   ├── reference/                 # Quick reference
│   │   ├── commands.md
│   │   ├── project-contents.md
│   │   └── troubleshooting.md
│   ├── api/                       # API documentation
│   │   ├── endpoints.md
│   │   └── examples.md
│   └── assets/                    # Images and stylesheets
├── mkdocs.yml                     # MkDocs configuration
├── pyproject.toml                 # Python project configuration
├── .python-version                # Python version
└── site/                          # Generated website (build output)
```

## Configuration

### mkdocs.yml
Main configuration file. Contains:
- Site metadata (name, description, URL)
- Theme configuration (Material theme with custom colors)
- Navigation structure
- Markdown extensions

### pyproject.toml
Python project configuration:
- Project metadata
- Dependencies (mkdocs, mkdocs-material, pymdown-extensions)
- Python version requirements

### .python-version
Specifies Python 3.12 for consistency.

## Automatic Deployment to GitHub Pages

The `.github/workflows/deploy-docs.yml` workflow automatically:
1. Builds the documentation when changes are pushed to `main`
2. Deploys the built site to GitHub Pages
3. Makes it available at `https://yourusername.github.io/otel-tutorial-rust/`

## Features

- **Material Theme**: Professional, mobile-responsive design
- **Full-Text Search**: Client-side search across all documentation
- **Code Highlighting**: Syntax highlighting for multiple languages
- **Responsive Design**: Looks great on desktop and mobile
- **Navigation**: Clear hierarchy and breadcrumb navigation
- **Dark Mode**: Automatic dark mode toggle
- **Markdown Extensions**: Support for admonitions, details, tables, and more

## Customization

### Add New Pages
1. Create a new `.md` file in the `docs/` directory
2. Add it to the navigation in `mkdocs.yml`
3. Rebuild: `mkdocs build`

### Customize Theme
Edit `mkdocs.yml`:
```yaml
theme:
  palette:
    - scheme: default
      primary: indigo
      accent: indigo
```

### Add Custom CSS
Create `docs/assets/stylesheets/extra.css` and update `mkdocs.yml`:
```yaml
extra_css:
  - assets/stylesheets/extra.css
```

## Common Tasks

```bash
# Serve locally with hot-reload
mkdocs serve

# Build for production
mkdocs build

# Clean build
rm -rf site/ && mkdocs build

# Update dependencies
uv sync --upgrade
```

## Deployment

### GitHub Pages (Recommended)
1. Ensure `.github/workflows/deploy-docs.yml` exists
2. Push changes to main branch
3. GitHub Actions automatically builds and deploys
4. Access at `https://yourusername.github.io/otel-tutorial-rust/`

### Manual Deployment
```bash
# 1. Build locally
mkdocs build

# 2. Deploy the 'site/' directory to your hosting
# - GitHub Pages
# - Netlify
# - Vercel
# - Any static host
```

## For More Information

See `docs/guides/mkdocs-setup.md` for comprehensive documentation about:
- Setting up and configuring MkDocs
- Using the Material theme
- Customizing appearance and behavior
- Advanced features and plugins
- Best practices for documentation

## Contributing

When adding content:
1. Follow the existing structure
2. Use clear, descriptive titles
3. Include code examples
4. Add links to related pages
5. Keep sentences short and clear
6. Update `mkdocs.yml` navigation

## Troubleshooting

### Port Already in Use
```bash
mkdocs serve --dev-addr=127.0.0.1:8001
```

### Dependencies Not Installing
```bash
rm -rf .venv
uv venv
uv sync
```

### Search Not Working
```bash
rm -rf site/
mkdocs build
```

## Resources

- [MkDocs Documentation](https://www.mkdocs.org/)
- [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/)
- [Markdown Guide](https://www.markdownguide.org/)

---

**Next:** Visit the [documentation home page](docs/getting-started/index.md) to get started!

