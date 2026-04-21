// Fuente: integration.qmd:84-94
// Parte: Integracion con R
// Seccion: Integración de Rust con R > Pasando Datos: Vectores y Matrices
// Bloque: 04
#[extendr]
fn euclidean_dist(x: Doubles, y: Doubles) -> f64 {
    // Iteramos sobre ambos vectores simultáneamente
    let sum_sq: f64 = x.iter().zip(y.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum();
    
    sum_sq.sqrt()
}
