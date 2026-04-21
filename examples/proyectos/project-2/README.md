# project_2

Fuente: `project_2.qmd`

## Como ejecutar

Esta carpeta mezcla un script R, fragmentos Rust progresivos y un benchmark complementario.

El script R inicial se ejecuta con:

```bash
Rscript 01-el-codigo-actual-enfoque-ingenuo.R
```

Los fragmentos Rust `02` a `09` construyen el proyecto paso a paso y usan la crate `h3o`; no estan pensados para `rustc` directo uno por uno. Para probarlos, crea un proyecto Cargo temporal y combina los bloques acumulados en `src/main.rs`:

```bash
ORIG="$PWD"
cargo new /tmp/rust-espacial-project-2
cd /tmp/rust-espacial-project-2
```

En `Cargo.toml`:

```toml
[dependencies]
h3o = "0.6"

[dev-dependencies]
criterion = "0.5"
```

Para el benchmark complementario:

```bash
mkdir -p benches
cp "$ORIG/10-benchmark-h3-optimization.rs" benches/h3_optimization.rs
```

Agrega tambien al `Cargo.toml`:

```toml
[[bench]]
name = "h3_optimization"
harness = false
```

Y ejecuta:

```bash
cargo bench --bench h3_optimization
```

Archivos generados:

- `01-el-codigo-actual-enfoque-ingenuo.R` | `r` | líneas 33-49 | Sistema de Optimización Espacial con Indexación H3 > Parte 1: Diagnóstico del Problema > El Código Actual (Enfoque Ingenuo)
- `02-paso-1-estructuras-de-datos.rs` | `rust` | líneas 181-204 | Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Paso 1: Estructuras de Datos
- `03-paso-2-construccion-del-indice.rs` | `rust` | líneas 208-238 | Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Paso 2: Construcción del Índice
- `04-funcion-auxiliar-distancia-haversine.rs` | `rust` | líneas 246-257 | Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Función auxiliar: distancia Haversine
- `05-paso-3-consulta-eficiente.rs` | `rust` | líneas 261-307 | Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Paso 3: Consulta Eficiente
- `06-paso-4-ejemplo-de-uso.rs` | `rust` | líneas 311-359 | Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Paso 4: Ejemplo de Uso
- `07-ejercicio-1-extender-el-indice.rs` | `rust` | líneas 447-458 | Sistema de Optimización Espacial con Indexación H3 > Parte 7: Ejercicios para el Lector > Ejercicio 1: Extender el Índice
- `08-ejercicio-2-ajustar-la-resolucion.rs` | `rust` | líneas 464-470 | Sistema de Optimización Espacial con Indexación H3 > Parte 7: Ejercicios para el Lector > Ejercicio 2: Ajustar la Resolución
- `09-ejercicio-3-benchmark-propio.rs` | `rust` | líneas 476-489 | Sistema de Optimización Espacial con Indexación H3 > Parte 7: Ejercicios para el Lector > Ejercicio 3: Benchmark Propio

Archivos complementarios:

- `10-benchmark-h3-optimization.rs` | `rust` | líneas 377 y 476-489 | Benchmark runnable para el ejemplo mencionado como `cargo bench --bench h3_optimization`
