# h3o

Fuente: `h3o.qmd`

## Como ejecutar

Estos ejemplos usan `h3o`; los de poligonos tambien usan `geo-types`. Crea un proyecto Cargo temporal:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-h3o
cd /tmp/rust-espacial-h3o
```

En `Cargo.toml`:

```toml
[dependencies]
h3o = { version = "0.6", features = ["geo"] }
geo-types = "0.7"
```

Luego:

```bash
cp "$ORIG/01-indexacion-coordenada-celda.rs" src/main.rs
cargo run
```

Los archivos `02`, `03`, `04`, `05` y `07` son fragmentos que asumen variables creadas antes, como `cell`. Para ejecutarlos, parte desde `01-indexacion-coordenada-celda.rs` y agrega el bloque que quieres probar dentro del mismo `main`.

Archivos generados:

- `01-indexacion-coordenada-celda.rs` | `rust` | líneas 30-48 | Funciones Principales > 1. Indexación (Coordenada ↔ Celda)
- `02-inspeccion.rs` | `rust` | líneas 54-58 | Funciones Principales > 2. Inspección
- `03-jerarquia-zoom-in-zoom-out.rs` | `rust` | líneas 66-78 | Funciones Principales > 3. Jerarquía (Zoom In / Zoom Out)
- `04-recorrido-y-vecindad-grid-traversal.rs` | `rust` | líneas 84-104 | Funciones Principales > 4. Recorrido y Vecindad (Grid Traversal)
- `05-aristas-dirigidas-directed-edges.rs` | `rust` | líneas 110-116 | Funciones Principales > 5. Aristas Dirigidas (Directed Edges)
- `06-regiones-y-poligonos-polyfill.rs` | `rust` | líneas 124-148 | Funciones Principales > 6. Regiones y Polígonos (Polyfill)
- `07-compactacion.rs` | `rust` | líneas 156-171 | Funciones Principales > 7. Compactación
