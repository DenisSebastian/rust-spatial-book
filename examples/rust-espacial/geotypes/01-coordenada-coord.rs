// Fuente: geotypes.qmd:18-23
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Primitivas Básicas > Coordenada (`Coord`)
// Bloque: 01
use geo::Coord;

let c = Coord { x: 10.0, y: 20.0 };
println!("x: {}, y: {}", c.x, c.y);
