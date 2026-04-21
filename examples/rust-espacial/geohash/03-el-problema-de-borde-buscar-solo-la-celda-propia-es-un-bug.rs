// Fuente: geohash.qmd:68-81
// Parte: Rust Espacial
// Seccion: Geohash e Indexación > El problema de borde: buscar solo la celda propia es un bug
// Bloque: 03
use geohash::{neighbors, Direction};

let hash = "66j9h";

// Obtener los 8 vecinos
let vecinos = neighbors(hash).expect("Hash inválido");

println!("Norte: {}", vecinos.n);
println!("Sur:   {}", vecinos.s);
println!("Este:  {}", vecinos.e);
println!("Oeste: {}", vecinos.w);
// ... y diagonales (ne, nw, se, sw)
