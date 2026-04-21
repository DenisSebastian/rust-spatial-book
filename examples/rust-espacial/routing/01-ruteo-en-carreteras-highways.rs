// Fuente: routing.qmd:33-49
// Parte: Rust Espacial
// Seccion: Routing y Distancias > Uso Básico > Ruteo en Carreteras (Highways)
// Bloque: 01
use routrs::prelude::*;
use routrs::highways::GEOGRAPH as highways;

fn main() {
    let from: Geoloc = (-33.4489, -70.6693); // Santiago centro
    let to: Geoloc = (-33.3942, -70.7950);  // Aeropuerto de Santiago

    // Calcula: distancia total, vector de puntos, y tipo de ruta
    let (distance, path, path_type) = highways::shortest_path(&from, &to);

    println!("Distancia: {:.2} km", distance);
    println!("Nodos en la ruta: {}", path.len());
    // Path type suele ser "ViaWaypoints" si encontró ruta, o directo si no.
    println!("Tipo de ruta: {}", path_type); 
}
