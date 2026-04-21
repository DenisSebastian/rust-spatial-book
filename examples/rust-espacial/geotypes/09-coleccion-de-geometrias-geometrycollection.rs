// Fuente: geotypes.qmd:147-157
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Tipos Genéricos > Colección de Geometrías (`GeometryCollection`)
// Bloque: 09
use geo::{GeometryCollection, Geometry, Point, LineString};

let p = Point::new(0.0, 0.0);
let l = LineString::from(vec![(1.0, 1.0), (2.0, 2.0)]);

let coleccion = GeometryCollection::from(vec![
    Geometry::Point(p),
    Geometry::LineString(l),
]);
