// Fuente: geotypes.qmd:114-122
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Colecciones y Multi-Geometrías > MultiPolygon
// Bloque: 07
use geo::{MultiPolygon, Polygon};

// Imaginemos dos polígonos simples (aquí simplificados)
let p1 = Polygon::new(vec![(0.,0.), (1.,1.), (1.,0.), (0.,0.)].into(), vec![]);
let p2 = Polygon::new(vec![(2.,2.), (3.,3.), (3.,2.), (2.,2.)].into(), vec![]);

let pais = MultiPolygon::new(vec![p1, p2]);
