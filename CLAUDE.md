# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **Quarto documentation project** for a Spanish-language book titled "Rust Espacial" about spatial analysis using Rust. The book teaches R users how to leverage Rust for high-performance geospatial operations.

## Building the Book

```bash
quarto render                    # Build all chapters
quarto render index.qmd          # Build a single chapter
quarto preview                   # Live preview at localhost:4444
```

## Key Dependencies

The book focuses on these Rust crates for spatial operations:

| Crate | Purpose |
|-------|---------|
| `geo-types` | Fundamental geometric primitives (Point, Polygon, LineString) |
| `geo` | Geometric algorithms (area, distance, intersection) |
| `geos` | Bindings to GEOS C++ library (buffer, convex_hull, spatial predicates) |
| `h3o` | H3 hexagonal indexing system (pure Rust implementation) |
| `gdal` | GDAL bindings for raster/vector I/O |
| `extendr` / `rextendr` | Rust-R integration |

## Common Workflow

1. Edit a `.qmd` chapter file
2. Use `quarto preview` to see changes in real-time
3. Run `quarto render` to build the final HTML book in `_book/`

## Book Structure

```
_index.qmd          # Book homepage
_intro.qmd          # Introduction
_install.qmd        # Installation instructions
[Basic Concepts]
  _variables.qmd, _flow_control.qmd, _scope.qmd, _funcions.qmd
  _macros.qmd, _iterators.qmd, _closures.qmd, _errors.qmd
  _struct.qmd, _traits.qmd, _impl.qmd, _ownership.qmd
[Optimization]
  _optimization.qmd, _paralelization.qmd
[Spatial Rust]
  _spatial_chapter.qmd, _geotypes.qmd, _formats.qmd
  _geos.qmd, _osm.qmd, _geocoding.qmd, _routing.qmd
  _geohash.qmd, _h3o.qmd
[R Integration]
  _integration.qmd, _h3o-R.qmd
[Database Integration]
  _sqlite.qmd, _duckdb.qmd
[Projects]
  _project_1.qmd, _project_2.qmd
[Appendices]
  _functional.qmd
```

## Notable Chapters

- **project_1.qmd**: First practical project implementing spatial structures from scratch (Point, Polygon, SpatialObject trait)
- **spatial_chapter.qmd**: Overview of Rust in geospatial infrastructure and the GeoRust ecosystem
- **integration.qmd**: Using extendr/rextendr to call Rust functions from R
- **geos.qmd**: GEOS library operations (buffer, convex_hull, spatial predicates)
- **h3o.qmd**: H3 hexagonal indexing system

## Terminología del libro (ISSUE-0030)

Política terminológica obligatoria para todo el libro:

| Término | Cuándo usarlo |
|---|---|
| `crate` | Paquetes del ecosistema Rust (`geo`, `h3o`, `geos`, `gdal`, etc.) |
| "paquete" | Paquetes del ecosistema R (`sf`, `terra`, `dplyr`, etc.) |
| "biblioteca" | Librerías C/C++ subyacentes (GDAL, GEOS, PROJ) |
| ~~"librería"~~ | **Nunca** como sinónimo de crate ni de paquete R |

Primera mención de "crate" en el libro: install.qmd L268 — ya tiene nota definitoria.

## Output

The rendered book is output to `_book/` directory as static HTML.
