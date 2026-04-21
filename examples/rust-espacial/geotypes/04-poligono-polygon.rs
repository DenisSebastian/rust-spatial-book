// Fuente: geotypes.qmd:64-78
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Primitivas Básicas > Polígono (`Polygon`)
// Bloque: 04
use geo::{Polygon, LineString};

// Definir el exterior (un cuadrado)
let exterior: LineString = vec![
    (0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0), (0.0, 0.0),
].into();

// Definir un agujero triangular dentro del cuadrado
let interior: LineString = vec![
    (2.0, 2.0), (8.0, 2.0), (5.0, 8.0), (2.0, 2.0),
].into();

let poly = Polygon::new(exterior, vec![interior]);
