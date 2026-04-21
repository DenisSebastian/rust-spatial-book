// Fuente: spatial_chapter.qmd:141-155
// Parte: Rust Espacial
// Seccion: Rust Espacial > Caso Práctico: Geohash > El problema del Vecino
// Bloque: 06
use geohash::{neighbor, Direction};

fn get_neighbor(hash_str: &str, direction_str: &str) -> String {
    let dir = match direction_str {
        "n" => Direction::N,
        "s" => Direction::S,
        "e" => Direction::E,
        "w" => Direction::W,
        _ => panic!("Dirección inválida"),
    };
    // Si olvidaras alguna dirección, el compilador lo detecta
    neighbor(hash_str, dir).expect("Error calculando vecino")
}
