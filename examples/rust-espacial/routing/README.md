# routing

Fuente: `routing.qmd`

## Como ejecutar

Estos ejemplos usan la crate `routrs`, asi que deben correrse con Cargo:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-routing
cd /tmp/rust-espacial-routing
```

En `Cargo.toml`:

```toml
[dependencies]
routrs = { version = "2.0.0", features = ["all"] }
```

Luego:

```bash
cp "$ORIG/01-ruteo-en-carreteras-highways.rs" src/main.rs
cargo run
```

Algunos ejemplos son esqueletos pedagogicos y pueden requerir datos de grafo o funciones auxiliares del ecosistema `routrs` antes de correr en un caso real.

Archivos generados:

- `01-ruteo-en-carreteras-highways.rs` | `rust` | líneas 33-49 | Routing y Distancias > Uso Básico > Ruteo en Carreteras (Highways)
- `02-ruteo-maritimo-y-ferroviario.rs` | `rust` | líneas 55-68 | Routing y Distancias > Uso Básico > Ruteo Marítimo y Ferroviario
- `03-calculo-concurrente-batch-processing.rs` | `rust` | líneas 74-91 | Routing y Distancias > Cálculo Concurrente (Batch Processing)
- `04-grafos-personalizados.rs` | `rust` | líneas 97-117 | Routing y Distancias > Grafos Personalizados
