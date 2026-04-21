// Fuente: geohash.qmd:44-52
// Parte: Rust Espacial
// Seccion: Geohash e Indexación > Operaciones Básicas > Decodificar (Decode)
// Bloque: 02
use geohash::decode;

let hash = "66j9";
let (coord, error_x, error_y) = decode(hash).expect("Hash inválido");

println!("Centro: {}, {}", coord.x, coord.y);
println!("Error: +/- {}, +/- {}", error_x, error_y);
