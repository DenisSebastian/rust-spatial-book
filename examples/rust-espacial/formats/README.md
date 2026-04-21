# formats

Fuente: `formats.qmd`

## Como ejecutar

Estos ejemplos necesitan Cargo porque usan GDAL/PROJ. Tambien requieren que las bibliotecas nativas de GDAL y PROJ esten instaladas en el sistema.

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-formats
cd /tmp/rust-espacial-formats
```

En `Cargo.toml`, agrega solo lo que necesite el archivo que vas a probar:

```toml
[dependencies]
gdal = "0.16"
proj = "0.27"
```

Luego:

```bash
cp "$ORIG/04-ejemplo-wgs84-utm-zona-19s-chile.rs" src/main.rs
cargo run
```

Los ejemplos de lectura/escritura usan rutas de datos ilustrativas (`datos/...`, `salida/...`); ajustalas a archivos reales antes de ejecutar. El ejemplo R se corre con:

```bash
Rscript 06-equivalencia-con-r.R
```

Archivos generados:

- `01-ejemplo-leyendo-un-shapefile-o-geopackage.rs` | `rust` | líneas 24-48 | Formatos Espaciales > Lectura de Datos Vectoriales > Ejemplo: Leyendo un Shapefile o GeoPackage
- `02-ejemplo-leyendo-un-csv-con-latitud-longitud.rs` | `rust` | líneas 56-75 | Formatos Espaciales > Lectura de Datos Vectoriales > Ejemplo: Leyendo un CSV con Latitud/Longitud
- `03-escritura-y-conversion.rs` | `rust` | líneas 84-115 | Formatos Espaciales > Escritura y Conversión
- `04-ejemplo-wgs84-utm-zona-19s-chile.rs` | `rust` | líneas 132-151 | Formatos Espaciales > Reproyección de CRS > Ejemplo: WGS84 → UTM Zona 19S (Chile)
- `05-lectura-de-datos-raster.rs` | `rust` | líneas 173-215 | Formatos Espaciales > Lectura de Datos Raster
- `06-equivalencia-con-r.R` | `r` | líneas 220-227 | Formatos Espaciales > Equivalencia con R
