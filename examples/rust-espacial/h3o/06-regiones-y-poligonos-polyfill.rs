// Fuente: h3o.qmd:124-148
// Parte: Rust Espacial
// Seccion: Funciones Principales > 6. Regiones y Polígonos (Polyfill)
// Bloque: 06
use geo_types::{Polygon, LineString, Coord};
use h3o::geom::{Polygon as H3Polygon, PolyfillConfig, ToCells};
use h3o::Resolution;

// Bounding box aproximado del centro de Santiago de Chile
// Convención de geo_types: Coord { x: longitud, y: latitud }
let exterior = LineString::from(vec![
    Coord { x: -70.690, y: -33.465 }, // Suroeste
    Coord { x: -70.640, y: -33.465 }, // Sureste
    Coord { x: -70.640, y: -33.430 }, // Noreste
    Coord { x: -70.690, y: -33.430 }, // Noroeste
    Coord { x: -70.690, y: -33.465 }, // Cierre
]);
let poli = Polygon::new(exterior, vec![]);

// Convertir a h3o::geom::Polygon (valida que las coordenadas sean finitas)
let h3_poli = H3Polygon::from_degrees(poli)?;

// Configurar resolución y ejecutar polyfill
let config = PolyfillConfig::new(Resolution::Five);
for cell in h3_poli.to_cells(config) {
    println!("Celda dentro del polígono: {}", cell);
}
