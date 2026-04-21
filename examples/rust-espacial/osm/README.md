# osm

Fuente: `osm.qmd`

## Como ejecutar

Estos ejemplos usan la crate `osmpbfreader` y esperan un archivo `.pbf` de OpenStreetMap. Crea un proyecto Cargo temporal:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-osm
cd /tmp/rust-espacial-osm
```

En `Cargo.toml`:

```toml
[dependencies]
osmpbfreader = "0.16"
```

Copia el ejemplo y ejecuta:

```bash
cp "$ORIG/01-ejemplo-contando-nodos.rs" src/main.rs
cargo run
```

Si el codigo referencia una ruta como `data/chile-latest.osm.pbf`, cambiala por la ruta real de tu archivo PBF antes de ejecutar.

Archivos generados:

- `01-ejemplo-contando-nodos.rs` | `rust` | líneas 18-39 | OpenStreetMap (OSM) > Lectura de Archivos PBF > Ejemplo: Contando Nodos
- `02-ejemplo-filtrado-y-extraccion.rs` | `rust` | líneas 45-78 | OpenStreetMap (OSM) > Lectura de Archivos PBF > Ejemplo: Filtrado y Extracción
