// Fuente: project_1.qmd:140-148
// Parte: Proyectos
// Seccion: Introducción > 4. El Contrato Espacial (`trait`)
// Bloque: 05
pub trait SpatialObject {
    // Cualquier objeto espacial debe saber decirme su área
    fn area(&self) -> f64;

    // Y digamos que también queremos saber su tipo como texto
    fn geometry_type(&self) -> &str;
}

fn main() {
    println!("Trait SpatialObject definido.");
}
