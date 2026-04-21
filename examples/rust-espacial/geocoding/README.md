# geocoding

Fuente: `geocoding.qmd`

## Como ejecutar

`01-busqueda-naive-fuerza-bruta.rs` es el ejemplo mas directo para probar. Los ejemplos de indice espacial usan `rstar`; el de Tantivy es conceptual.

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-geocoding
cd /tmp/rust-espacial-geocoding
```

En `Cargo.toml`, agrega:

```toml
[dependencies]
rstar = "0.12"
```

Luego copia el archivo que quieras probar:

```bash
cp "$ORIG/03-geocodificacion-inversa-reverse-geocoding.rs" src/main.rs
cargo run
```

Si el archivo no tiene `fn main` completo o contiene pseudocodigo, usalo como plantilla y completa los datos de entrada indicados en el capitulo.

Archivos generados:

- `01-busqueda-naive-fuerza-bruta.rs` | `rust` | líneas 18-79 | Geocodificación > Búsqueda Naive (Fuerza Bruta)
- `02-ejemplo-conceptual-con-tantivy.rs` | `rust` | líneas 93-109 | Geocodificación > Estrategias para Producción > Ejemplo conceptual con Tantivy
- `03-geocodificacion-inversa-reverse-geocoding.rs` | `rust` | líneas 120-177 | Geocodificación > Geocodificación Inversa (Reverse Geocoding)
