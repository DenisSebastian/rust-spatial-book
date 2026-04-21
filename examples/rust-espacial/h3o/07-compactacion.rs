// Fuente: h3o.qmd:156-171
// Parte: Rust Espacial
// Seccion: Funciones Principales > 7. Compactación
// Bloque: 07
use h3o::{CellIndex, Resolution};

// Supón un set de celdas (todas deben ser únicas y de la misma resolución)
let muchas_celdas: Vec<CellIndex> = vec![/* ... */];

// Compactar — agrupa celdas hermanas en su celda padre cuando es posible
// Falla si el conjunto tiene duplicados o celdas de distintas resoluciones
let celdas_compactadas: Vec<CellIndex> = CellIndex::compact(muchas_celdas)
    .expect("Las celdas deben ser únicas y de la misma resolución")
    .collect();

// Descompactar (expandir todo a la resolución objetivo)
let celdas_planas: Vec<CellIndex> =
    CellIndex::uncompact(celdas_compactadas, Resolution::Nine).collect();
