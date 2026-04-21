// Fuente: project_1.qmd:56-63
// Parte: Proyectos
// Seccion: Introducción > 2. Construyendo Geometrías: El Polígono (`Polygon`)
// Bloque: 02
#[path = "support/project1.rs"]
mod project1;

use project1::Point;

#[derive(Debug, Clone)]
pub struct Polygon {
    // Usamos Vec (Vector) porque el polígono puede tener 3, 4, 100 o mil puntos.
    // Es una lista dinámica.
    pub exterior: Vec<Point>,
}

fn main() {
    let poligono = Polygon {
        exterior: vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 0.0),
        ],
    };

    println!("{:?}", poligono);
}
