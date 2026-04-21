// Fuente: project_2.qmd:181-204
// Parte: Proyectos
// Seccion: Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Paso 1: Estructuras de Datos
// Bloque: 02
use h3o::{CellIndex, LatLng, Resolution};
use std::collections::HashMap;

/// Representa un Punto de Interés (POI)
#[derive(Debug, Clone)]
struct Poi {
    id: u64,
    nombre: String,
    lat: f64,
    lon: f64,
    tipo: String, // "restaurante", "farmacia", etc.
}

/// Índice H3 que mapea celdas a POIs
struct H3Index {
    /// Mapeo: celda H3 → lista de POIs en esa celda
    celdas: HashMap<CellIndex, Vec<Poi>>,
    /// Índice inverso: ID de POI → celda donde está
    ubicacion: HashMap<u64, CellIndex>,
    /// Resolución usada para construir el índice
    resolucion: Resolution,
}
