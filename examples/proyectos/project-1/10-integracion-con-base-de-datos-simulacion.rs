// Fuente: project_1.qmd:274-288
// Parte: Proyectos
// Seccion: Introducción > 6. Integración con Base de Datos (Simulación)
// Bloque: 10
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
    pub fn to_wkt(&self) -> String {
        let coords: Vec<String> = self
            .exterior
            .iter()
            .map(|p| format!("{} {}", p.x, p.y)) // "x y"
            .collect();

        // Unimos con comas
        let coords_str = coords.join(", ");

        format!("POLYGON(({}))", coords_str)
    }
}

fn main() {
    let poligono = Polygon {
        exterior: vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 0.0 },
        ],
    };

    println!("{}", poligono.to_wkt());
}
