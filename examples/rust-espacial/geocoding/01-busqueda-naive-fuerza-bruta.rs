// Fuente: geocoding.qmd:18-79
// Parte: Rust Espacial
// Seccion: Geocodificación > Búsqueda Naive (Fuerza Bruta)
// Bloque: 01
use osmpbfreader::{OsmPbfReader, OsmObj};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Abrir la base de datos local (archivo PBF)
    let path = std::path::Path::new("datos/chile-latest.osm.pbf");
    let file = File::open(&path)?;
    let mut pbf = OsmPbfReader::new(file);

    let calle_objetivo = "Avenida Providencia";
    let numero_objetivo = "1234";

    println!("Buscando {} #{} en la base local...", calle_objetivo, numero_objetivo);

    // 2. Iterar y filtrar
    // Nota: Esto lee todo el archivo. Para producción, necesitarías un índice.
    let encontrados: Vec<OsmObj> = pbf.iter()
        .map(|r| r.unwrap())
        .filter(|obj| {
            // Verificamos si es un Nodo (punto) o Way (polígono/edificio)
            // Y si tiene los tags coincidentes
            let tags = obj.tags();
            
            // Usamos un match flexible para la calle (case insensitive sería ideal en un caso real)
            let match_calle = tags.get("addr:street")
                .map(|s| s.contains(calle_objetivo)) // Contains es más flexible que ==
                .unwrap_or(false);
                
            let match_numero = tags.get("addr:housenumber")
                .map(|n| n == numero_objetivo)
                .unwrap_or(false);

            match_calle && match_numero
        })
        .collect();

    // 3. Mostrar resultados
    if encontrados.is_empty() {
        println!("No se encontró la dirección.");
    } else {
        for obj in encontrados {
            let id = obj.id();
            let tags = obj.tags();
            
            // Para obtener Lat/Lon, depende del tipo de objeto.
            // Si es Node, tiene lat/lon directos.
            // Si es Way, necesitamos sus Nodos (requiere indexación previa, ver osmpbfreader docs)
            if let OsmObj::Node(n) = obj {
                println!("Encontrado: {} (lat: {}, lon: {})", 
                    tags.get("name").unwrap_or(&"Sin nombre".to_string()),
                    n.lat(), n.lon()
                );
            } else {
                println!("Encontrado objeto complejo (Way/Relation): {:?}", id);
            }
        }
    }
    
    Ok(())
}
