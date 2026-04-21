// Fuente: geos.qmd:122-132
// Parte: Rust Espacial
// Seccion: Análisis Escalar
// Bloque: 10
// Área (para polígonos)
let area = geom.area().unwrap();

// Longitud (para líneas) o perímetro (para polígonos)
let largo = geom.length().unwrap();

// Distancia mínima Euclidiana entre dos geometrías
// IMPORTANTE: resultado en unidades del CRS — reproyectar a UTM para obtener metros
let dist = a.distance(&b).unwrap();
