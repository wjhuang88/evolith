#!/usr/bin/env python3
"""Fail when changed Markdown contains a broken local relative file link."""

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import unquote

REPO_ROOT = Path(__file__).resolve().parents[2]
SKIP_DIRS = {".git", "node_modules", "target", "dist", "backups"}
LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")


def all_markdown_files() -> list[Path]:
    return sorted(
        path
        for path in REPO_ROOT.rglob("*.md")
        if not any(part in SKIP_DIRS for part in path.relative_to(REPO_ROOT).parts)
    )


def markdown_files() -> list[Path]:
    base = os.environ.get("MARKDOWN_LINK_BASE", "").strip()
    if not base:
        return all_markdown_files()

    result = subprocess.run(
        [
            "git",
            "diff",
            "--name-only",
            "--diff-filter=ACMR",
            f"{base}...HEAD",
            "--",
            "*.md",
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    files: list[Path] = []
    for raw_path in result.stdout.splitlines():
        candidate = REPO_ROOT / raw_path
        if candidate.is_file() and candidate.suffix.lower() == ".md":
            files.append(candidate)
    return sorted(files)


def normalize_destination(raw: str) -> str:
    destination = raw.strip()
    if destination.startswith("<") and ">" in destination:
        destination = destination[1 : destination.index(">")]
    elif " " in destination:
        destination = destination.split(" ", 1)[0]
    return unquote(destination)


def main() -> int:
    failures: list[str] = []
    files = markdown_files()
    for markdown in files:
        text = markdown.read_text(encoding="utf-8")
        for line_number, line in enumerate(text.splitlines(), start=1):
            for match in LINK_RE.finditer(line):
                destination = normalize_destination(match.group(1))
                if not destination or destination.startswith("#"):
                    continue
                lowered = destination.lower()
                if lowered.startswith(("http://", "https://", "mailto:", "tel:", "data:")):
                    continue
                path_part = destination.split("#", 1)[0].split("?", 1)[0]
                if not path_part or path_part.startswith("/"):
                    # Root-absolute destinations are product/site routes, not repository files.
                    continue
                target = markdown.parent / path_part
                if not target.exists():
                    failures.append(
                        f"{markdown.relative_to(REPO_ROOT)}:{line_number}: "
                        f"broken local link {destination!r}"
                    )

    if failures:
        print("Markdown link validation failed:", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    scope = "changed" if os.environ.get("MARKDOWN_LINK_BASE") else "repository"
    print(f"Markdown link validation passed for {len(files)} {scope} file(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
