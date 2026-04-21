// Fuente: project_2.qmd:377 y project_2.qmd:476-489
// Parte: Proyectos
// Seccion: Sistema de Optimización Espacial con Indexación H3 > Parte 5: Resultados y Benchmark
// Bloque: complementario
//
// Script complementario para el ejemplo mencionado como:
//   cargo bench --bench h3_optimization
//
// Para ejecutarlo en un crate Cargo, guarda este archivo como:
//   benches/h3_optimization.rs
//
// Dependencias sugeridas:
//   h3o = "0.6"
//   criterion = "0.5"
//
// En Cargo.toml, agrega tambien:
//   [[bench]]
//   name = "h3_optimization"
//   harness = false

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use h3o::{CellIndex, LatLng, Resolution};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Poi {
    id: u64,
    nombre: String,
    lat: f64,
    lon: f64,
    tipo: String,
}

#[allow(dead_code)]
struct H3Index {
    celdas: HashMap<CellIndex, Vec<Poi>>,
    ubicacion: HashMap<u64, CellIndex>,
    resolucion: Resolution,
}

impl H3Index {
    pub fn construir(pois: &[Poi], resolucion: Resolution) -> Self {
        let mut celdas = HashMap::new();
        let mut ubicacion = HashMap::new();

        for poi in pois {
            let coord = LatLng::new(poi.lat, poi.lon).expect("coordenadas validas");
            let celda = coord.to_cell(resolucion);

            celdas
                .entry(celda)
                .or_insert_with(Vec::new)
                .push(poi.clone());
            ubicacion.entry(poi.id).or_insert(celda);
        }

        Self {
            celdas,
            ubicacion,
            resolucion,
        }
    }

    pub fn buscar_en_radio(&self, latitud: f64, longitud: f64, radio_metros: f64) -> Vec<&Poi> {
        let coord = LatLng::new(latitud, longitud).expect("coordenadas validas");
        let celda_centro = coord.to_cell(self.resolucion);
        let celdas_vecinas = celda_centro.grid_disk::<Vec<_>>(1);

        let mut resultados = Vec::new();
        for celda in celdas_vecinas {
            if let Some(pois) = self.celdas.get(&celda) {
                resultados.extend(pois.iter().filter(|poi| {
                    haversine_dist(latitud, longitud, poi.lat, poi.lon) <= radio_metros
                }));
            }
        }

        resultados
    }
}

fn buscar_en_radio_ingenuo(
    pois: &[Poi],
    latitud: f64,
    longitud: f64,
    radio_metros: f64,
) -> Vec<&Poi> {
    pois.iter()
        .filter(|poi| haversine_dist(latitud, longitud, poi.lat, poi.lon) <= radio_metros)
        .collect()
}

fn haversine_dist(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6_371_000.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    R * c
}

fn pois_de_prueba(total: usize) -> Vec<Poi> {
    let tipos = ["restaurante", "farmacia", "estacion_servicio", "cafeteria"];
    let ancho_grilla = (total as f64).sqrt().ceil() as usize;

    (0..total)
        .map(|i| {
            let fila = i / ancho_grilla;
            let columna = i % ancho_grilla;

            Poi {
                id: i as u64,
                nombre: format!("POI {i}"),
                lat: -33.56 + fila as f64 * 0.001,
                lon: -70.76 + columna as f64 * 0.001,
                tipo: tipos[i % tipos.len()].to_string(),
            }
        })
        .collect()
}

fn benchmark_h3(c: &mut Criterion) {
    let todos_los_pois = pois_de_prueba(50_000);
    let indice = H3Index::construir(&todos_los_pois, Resolution::Nine);

    let mut group = c.benchmark_group("busqueda_en_radio");

    group.bench_function("enfoque_ingenuo", |b| {
        b.iter(|| {
            buscar_en_radio_ingenuo(
                black_box(&todos_los_pois),
                black_box(-33.4489),
                black_box(-70.6693),
                black_box(500.0),
            )
        })
    });

    group.bench_function("indexacion_h3", |b| {
        b.iter(|| {
            indice.buscar_en_radio(black_box(-33.4489), black_box(-70.6693), black_box(500.0))
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_h3);
criterion_main!(benches);
