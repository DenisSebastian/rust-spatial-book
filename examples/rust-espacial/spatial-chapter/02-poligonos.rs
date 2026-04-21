// Fuente: spatial_chapter.qmd:53-61
// Parte: Rust Espacial
// Seccion: Rust Espacial > Objetos Espaciales > Polígonos
// Bloque: 02
use geo_types::{Polygon, LineString};

let exterior = LineString::from(vec![
    (0., 0.), (10., 0.), (10., 10.), (0., 10.), (0., 0.)
]);

let poly = Polygon::new(exterior, vec![]);
