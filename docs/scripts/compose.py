#!/usr/bin/env python3
"""
compose.py — Assemble the MKTd03 integration guide from local docs/sections/.

Usage:
    python3 docs/scripts/compose.py [manifest]

Arguments:
    manifest    Path to compose.yaml (default: docs/compose.yaml)

Output:
    Writes the composed markdown file to the repo root (filename from manifest).

Dependencies:
    PyYAML (pip install pyyaml)
"""

import os
import sys
import yaml


def read_section(filepath: str) -> str:
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()

    if not content.endswith("\n"):
        content += "\n"
    return content.lstrip("\n")


def strip_first_heading(content: str) -> str:
    lines = content.split("\n")
    for i, line in enumerate(lines):
        if line.startswith("## ") or line.startswith("### ") or line.startswith("#### "):
            lines.pop(i)
            if i < len(lines) and lines[i].strip() == "":
                lines.pop(i)
            return "\n".join(lines)
        if line.startswith("# "):
            return content
    return content


def main() -> int:
    manifest_path = sys.argv[1] if len(sys.argv) > 1 else "docs/compose.yaml"
    sections_dir = os.path.join(os.path.dirname(manifest_path), "sections")

    with open(manifest_path, "r", encoding="utf-8") as f:
        manifest = yaml.safe_load(f)

    output_path = manifest["output"]
    parts = [
        "<!-- GENERATED FILE — do not edit directly. Source: docs/sections/ + docs/compose.yaml. See README.md for rebuild instructions. -->\n\n"
    ]

    for section in manifest["sections"]:
        content = read_section(os.path.join(sections_dir, section["local"]))
        if section.get("strip_heading", False):
            content = strip_first_heading(content)
        if "heading" in section:
            parts.append(f"\n---\n\n{section['heading']}\n\n")
        if "preamble" in section:
            parts.append(f"{section['preamble']}\n\n")
        parts.append(content)
        if not content.endswith("\n"):
            parts.append("\n")

    with open(output_path, "w", encoding="utf-8") as f:
        f.write("".join(parts))

    print(f"Wrote {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
