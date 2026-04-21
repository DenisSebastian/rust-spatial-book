// Fuente: geos.qmd:63-81
// Parte: Rust Espacial
// Seccion: Predicados (Relaciones Espaciales)
// Bloque: 05
let a = ...;
let b = ...;

// ¿Se intersectan de alguna manera?
let toca_o_cruza = a.intersects(&b).unwrap();

// ¿Está 'b' completamente dentro de 'a'?
let contiene = a.contains(&b).unwrap();

// ¿Son disjuntos (no tienen ningún punto en común)?
let separados = a.disjoint(&b).unwrap();

// ¿Se tocan (tienen puntos comunes solo en el borde)?
let tocan = a.touches(&b).unwrap();

// ¿Se cruzan (tienen puntos comunes interiores, pero no se contienen)?
let cruzan = a.crosses(&b).unwrap();
