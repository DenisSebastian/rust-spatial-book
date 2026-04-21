# integration

Fuente: `integration.qmd`

## Como ejecutar

Estos ejemplos forman un flujo `rextendr`: primero se prepara el paquete en R, luego se compila Rust y finalmente se llama desde R. Ejecutalos desde una sesion de R o con `Rscript` en el orden del capitulo:

```bash
Rscript 01-preparacion-en-r.R
```

Los archivos Rust (`02` y `04`) no se compilan con `rustc` directo: pertenecen a un proyecto creado por `rextendr` y usan la crate `extendr-api`. Despues de crear el proyecto con R, copia el codigo Rust al archivo que corresponda dentro de `src/rust/src/lib.rs` y ejecuta:

```bash
Rscript 03-ejecucion-desde-r.R
```

Necesitas tener instalados los paquetes R `rextendr` y `devtools`, ademas de una toolchain Rust funcional.

Archivos generados:

- `01-preparacion-en-r.R` | `r` | líneas 28-31 | Integración de Rust con R > Tu primera función híbrida > Preparación en R
- `02-el-codigo-rust.rs` | `rust` | líneas 37-52 | Integración de Rust con R > Tu primera función híbrida > El código Rust
- `03-ejecucion-desde-r.R` | `r` | líneas 58-67 | Integración de Rust con R > Tu primera función híbrida > Ejecución desde R
- `04-pasando-datos-vectores-y-matrices.rs` | `rust` | líneas 84-94 | Integración de Rust con R > Pasando Datos: Vectores y Matrices
