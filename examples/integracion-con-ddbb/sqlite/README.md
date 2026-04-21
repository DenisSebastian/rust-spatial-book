# sqlite

Fuente: `sqlite.qmd`

## Como ejecutar

Este ejemplo usa `rusqlite`, `geo` y `wkt`, asi que debe correrse con Cargo:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-sqlite
cd /tmp/rust-espacial-sqlite
```

En `Cargo.toml`:

```toml
[dependencies]
rusqlite = "0.29.0"
geo = "0.26.0"
wkt = "0.10.0"
```

Luego:

```bash
cp "$ORIG/01-ejemplo-guardando-puntos-y-poligonos.rs" src/main.rs
cargo run
```

Archivos generados:

- `01-ejemplo-guardando-puntos-y-poligonos.rs` | `rust` | líneas 25-69 | SQLite en Rust > La Crate `rusqlite` > Ejemplo: Guardando Puntos y Polígonos
