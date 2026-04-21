# Fuente: integration.qmd:58-67
# Parte: Integracion con R
# Seccion: Integración de Rust con R > Tu primera función híbrida > Ejecución desde R
# Bloque: 03
rextendr::rust_function("
fn sum_rust(x: i32, y: i32) -> i32 {
    x + y
}
")

sum_rust(10, 20)
#> [1] 30
