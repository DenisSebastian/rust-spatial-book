// Fuente: geotypes.qmd:29-36
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Primitivas Básicas > Punto (`Point`)
// Bloque: 02
use geo::Point;

let p = Point::new(10.0, 20.0);
// Puedes acceder a sus coordenadas
let x = p.x();
let y = p.y();
