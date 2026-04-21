# duckdb

Fuente: `duckdb.qmd`

## Como ejecutar

Este ejemplo usa DuckDB con la extension espacial. Ejecutalo con Cargo:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-duckdb
cd /tmp/rust-espacial-duckdb
```

En `Cargo.toml`:

```toml
[dependencies]
duckdb = { version = "0.9.0", features = ["bundled"] }
geo = "0.26.0"
wkt = "0.10.0"
```

Luego:

```bash
cp "$ORIG/01-ejemplo-insercion-de-objetos-espaciales.rs" src/main.rs
cargo run
```

La primera compilacion puede tardar porque `duckdb` con `bundled` compila/descarga componentes nativos.

Archivos generados:

- `01-ejemplo-insercion-de-objetos-espaciales.rs` | `rust` | líneas 29-91 | El Poder de `spatial` Extension > Ejemplo: Inserción de Objetos Espaciales
