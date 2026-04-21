// Fuente: geos.qmd:27-35
// Parte: Rust Espacial
// Seccion: Las unidades dependen del CRS — esto es crítico > Buffer (Zona de Influencia)
// Bloque: 01
use geos::Geom; // Trait necesario para usar los métodos

// IMPORTANTE: 'polygon' debe estar en CRS proyectado (ej. EPSG:32719)
// para que la distancia sea en metros.
let geom = polygon.buffer(10.0, 8).expect("Error en buffer");
// 10.0 = distancia en las unidades del CRS (metros si está proyectado)
// 8 = 'quadsegs' (segmentos por cuadrante para suavizar curvas)
