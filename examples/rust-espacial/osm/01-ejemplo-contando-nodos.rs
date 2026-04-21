// Fuente: osm.qmd:18-39
// Parte: Rust Espacial
// Seccion: OpenStreetMap (OSM) > Lectura de Archivos PBF > Ejemplo: Contando Nodos
// Bloque: 01
use osmpbfreader::OsmPbfReader;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let archivo = File::open("chile-latest.osm.pbf")?;
    let mut pbf = OsmPbfReader::new(archivo);
    
    let mut contador_nodos = 0;
    
    // iter() decodifica los bloques del PBF en paralelo si es posible
    for obj in pbf.iter() {
        let obj = obj?;
        if obj.is_node() {
            contador_nodos += 1;
        }
    }
    
    println!("Se encontraron {} nodos", contador_nodos);
    Ok(())
}
