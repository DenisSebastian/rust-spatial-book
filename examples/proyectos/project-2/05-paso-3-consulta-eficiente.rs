// Fuente: project_2.qmd:261-307
// Parte: Proyectos
// Seccion: Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Paso 3: Consulta Eficiente
// Bloque: 05
impl H3Index {
    /// Busca POIs dentro de un radio dado
    /// COSTO: O(k) donde k es el número de celdas en el k-ring
    /// TIEMPO: ~5-10ms para 2.3M POIs
    pub fn buscar_en_radio(
        &self,
        latitud: f64,
        longitud: f64,
        radio_metros: f64,
    ) -> Vec<&Poi> {
        // 1. Convertir coordenadas a celda H3
        let coord = LatLng::new(latitud, longitud)
            .expect("Coordenadas inválidas");

        let celda_centro = CellIndex::try_from_latlng(coord, self.resolucion)
            .expect("No se pudo crear celda H3");

        // 2. Obtener celdas vecinas (k-ring)
        // k=1 = celda centro + 6 vecinos inmediatos
        let celdas_vecinas: Vec<CellIndex> = celda_centro
            .grid_disk(1) // 7 celdas totales
            .collect();

        // 3. Recoger POIs de todas las celdas vecinas
        let mut candidatos: Vec<&Poi> = Vec::new();
        for celda in &celdas_vecinas {
            if let Some(pois) = self.celdas.get(celda) {
                candidatos.extend(pois.iter());
            }
        }

        // 4. Filtrar por distancia exacta (H3 es aproximado)
        let mut resultados = Vec::new();
        for poi in candidatos {
            let dist = haversine_dist(
                latitud, longitud, poi.lat, poi.lon
            );
            if dist <= radio_metros {
                resultados.push(poi);
            }
        }

        resultados
    }
}
