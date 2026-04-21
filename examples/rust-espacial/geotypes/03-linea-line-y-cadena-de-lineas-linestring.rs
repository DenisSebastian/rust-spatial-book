// Fuente: geotypes.qmd:43-58
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Primitivas Básicas > Línea (`Line`) y Cadena de Líneas (`LineString`)
// Bloque: 03
use geo::{Line, LineString, Point};

// Un segmento simple
let p1 = Point::new(0.0, 0.0);
let p2 = Point::new(1.0, 1.0);
let linea = Line::new(p1, p2);

// Una polilínea compleja
let ls: LineString = vec![
    (0.0, 0.0),
    (5.0, 0.0),
    (5.0, 5.0),
    (0.0, 5.0),
].into();
