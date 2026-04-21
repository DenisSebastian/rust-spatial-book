// Fuente: geohash.qmd:25-38
// Parte: Rust Espacial
// Seccion: Geohash e Indexación > Operaciones Básicas > Codificar (Encode)
// Bloque: 01
use geohash::{encode, Coordinate};

fn main() -> Result<(), geohash::GeohashError> {
    // Santiago de Chile (aprox)
    let coord = Coordinate { x: -70.6693, y: -33.4489 };
    
    // Precisión 9 (~5 metros)
    let hash = encode(coord, 9)?;
    
    println!("Geohash: {}", hash); // ej. "66j9..."
    Ok(())
}
