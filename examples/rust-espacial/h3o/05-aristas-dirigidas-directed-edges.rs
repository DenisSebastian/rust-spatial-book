// Fuente: h3o.qmd:110-116
// Parte: Rust Espacial
// Seccion: Funciones Principales > 5. Aristas Dirigidas (Directed Edges)
// Bloque: 05
// Obtener las aristas salientes de una celda
// (6 aristas para hexágonos, 5 para pentágonos)
for edge in cell.edges() {
    println!("Arista hacia vecino: {}", edge.destination());
}
