// Fuente: h3o.qmd:66-78
// Parte: Rust Espacial
// Seccion: Funciones Principales > 3. Jerarquía (Zoom In / Zoom Out)
// Bloque: 03
// Obtener el padre (Zoom Out)
let parent = cell.parent(Resolution::Eight).unwrap();

// Obtener los hijos directos (Zoom In)
// Retorna un iterador
for child in cell.children(Resolution::Ten) {
    println!("Hijo: {}", child);
}

// Obtener el hijo central
let center_child = cell.center_child(Resolution::Ten).unwrap();
