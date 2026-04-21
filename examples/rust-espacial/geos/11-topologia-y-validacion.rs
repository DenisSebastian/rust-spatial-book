// Fuente: geos.qmd:138-149
// Parte: Rust Espacial
// Seccion: Topología y Validación
// Bloque: 11
if !geom.is_valid() {
    println!("¡Geometría inválida encontrada!");
    println!("Razón: {}", geom.get_valid_reason().unwrap_or_default());
    
    // Intentar arreglarla (disponible en versiones recientes de GEOS)
    // make_valid intenta descomponer el polígono en partes válidas
    if let Ok(fixed) = geom.make_valid() {
        // Usar 'fixed'...
    }
}
