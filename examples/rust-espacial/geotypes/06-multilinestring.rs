// Fuente: geotypes.qmd:101-108
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Colecciones y Multi-Geometrías > MultiLineString
// Bloque: 06
use geo::{MultiLineString, LineString};

let camino1: LineString = vec![(0.0, 0.0), (1.0, 1.0)].into();
let camino2: LineString = vec![(2.0, 2.0), (3.0, 3.0)].into();

let red = MultiLineString::new(vec![camino1, camino2]);
