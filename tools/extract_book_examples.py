#!/usr/bin/env python3
"""Extrae bloques de código Rust y R desde capítulos Quarto del libro.

Genera una carpeta `examples/` organizada por parte y capítulo, conservando:
- archivo fuente
- rango de líneas original
- jerarquía de encabezados activa al momento del bloque
"""

from __future__ import annotations

import json
import re
import shutil
import unicodedata
from dataclasses import asdict, dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
OUTPUT_DIR = ROOT / "examples"

TARGET_PARTS = [
    {
        "title": "Rust Espacial",
        "slug": "rust-espacial",
        "chapters": [
            "spatial_chapter.qmd",
            "geotypes.qmd",
            "formats.qmd",
            "geos.qmd",
            "osm.qmd",
            "geocoding.qmd",
            "routing.qmd",
            "geohash.qmd",
            "h3o.qmd",
        ],
    },
    {
        "title": "Integracion con R",
        "slug": "integracion-con-r",
        "chapters": [
            "integration.qmd",
            "h3o-R.qmd",
        ],
    },
    {
        "title": "Integracion con DDBB",
        "slug": "integracion-con-ddbb",
        "chapters": [
            "sqlite.qmd",
            "duckdb.qmd",
        ],
    },
    {
        "title": "Proyectos",
        "slug": "proyectos",
        "chapters": [
            "project_1.qmd",
            "project_2.qmd",
        ],
    },
]

LANG_EXTENSIONS = {
    "rust": "rs",
    "r": "R",
}

CODE_FENCE_RE = re.compile(r"^```(?:\{)?\s*(rust|r)\b")
HEADING_RE = re.compile(r"^(#{1,6})\s+(.*)$")
ANCHOR_RE = re.compile(r"\s*\{#.*?\}\s*$")
LEADING_SECTION_RE = re.compile(r"^\d+[\.\)]\s*")


@dataclass
class ExampleBlock:
    part_title: str
    part_slug: str
    chapter_file: str
    chapter_slug: str
    block_index: int
    language: str
    extension: str
    heading_path: list[str]
    source_start_line: int
    source_end_line: int
    output_path: str


def slugify(value: str) -> str:
    normalized = unicodedata.normalize("NFKD", value)
    ascii_only = normalized.encode("ascii", "ignore").decode("ascii")
    slug = re.sub(r"[^a-zA-Z0-9]+", "-", ascii_only).strip("-").lower()
    return slug or "bloque"


def clean_heading(value: str) -> str:
    without_anchor = ANCHOR_RE.sub("", value)
    return without_anchor.strip().strip('"')


def heading_for_filename(value: str) -> str:
    return LEADING_SECTION_RE.sub("", value).strip()


def build_header(block: ExampleBlock) -> str:
    heading = " > ".join(block.heading_path) if block.heading_path else "Sin encabezado cercano"

    if block.extension == "rs":
        prefix = "//"
    else:
        prefix = "#"

    return "\n".join(
        [
            f"{prefix} Fuente: {block.chapter_file}:{block.source_start_line}-{block.source_end_line}",
            f"{prefix} Parte: {block.part_title}",
            f"{prefix} Seccion: {heading}",
            f"{prefix} Bloque: {block.block_index:02d}",
            "",
        ]
    )


def extract_examples(part_title: str, part_slug: str, chapter_name: str) -> list[ExampleBlock]:
    chapter_path = ROOT / chapter_name
    lines = chapter_path.read_text(encoding="utf-8").splitlines()
    chapter_slug = slugify(chapter_path.stem)
    chapter_dir = OUTPUT_DIR / part_slug / chapter_slug
    chapter_dir.mkdir(parents=True, exist_ok=True)

    headings: list[tuple[int, str]] = []
    examples: list[ExampleBlock] = []
    inside_front_matter = False
    front_matter_complete = False
    line_index = 0
    block_index = 0

    while line_index < len(lines):
        raw_line = lines[line_index]
        line_no = line_index + 1

        if line_no == 1 and raw_line.strip() == "---":
            inside_front_matter = True
            line_index += 1
            continue

        if inside_front_matter:
            if raw_line.strip() == "---":
                inside_front_matter = False
                front_matter_complete = True
            line_index += 1
            continue

        heading_match = HEADING_RE.match(raw_line)
        if heading_match:
            level = len(heading_match.group(1))
            title = clean_heading(heading_match.group(2))
            headings = [item for item in headings if item[0] < level]
            headings.append((level, title))
            line_index += 1
            continue

        fence_match = CODE_FENCE_RE.match(raw_line)
        if fence_match:
            language = fence_match.group(1).lower()
            extension = LANG_EXTENSIONS[language]
            block_lines: list[str] = []
            start_line = line_no
            line_index += 1

            while line_index < len(lines) and not lines[line_index].startswith("```"):
                block_lines.append(lines[line_index])
                line_index += 1

            end_line = line_index + 1 if line_index < len(lines) else len(lines)
            block_index += 1

            current_heading_path = [title for _, title in headings]
            nearest_heading = current_heading_path[-1] if current_heading_path else chapter_path.stem
            file_name = f"{block_index:02d}-{slugify(heading_for_filename(nearest_heading))}.{extension}"
            output_file = chapter_dir / file_name

            block = ExampleBlock(
                part_title=part_title,
                part_slug=part_slug,
                chapter_file=chapter_name,
                chapter_slug=chapter_slug,
                block_index=block_index,
                language=language,
                extension=extension,
                heading_path=current_heading_path,
                source_start_line=start_line,
                source_end_line=end_line,
                output_path=str(output_file.relative_to(ROOT)),
            )

            header = build_header(block)
            body = "\n".join(block_lines).rstrip() + "\n"
            output_file.write_text(header + body, encoding="utf-8")
            examples.append(block)

            if line_index < len(lines) and lines[line_index].startswith("```"):
                line_index += 1
            continue

        line_index += 1

    chapter_readme = chapter_dir / "README.md"
    lines_out = [
        f"# {chapter_path.stem}",
        "",
        f"Fuente: `{chapter_name}`",
        "",
        "Archivos generados:",
        "",
    ]
    for block in examples:
        heading = " > ".join(block.heading_path) if block.heading_path else "Sin encabezado cercano"
        lines_out.append(
            f"- `{Path(block.output_path).name}` | `{block.language}` | líneas {block.source_start_line}-{block.source_end_line} | {heading}"
        )
    lines_out.append("")
    chapter_readme.write_text("\n".join(lines_out), encoding="utf-8")

    return examples


def write_root_readme(examples: list[ExampleBlock]) -> None:
    grouped: dict[str, dict[str, int]] = {}
    for block in examples:
        grouped.setdefault(block.part_title, {})
        grouped[block.part_title].setdefault(block.chapter_slug, 0)
        grouped[block.part_title][block.chapter_slug] += 1

    lines = [
        "# Ejemplos extraidos del libro",
        "",
        "Esta carpeta reune los bloques de codigo `rust` y `r` de las partes:",
        "",
        "- Rust Espacial",
        "- Integracion con R",
        "- Integracion con DDBB",
        "- Proyectos",
        "",
        "Criterios usados:",
        "",
        "- Los archivos conservan el bloque tal como aparece en el libro.",
        "- Cada archivo incluye fuente, rango de lineas y seccion de origen.",
        "- Algunos bloques son programas completos y otros son fragmentos didacticos progresivos.",
        "- La estructura esta pensada para que lectores encuentren rapidamente el ejemplo por capitulo.",
        "- Para regenerar la carpeta completa: `python3 tools/extract_book_examples.py`.",
        "",
        "Resumen por parte:",
        "",
    ]

    for part in TARGET_PARTS:
        part_examples = grouped.get(part["title"], {})
        total = sum(part_examples.values())
        lines.append(f"## {part['title']} ({total} archivos)")
        lines.append("")
        for chapter_name in part["chapters"]:
            chapter_slug = slugify(Path(chapter_name).stem)
            count = part_examples.get(chapter_slug, 0)
            lines.append(f"- `{part['slug']}/{chapter_slug}`: {count} archivos")
        lines.append("")

    (OUTPUT_DIR / "README.md").write_text("\n".join(lines), encoding="utf-8")


def main() -> None:
    if OUTPUT_DIR.exists():
        shutil.rmtree(OUTPUT_DIR)
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    manifest: list[ExampleBlock] = []
    for part in TARGET_PARTS:
        for chapter_name in part["chapters"]:
            manifest.extend(extract_examples(part["title"], part["slug"], chapter_name))

    write_root_readme(manifest)
    manifest_path = OUTPUT_DIR / "manifest.json"
    manifest_path.write_text(
        json.dumps([asdict(item) for item in manifest], ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    print(f"Generados {len(manifest)} archivos de ejemplo en {OUTPUT_DIR.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
