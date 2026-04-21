// Fuente: geotypes.qmd:88-95
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Colecciones y Multi-Geometrías > MultiPoint
// Bloque: 05
use geo::{MultiPoint, Point};

let puntos = MultiPoint::new(vec![
    Point::new(0.0, 0.0),
    Point::new(10.0, 10.0),
]);
