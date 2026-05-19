from __future__ import annotations

import re
from pathlib import Path


HEADER_RE = re.compile(r"^[<>] \d{4}/\d{2}/\d{2} ")


def beautify_dump(input_path: Path) -> str:
    extracted: list[str] = []

    with input_path.open("r", encoding="utf-8", errors="ignore") as handle:
        for line in handle:
            if HEADER_RE.match(line):
                extracted.append(f"\n\n{'=' * 60}\n{line.strip()}\n{'=' * 60}\n")
                continue

            if line.startswith("--") or not line.startswith(" "):
                continue

            parts = line.split("  ")
            if len(parts) <= 1:
                continue

            text_segment = parts[-1].rstrip("\n")
            text_panel = text_segment[:16]
            cleaned_chars: list[str] = []

            for char in text_panel:
                if char.isalnum() or char in '{"}:,[]-_ ':
                    cleaned_chars.append(char)
                elif char == "." and cleaned_chars and cleaned_chars[-1].isalnum():
                    cleaned_chars.append(char)

            if cleaned_chars:
                extracted.append("".join(cleaned_chars))

    return "".join(extracted)


def write_beautified_dump(input_path: Path, output_path: Path) -> None:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(beautify_dump(input_path), encoding="utf-8")