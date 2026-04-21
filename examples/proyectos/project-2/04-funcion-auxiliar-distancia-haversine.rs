// Fuente: project_2.qmd:246-257
// Parte: Proyectos
// Seccion: Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Función auxiliar: distancia Haversine
// Bloque: 04
/// Calcula la distancia en metros entre dos puntos geográficos (WGS84)
fn haversine_dist(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6_371_000.0; // Radio medio de la Tierra en metros
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    R * c
}
