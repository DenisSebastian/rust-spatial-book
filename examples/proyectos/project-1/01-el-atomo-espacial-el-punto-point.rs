// Fuente: project_1.qmd:29-45
// Parte: Proyectos
// Seccion: Introducción > 1. El Átomo Espacial: El Punto (`Point`)
// Bloque: 01
// Derivamos Debug para poder imprimirlo en la consola.
// Derivamos Clone y Copy porque un punto es muy ligero (solo dos números),
// y es más fácil copiarlo que pasarlo por referencia constantemente.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    // Un "constructor" simple
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

fn main() {
    let punto = Point::new(10.0, 20.0);
    println!("{:?}", punto);
}
