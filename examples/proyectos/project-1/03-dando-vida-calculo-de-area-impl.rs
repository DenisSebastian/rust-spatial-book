// Fuente: project_1.qmd:73-109
// Parte: Proyectos
// Seccion: Introducción > 3. Dando Vida: Cálculo de Área (`impl`)
// Bloque: 03
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
    // Constructor básico
    pub fn new(points: Vec<Point>) -> Self {
        Polygon { exterior: points }
    }

    // Método privado con la lógica real (Algoritmo del Cordón / Shoelace)
    // Al ser privado, no colisiona con el método `area` del trait SpatialObject
    fn area_shoelace(&self) -> f64 {
        let points = &self.exterior;
        let n = points.len();

        if n < 3 {
            return 0.0; // No es un polígono si tiene menos de 3 puntos
        }

        let mut sum = 0.0;

        for i in 0..n {
            // Conectamos el punto actual (i) con el siguiente (j)
            // Si i es el último, j debe ser el primero (el índice 0)
            let j = (i + 1) % n;

            sum += points[i].x * points[j].y;
            sum -= points[j].x * points[i].y;
        }

        (sum / 2.0).abs()
    }

    // Método público que delega en el helper privado
    pub fn area(&self) -> f64 {
        self.area_shoelace()
    }
}

fn main() {
    let poligono = Polygon::new(vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 10.0, y: 0.0 },
        Point { x: 10.0, y: 10.0 },
        Point { x: 0.0, y: 0.0 },
    ]);

    println!("Área del polígono: {}", poligono.area());
}
