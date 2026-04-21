// Fuente: geos.qmd:54-57
// Parte: Rust Espacial
// Seccion: Las unidades dependen del CRS — esto es crítico > Simplify (Simplificación)
// Bloque: 04
let simplificada = geom.simplify(0.5).expect("Error simplificando");
// 0.5 = tolerancia (distancia máxima permitida entre la curva original y la simplificada)
