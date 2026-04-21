# spatial_chapter

Fuente: `spatial_chapter.qmd`

## Como ejecutar

Esta carpeta mezcla fragmentos conceptuales y programas completos. Para un archivo Rust con `fn main`, usa Cargo si depende de crates externas:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-spatial-chapter
cd /tmp/rust-espacial-spatial-chapter
```

Agrega al `Cargo.toml` las dependencias que necesite el ejemplo:

```toml
[dependencies]
geo = "0.26"
geo-types = "0.7"
geohash = "0.14"
rayon = "1"
```

Luego copia y ejecuta el archivo:

```bash
cp "$ORIG/03-distancia-haversine-vs-euclidiana.rs" src/main.rs
cargo run
```

Los archivos sin `fn main` son fragmentos del capitulo: pegalos dentro de una funcion `main` o completalos con los datos que el texto describe. El ejemplo R se ejecuta con:

```bash
Rscript 04-pipelines-de-procesamiento.R
```

Archivos generados:

- `01-puntos-y-coordenadas.rs` | `rust` | líneas 38-45 | Rust Espacial > Objetos Espaciales > Puntos y Coordenadas
- `02-poligonos.rs` | `rust` | líneas 53-61 | Rust Espacial > Objetos Espaciales > Polígonos
- `03-distancia-haversine-vs-euclidiana.rs` | `rust` | líneas 77-95 | Rust Espacial > Operaciones Espaciales > Distancia (Haversine vs Euclidiana)
- `04-pipelines-de-procesamiento.R` | `r` | líneas 115-117 | Rust Espacial > Error silencioso frecuente > Pipelines de Procesamiento
- `05-pipelines-de-procesamiento.rs` | `rust` | líneas 120-124 | Rust Espacial > Error silencioso frecuente > Pipelines de Procesamiento
- `06-el-problema-del-vecino.rs` | `rust` | líneas 141-155 | Rust Espacial > Caso Práctico: Geohash > El problema del Vecino
- `07-ingenieria-de-sistemas-espaciales.rs` | `rust` | líneas 174-180 | Rust Espacial > Ingeniería de Sistemas Espaciales
