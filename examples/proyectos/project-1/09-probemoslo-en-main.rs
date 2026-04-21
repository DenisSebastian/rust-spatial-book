// Fuente: project_1.qmd:238-266
// Parte: Proyectos
// Seccion: Introducción > 5. Validación y Robustez: Evitando Geometrías Imposibles > Probémoslo en `main`
// Bloque: 09
#[path = "support/project1.rs"]
mod project1;

use project1::{Point, Polygon};

fn main() {
    // Caso 1: Puntos insuficientes (solo una línea)
    let puntos_malos = vec![Point::new(0.0, 0.0), Point::new(10.0, 10.0)];

    println!("Intentando crear polígono inválido...");
    let resultado = Polygon::new_checked(puntos_malos);

    match resultado {
        Some(p) => println!("¡Creado! Área: {}", p.area()),
        None => println!(">> Error: No se pudo crear el polígono. Datos inválidos."),
    }

    // Caso 2: Datos correctos
    let puntos_buenos = vec![
        Point::new(0.0, 0.0),
        Point::new(5.0, 0.0),
        Point::new(5.0, 5.0),
        Point::new(0.0, 5.0),
        Point::new(0.0, 0.0), // Cerrado
    ];

    println!("\nIntentando crear polígono válido...");
    if let Some(poly) = Polygon::new_checked(puntos_buenos) {
        println!(">> ¡Éxito! Polígono creado correctamente.");
    }
}
