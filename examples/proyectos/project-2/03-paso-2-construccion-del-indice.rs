// Fuente: project_2.qmd:208-238
// Parte: Proyectos
// Seccion: Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Paso 2: Construcción del Índice
// Bloque: 03
impl H3Index {
    /// Construye un índice H3 a partir de una lista de POIs
    /// COSTO: O(n) - se ejecuta UNA VEZ durante la inicialización
    pub fn construir(pois: &[Poi], resolucion: Resolution) -> Self {
        let mut celdas = HashMap::new();
        let mut ubicacion = HashMap::new();

        for poi in pois {
            // Convertir coordenadas a celda H3
            let coord = LatLng::new(poi.lat, poi.lon)
                .expect("Coordenadas inválidas");

            let celda = CellIndex::try_from_latlng(coord, resolucion)
                .expect("No se pudo crear celda H3");

            // Agregar POI a la celda correspondiente
            celdas.entry(celda).or_insert_with(Vec::new).push(poi.clone());

            // Registrar ubicación del POI
            ubicacion.entry(poi.id).or_insert(celda);
        }

        Self {
            celdas,
            ubicacion,
            resolucion,
        }
    }
}
