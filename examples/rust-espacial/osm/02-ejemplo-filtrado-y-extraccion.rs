// Fuente: osm.qmd:45-78
// Parte: Rust Espacial
// Seccion: OpenStreetMap (OSM) > Lectura de Archivos PBF > Ejemplo: Filtrado y Extracción
// Bloque: 02
use osmpbfreader::{OsmPbfReader, OsmObj};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("chile-latest.osm.pbf");
    let r = File::open(&path)?;
    let mut pbf = OsmPbfReader::new(r);

    // .get_objs_and_deps es útil cuando necesitas reconstruir geometrías (vías necesitan sus nodos),
    // pero para puntos simples podemos iterar y filtrar directamente.
    
    println!("Buscando pubs...");
    
    let pubs: Vec<String> = pbf.iter()
        .map(|r| r.unwrap())
        .filter(|obj| {
            // Verificamos si tiene el tag amenity=pub
            obj.tags().contains("amenity", "pub")
        })
        .map(|obj| {
            // Extraer el nombre, o usar "Sin Nombre"
            obj.tags().get("name").map(|s| s.to_string()).unwrap_or("Sin Nombre".to_string())
        })
        .collect();

    println!("Encontrados {} pubs:", pubs.len());
    for nombre in pubs.iter().take(5) {
        println!("- {}", nombre);
    }
    
    Ok(())
}
