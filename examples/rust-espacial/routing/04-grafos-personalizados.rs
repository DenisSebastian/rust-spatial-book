// Fuente: routing.qmd:97-117
// Parte: Rust Espacial
// Seccion: Routing y Distancias > Grafos Personalizados
// Bloque: 04
use routrs::prelude::*;
use routrs::json::*;

let json_data = r#"
{
    "geograph": "mi_red_personal",
    "nodes": [
        { "id": 0, "coordinates": [179.5, 51.3], "waypoints": [1, 2] },
        { "id": 1, "coordinates": [177.2, 52.1], "waypoints": [0] },
        { "id": 2, "coordinates": [178.0, 51.8], "waypoints": [0] }
    ]
}
"#;

let grafo_json: JsonGeograph = serde_json::from_str(json_data).unwrap();
let grafo: Geograph = grafo_json.into();

// Ahora úsalo igual que los built-ins
let (dist, _, _) = grafo.shortest_path(&(179.5, 51.3), &(177.2, 52.1));
