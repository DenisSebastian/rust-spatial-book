// Fuente: formats.qmd:56-75
// Parte: Rust Espacial
// Seccion: Formatos Espaciales > Lectura de Datos Vectoriales > Ejemplo: Leyendo un CSV con Latitud/Longitud
// Bloque: 02
use gdal::Dataset;

fn main() -> gdal::errors::Result<()> {
    // Opciones para decirle a GDAL qué columnas son la geometría
    // "X_POSSIBLE_NAMES=lon,x" y "Y_POSSIBLE_NAMES=lat,y" son defaults comunes
    let path = "datos/ubicaciones.csv";
    
    // GDAL intenta abrir CSVs directamente
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;

    for feature in layer.features() {
        // Acceso igual que con otros formatos
        let geom = feature.geometry();
        // ...
    }
    Ok(())
}
