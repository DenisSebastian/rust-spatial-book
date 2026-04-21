// Fuente: geocoding.qmd:120-177
// Parte: Rust Espacial
// Seccion: Geocodificación > Geocodificación Inversa (Reverse Geocoding)
// Bloque: 03
use rstar::{RTree, RTreeObject, AABB, PointDistance};

/// Punto de interés con dirección asociada
#[derive(Debug, Clone)]
struct POI {
    coords: [f64; 2],   // [longitud, latitud] — WGS84
    direccion: String,
}

impl RTreeObject for POI {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.coords)
    }
}

impl PointDistance for POI {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let dx = self.coords[0] - point[0];
        let dy = self.coords[1] - point[1];
        dx * dx + dy * dy  // distancia euclidiana al cuadrado
    }
}

fn main() {
    // Simular edificios con dirección en Providencia, Santiago
    let pois = vec![
        POI { coords: [-70.6358, -33.4172], direccion: "Av. Providencia 1234".to_string() },
        POI { coords: [-70.6400, -33.4200], direccion: "Av. Providencia 890".to_string() },
        POI { coords: [-70.6300, -33.4150], direccion: "Av. Pedro de Valdivia 100".to_string() },
        POI { coords: [-70.6450, -33.4250], direccion: "Av. Irarrázaval 321".to_string() },
    ];

    // bulk_load es más eficiente que insertar uno a uno
    let rtree = RTree::bulk_load(pois);

    // Geocodificación inversa: ¿qué dirección está más cerca de esta coordenada?
    let mi_ubicacion = [-70.6380_f64, -33.4190_f64];

    if let Some(poi) = rtree.nearest_neighbor(&mi_ubicacion) {
        println!("Dirección más cercana: {}", poi.direccion);
        println!("Coordenadas: {:?}", poi.coords);
    }

    // Buscar todos los POIs dentro de un radio (~500m en grados ≈ 0.0045)
    let radio_2 = 0.005_f64.powi(2); // radio al cuadrado
    let cercanos: Vec<_> = rtree
        .locate_within_distance(mi_ubicacion, radio_2)
        .collect();

    println!("\nDirecciones en el radio:");
    for poi in &cercanos {
        println!("  - {}", poi.direccion);
    }
}
