// Fuente: integration.qmd:37-52
// Parte: Integracion con R
// Seccion: Integración de Rust con R > Tu primera función híbrida > El código Rust
// Bloque: 02
use extendr_api::prelude::*; // Importamos las herramientas necesarias

/// Suma dos enteros
/// @export
#[extendr] // Esta macro hace la magia
fn sum_rust(x: i32, y: i32) -> i32 {
    x + y
}

// Esta macro genera el código "pegamento" para R
extendr_module! {
    mod my_module;
    fn sum_rust;
}
