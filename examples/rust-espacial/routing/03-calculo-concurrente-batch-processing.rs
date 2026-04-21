// Fuente: routing.qmd:74-91
// Parte: Rust Espacial
// Seccion: Routing y Distancias > Cálculo Concurrente (Batch Processing)
// Bloque: 03
use routrs::concurrency::*;
use routrs::highways::GEOGRAPH as highways;
use routrs::prelude::*;

let legs: Vec<Leg<Geoloc>> = vec![
    Leg((-33.4489, -70.6693), (-33.3942, -70.7950)), // Santiago → Aeropuerto
    Leg((-33.4270, -70.6100), (-33.5094, -70.7647)), // Providencia → Maipú
    // ... miles más ...
];

// par_distance procesa en paralelo automáticamente
let results = highways.par_distance(&legs);

for (dist, path_type) in results {
    println!("Distancia calculada: {}", dist);
}
