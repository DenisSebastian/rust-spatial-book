// Fuente: project_1.qmd:115-130
// Parte: Proyectos
// Seccion: Introducción > 3. Dando Vida: Cálculo de Área (`impl`) > Probémoslo
// Bloque: 04
#[path = "support/project1.rs"]
mod project1;

use project1::{Point, Polygon};

fn main() {
    // Creamos un cuadrado 10x10
    let puntos = vec![
        Point::new(0.0, 0.0),
        Point::new(10.0, 0.0),
        Point::new(10.0, 10.0),
        Point::new(0.0, 10.0),
        Point::new(0.0, 0.0), // Cerramos el ciclo
    ];

    let mi_poligono = Polygon::new(puntos);

    println!("Área del polígono: {}", mi_poligono.area());
}
