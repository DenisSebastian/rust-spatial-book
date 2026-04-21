// Fuente: spatial_chapter.qmd:38-45
// Parte: Rust Espacial
// Seccion: Rust Espacial > Objetos Espaciales > Puntos y Coordenadas
// Bloque: 01
use geo_types::{Point, Coord};

let p = Point::new(10.0, 20.0);
// O usando coordenadas explícitas
let c = Coord { x: 10.0, y: 20.0 };
let p2: Point<f64> = c.into();
