#!/usr/bin/env python3
"""Fail when repository Markdown contains a broken local file link."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote

REPO_ROOT = Path(__file__).resolve().parents[2]
SKIP_DIRS = {".git", "node_modules", "target", "dist", "backups"}
LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")


def markdown_files() -> list[Path]:
    return sorted(
        path
        for path in REPO_ROOT.rglob("*.md")
        if not any(part in SKIP_DIRS for part in path.relative_to(REPO_ROOT).parts)
    )


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
                if not path_part:
                    continue
                if path_part.startswith("/"):
                    target = REPO_ROOT / path_part.lstrip("/")
                else:
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

    print(f"Markdown link validation passed for {len(files)} file(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
