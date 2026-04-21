// Fuente: routing.qmd:55-68
// Parte: Rust Espacial
// Seccion: Routing y Distancias > Uso Básico > Ruteo Marítimo y Ferroviario
// Bloque: 02
use routrs::prelude::*;
use routrs::maritime::GEOGRAPH as maritime;
use routrs::railways::GEOGRAPH as railways;

let from: Geoloc = (/* ... */);
let to: Geoloc = (/* ... */);

// Para barcos
let (dist_mar, _, _) = maritime::shortest_path(&from, &to);

// Para trenes
let (dist_ren, _, _) = railways::shortest_path(&from, &to);
