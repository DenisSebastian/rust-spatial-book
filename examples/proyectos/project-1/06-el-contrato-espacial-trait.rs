// Fuente: project_1.qmd:152-164
// Parte: Proyectos
// Seccion: Introducción > 4. El Contrato Espacial (`trait`)
// Bloque: 06
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
    fn area_shoelace(&self) -> f64 {
        let points = &self.exterior;
        let n = points.len();
        let mut sum = 0.0;

        for i in 0..n {
            let j = (i + 1) % n;
            sum += points[i].x * points[j].y;
            sum -= points[j].x * points[i].y;
        }

        (sum / 2.0).abs()
    }
}

pub trait SpatialObject {
    fn area(&self) -> f64;

    fn geometry_type(&self) -> &str;
}

impl SpatialObject for Polygon {
    fn area(&self) -> f64 {
        // Llamamos al helper privado, no al método del trait (evita recursión infinita)
        // `self.area()` aquí resolvería al trait de nuevo → stack overflow
        self.area_shoelace()
    }

    fn geometry_type(&self) -> &str {
        "Polygon"
    }
}

fn main() {
    let poligono = Polygon {
        exterior: vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 4.0, y: 0.0 },
            Point { x: 4.0, y: 4.0 },
            Point { x: 0.0, y: 0.0 },
        ],
    };

    println!("{} con área {}", poligono.geometry_type(), poligono.area());
}
