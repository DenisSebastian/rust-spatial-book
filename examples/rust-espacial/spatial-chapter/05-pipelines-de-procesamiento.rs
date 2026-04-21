// Fuente: spatial_chapter.qmd:120-124
// Parte: Rust Espacial
// Seccion: Rust Espacial > Error silencioso frecuente > Pipelines de Procesamiento
// Bloque: 05
let centroids: Vec<Point<f64>> = polygons.iter()
    .map(|poly| poly.centroid().unwrap()) // unwrap porque el centroide podría fallar si el poligono es inválido
    .collect();
