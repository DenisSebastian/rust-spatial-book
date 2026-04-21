# geohash

Fuente: `geohash.qmd`

## Como ejecutar

Estos ejemplos usan la crate `geohash`:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-geohash
cd /tmp/rust-espacial-geohash
```

En `Cargo.toml`:

```toml
[dependencies]
geohash = "0.14"
```

Luego:

```bash
cp "$ORIG/01-codificar-encode.rs" src/main.rs
cargo run
```

Los archivos que no tengan `fn main` completo deben pegarse dentro de `src/main.rs` con una funcion `main` pequeña para imprimir el resultado.

Archivos generados:

- `01-codificar-encode.rs` | `rust` | líneas 25-38 | Geohash e Indexación > Operaciones Básicas > Codificar (Encode)
- `02-decodificar-decode.rs` | `rust` | líneas 44-52 | Geohash e Indexación > Operaciones Básicas > Decodificar (Decode)
- `03-el-problema-de-borde-buscar-solo-la-celda-propia-es-un-bug.rs` | `rust` | líneas 68-81 | Geohash e Indexación > El problema de borde: buscar solo la celda propia es un bug
