// Fuente: geotypes.qmd:130-141
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Tipos Genéricos > Geometría (`Geometry`)
// Bloque: 08
use geo::Geometry;

// Supongamos que esta función viene de leer un archivo
let geom: Geometry = some_polygon.into();

match geom {
    Geometry::Point(p) => println!("Es un punto: {:?}", p),
    Geometry::Polygon(poly) => println!("Es un polígono"),
    _ => println!("Es otro tipo de geometría"),
}
