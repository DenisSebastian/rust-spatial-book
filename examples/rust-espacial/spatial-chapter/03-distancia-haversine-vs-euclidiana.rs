// Fuente: spatial_chapter.qmd:77-95
// Parte: Rust Espacial
// Seccion: Rust Espacial > Operaciones Espaciales > Distancia (Haversine vs Euclidiana)
// Bloque: 03
use geo::{point, EuclideanDistance, HaversineDistance};

fn main() {
    // Santiago de Chile y Valparaíso
    let santiago   = point!(x: -70.6693, y: -33.4489); // (lon, lat)
    let valparaiso = point!(x: -71.6197, y: -33.0472);

    // Haversine: para coordenadas geográficas (lon/lat) — resultado en metros
    let dist_haversine = santiago.haversine_distance(&valparaiso);
    println!("Haversine: {:.1} m ({:.1} km)", dist_haversine, dist_haversine / 1000.0);
    // Haversine: ~112 000 m (~112 km) ✓

    // Euclidiana sobre grados: resultado en "grados" — incorrecto para análisis real
    let dist_euclidiana_grados = santiago.euclidean_distance(&valparaiso);
    println!("Euclidiana (grados): {:.4}°", dist_euclidiana_grados);
    // ~1.04° — número sin significado métrico directo ✗
}
