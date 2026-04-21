// Fuente: h3o.qmd:30-48
// Parte: Rust Espacial
// Seccion: Funciones Principales > 1. Indexación (Coordenada ↔ Celda)
// Bloque: 01
use h3o::{CellIndex, LatLng, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Santiago de Chile
    let coord = LatLng::new(-33.4489, -70.6693)?;

    // De Coordenada a Celda (Resolución 9)
    // to_cell() es infallible una vez que LatLng es válido
    let cell: CellIndex = coord.to_cell(Resolution::Nine);
    println!("Cell Index: {}", cell);

    // De Celda a Coordenada (Centroide)
    let centro: LatLng = cell.into();
    println!("Centrolat: {}, Centrolon: {}", centro.lat(), centro.lng());

    Ok(())
}
