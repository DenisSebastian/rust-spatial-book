// Fuente: geocoding.qmd:93-109
// Parte: Rust Espacial
// Seccion: Geocodificación > Estrategias para Producción > Ejemplo conceptual con Tantivy
// Bloque: 02
// (Pseudocódigo conceptual)
let index = Index::create_in_dir(&path, schema)?;
let mut writer = index.writer(50_000_000)?;

// Indexar
for obj in pbf.iter() {
    if let Some(addr) = extraer_direccion(&obj) {
        writer.add_document(doc!(
            calle => addr.calle,
            numero => addr.numero,
            coords => addr.coords
        ));
    }
}
writer.commit()?;
