// Fuente: h3o.qmd:84-104
// Parte: Rust Espacial
// Seccion: Funciones Principales > 4. Recorrido y Vecindad (Grid Traversal)
// Bloque: 04
// k-ring: Obtener el hexágono y sus vecinos hasta distancia k
// k=1 retorna el anillo inmediato (6 vecinos + centro = 7 celdas)
let k = 1u32;
for neighbor in cell.grid_disk::<Vec<_>>(k) {
    println!("Vecino: {}", neighbor);
}

// Solo el anillo exterior (sin el interior)
// grid_ring_fast devuelve Option<CellIndex>: los pentágonos pueden
// interrumpir el anillo y producir None en esas posiciones.
for maybe_cell in cell.grid_ring_fast(k) {
    if let Some(anillo_cell) = maybe_cell {
        println!("Anillo: {}", anillo_cell);
    }
}

// Distancia topológica (saltos de grilla)
let destino: CellIndex = LatLng::new(-33.4600, -70.6500)?.to_cell(Resolution::Nine);
let saltos = cell.grid_distance(destino).unwrap(); // Ej: 5 saltos
