# geos

Fuente: `geos.qmd`

## Como ejecutar

La mayoria de archivos son fragmentos de API; el programa completo de esta carpeta es `12-spatial-join-puntos-en-poligonos.rs`.

Para ejecutarlo, usa Cargo y la crate `geos`:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-geos
cd /tmp/rust-espacial-geos
```

En `Cargo.toml`:

```toml
[dependencies]
geos = "8"
```

Luego:

```bash
cp "$ORIG/12-spatial-join-puntos-en-poligonos.rs" src/main.rs
cargo run
```

Los archivos `01` a `11` muestran llamadas aisladas y necesitan que definas geometrías como `geom`, `a`, `b` o `polygon` antes de ejecutarlos. El ejemplo R se ejecuta con `Rscript 13-equivalencia-con-r.R` despues de instalar `sf`.

Archivos generados:

- `01-buffer-zona-de-influencia.rs` | `rust` | líneas 27-35 | Las unidades dependen del CRS — esto es crítico > Buffer (Zona de Influencia)
- `02-convex-hull-cierre-convexo.rs` | `rust` | líneas 40-42 | Las unidades dependen del CRS — esto es crítico > Convex Hull (Cierre Convexo)
- `03-envelope-bounding-box.rs` | `rust` | líneas 47-49 | Las unidades dependen del CRS — esto es crítico > Envelope (Bounding Box)
- `04-simplify-simplificacion.rs` | `rust` | líneas 54-57 | Las unidades dependen del CRS — esto es crítico > Simplify (Simplificación)
- `05-predicados-relaciones-espaciales.rs` | `rust` | líneas 63-81 | Predicados (Relaciones Espaciales)
- `06-interseccion.rs` | `rust` | líneas 93-95 | Operaciones de Conjuntos (Set Operations) > Intersección
- `07-union.rs` | `rust` | líneas 100-102 | Operaciones de Conjuntos (Set Operations) > Unión
- `08-diferencia.rs` | `rust` | líneas 107-109 | Operaciones de Conjuntos (Set Operations) > Diferencia
- `09-diferencia-simetrica.rs` | `rust` | líneas 114-116 | Operaciones de Conjuntos (Set Operations) > Diferencia Simétrica
- `10-analisis-escalar.rs` | `rust` | líneas 122-132 | Análisis Escalar
- `11-topologia-y-validacion.rs` | `rust` | líneas 138-149 | Topología y Validación
- `12-spatial-join-puntos-en-poligonos.rs` | `rust` | líneas 155-197 | Spatial Join: Puntos en Polígonos
- `13-equivalencia-con-r.R` | `r` | líneas 213-217 | Equivalencia con R
