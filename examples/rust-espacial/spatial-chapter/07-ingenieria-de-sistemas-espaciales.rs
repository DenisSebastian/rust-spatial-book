// Fuente: spatial_chapter.qmd:174-180
// Parte: Rust Espacial
// Seccion: Rust Espacial > Ingeniería de Sistemas Espaciales
// Bloque: 07
use rayon::prelude::*;

let projected: Vec<_> = points.par_iter()
    .map(|p| reproyectar(p))
    .collect();
