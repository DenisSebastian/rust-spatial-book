# geotypes

Fuente: `geotypes.qmd`

## Como ejecutar

Estos ejemplos usan las crates `geo` y, para el ultimo ejemplo, `geo-svg`. Ejecutalos dentro de un proyecto Cargo temporal:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-geotypes
cd /tmp/rust-espacial-geotypes
```

En `Cargo.toml`:

```toml
[dependencies]
geo = "0.26"
geo-svg = "0.1"
```

Despues copia un ejemplo completo a `src/main.rs` y ejecutalo:

```bash
cp "$ORIG/10-visualizacion-de-debug-con-geo-svg.rs" src/main.rs
cargo run
```

Varios archivos iniciales son fragmentos sin `fn main`; para probarlos, envuelve las lineas del ejemplo dentro de `fn main() { ... }` en `src/main.rs`.

Archivos generados:

- `01-coordenada-coord.rs` | `rust` | líneas 18-23 | Datos Geoespaciales > Primitivas Básicas > Coordenada (`Coord`)
- `02-punto-point.rs` | `rust` | líneas 29-36 | Datos Geoespaciales > Primitivas Básicas > Punto (`Point`)
- `03-linea-line-y-cadena-de-lineas-linestring.rs` | `rust` | líneas 43-58 | Datos Geoespaciales > Primitivas Básicas > Línea (`Line`) y Cadena de Líneas (`LineString`)
- `04-poligono-polygon.rs` | `rust` | líneas 64-78 | Datos Geoespaciales > Primitivas Básicas > Polígono (`Polygon`)
- `05-multipoint.rs` | `rust` | líneas 88-95 | Datos Geoespaciales > Colecciones y Multi-Geometrías > MultiPoint
- `06-multilinestring.rs` | `rust` | líneas 101-108 | Datos Geoespaciales > Colecciones y Multi-Geometrías > MultiLineString
- `07-multipolygon.rs` | `rust` | líneas 114-122 | Datos Geoespaciales > Colecciones y Multi-Geometrías > MultiPolygon
- `08-geometria-geometry.rs` | `rust` | líneas 130-141 | Datos Geoespaciales > Tipos Genéricos > Geometría (`Geometry`)
- `09-coleccion-de-geometrias-geometrycollection.rs` | `rust` | líneas 147-157 | Datos Geoespaciales > Tipos Genéricos > Colección de Geometrías (`GeometryCollection`)
- `10-visualizacion-de-debug-con-geo-svg.rs` | `rust` | líneas 168-194 | Datos Geoespaciales > Visualización de Debug con `geo-svg`
