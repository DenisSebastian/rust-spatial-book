// Fuente: project_1.qmd:208-230
// Parte: Proyectos
// Seccion: Introducción > 5. Validación y Robustez: Evitando Geometrías Imposibles
// Bloque: 08
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub exterior: Vec<Point>,
}

impl Polygon {
    // Ahora devolvemos Option<Self>
    pub fn new_checked(points: Vec<Point>) -> Option<Self> {
        if points.len() < 4 {
            // Un polígono cerrado necesita al menos 3 vértices + 1 de cierre = 4 puntos
            return None;
        }

        // Verificamos si está cerrado (primero == último)
        // Ojo: comparar f64 con == es peligroso por decimales, pero para este ejemplo sirve.
        let first = points.first().unwrap();
        let last = points.last().unwrap();

        // Pequeño truco para comparar float: la diferencia debe ser muy pequeña
        if (first.x - last.x).abs() > 1e-6 || (first.y - last.y).abs() > 1e-6 {
            return None; // No está cerrado
        }

        Some(Polygon { exterior: points })
    }
}

fn main() {
    let puntos = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 0.0, y: 0.0 },
    ];

    println!(
        "Polígono válido: {}",
        Polygon::new_checked(puntos).is_some()
    );
}
