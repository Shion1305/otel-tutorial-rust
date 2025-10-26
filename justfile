# OpenTelemetry + Rust + K6 Tutorial - Just recipes

# Default recipe - show help
default:
    @just --list

# Install documentation dependencies
docs-install:
    uv sync

# Preview documentation locally with auto-reload
docs-serve:
    uv run mkdocs serve

# Build documentation for production (with strict link checking)
docs-build:
    uv run mkdocs build --strict

# Clean built documentation
docs-clean:
    rm -rf site/
    rm -rf .venv/

# Rebuild documentation from scratch (with strict link checking)
docs-rebuild: docs-clean docs-build
    @echo "Documentation rebuilt in site/"

# Check for broken links in documentation (alias for docs-build)
docs-check:
    uv run mkdocs build --strict
