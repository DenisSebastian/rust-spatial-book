// Fuente: geos.qmd:47-49
// Parte: Rust Espacial
// Seccion: Las unidades dependen del CRS — esto es crítico > Envelope (Bounding Box)
// Bloque: 03
let bbox = geom.envelope().expect("Error en envelope");
