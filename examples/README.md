# Ejemplos extraidos del libro

Esta carpeta reune los bloques de codigo `rust` y `r` de las partes, mas los scripts complementarios que hacen runnable un ejemplo mencionado en el texto:

- Rust Espacial
- Integracion con R
- Integracion con DDBB
- Proyectos

Criterios usados:

- Los archivos conservan el bloque tal como aparece en el libro.
- Cada archivo incluye fuente, rango de lineas y seccion de origen.
- Algunos bloques son programas completos y otros son fragmentos didacticos progresivos.
- La estructura esta pensada para que lectores encuentren rapidamente el ejemplo por capitulo.
- Los scripts complementarios cubren ejemplos mencionados por el capitulo, pero no presentes como bloque `rust` o `r` completo.
- Para regenerar los bloques extraidos desde Quarto: `python3 tools/extract_book_examples.py`.

## Como ejecutar los ejemplos

Cada subcarpeta tiene su propio `README.md` con instrucciones concretas. Regla general:

- Si el archivo Rust no usa crates externas y tiene `fn main`, puedes compilarlo con `rustc archivo.rs` y luego ejecutar `./archivo`.
- Si el archivo Rust usa crates externas (`geo`, `h3o`, `geos`, `gdal`, `rusqlite`, etc.), crea un proyecto Cargo temporal, agrega las dependencias indicadas en el README de la carpeta, copia el ejemplo a `src/main.rs` y ejecuta `cargo run`.
- Algunos archivos son fragmentos pedagogicos progresivos del libro. En esos casos el README indica si hay que envolver el bloque en `fn main`, combinarlo con bloques anteriores o usar el archivo de soporte de la carpeta.
- Los ejemplos R se ejecutan con `Rscript archivo.R`, despues de instalar los paquetes R que menciona el README de la carpeta.

Resumen por parte:

## Rust Espacial (55 archivos)

- `rust-espacial/spatial-chapter`: 7 archivos
- `rust-espacial/geotypes`: 10 archivos
- `rust-espacial/formats`: 6 archivos
- `rust-espacial/geos`: 13 archivos
- `rust-espacial/osm`: 2 archivos
- `rust-espacial/geocoding`: 3 archivos
- `rust-espacial/routing`: 4 archivos
- `rust-espacial/geohash`: 3 archivos
- `rust-espacial/h3o`: 7 archivos

## Integracion con R (12 archivos)

- `integracion-con-r/integration`: 4 archivos
- `integracion-con-r/h3o-r`: 8 archivos

## Integracion con DDBB (2 archivos)

- `integracion-con-ddbb/sqlite`: 1 archivos
- `integracion-con-ddbb/duckdb`: 1 archivos

## Proyectos (21 archivos: 20 extraidos + 1 complementario)

- `proyectos/project-1`: 11 archivos
- `proyectos/project-2`: 10 archivos
