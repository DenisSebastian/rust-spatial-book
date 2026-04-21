// Fuente: project_1.qmd:172-195
// Parte: Proyectos
// Seccion: Introducción > 4. El Contrato Espacial (`trait`) > Probémoslo en `main`
// Bloque: 07
#[path = "support/project1.rs"]
mod project1;

use project1::{Point, Polygon, SpatialObject};

fn main() {
    let puntos = vec![
        Point::new(0.0, 0.0),
        Point::new(4.0, 0.0),
        Point::new(4.0, 4.0),
        Point::new(0.0, 4.0),
        Point::new(0.0, 0.0),
    ];

    // Creamos el polígono
    let cuadrado = Polygon::new(puntos);

    // ¡Aquí está la magia!
    // Pasamos el polígono a una función que SOLO espera un "SpatialObject"
    reporte_espacial(&cuadrado);
}

// Esta función acepta CUALQUIER cosa que cumpla el contrato SpatialObject
fn reporte_espacial(objeto: &impl SpatialObject) {
    println!("--- REPORTE ---");
    println!("Tipo de Geometría: {}", objeto.geometry_type());
    println!("Medida Espacial:   {}", objeto.area());
    println!("-----------------");
}
