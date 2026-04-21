// Fuente: formats.qmd:24-48
// Parte: Rust Espacial
// Seccion: Formatos Espaciales > Lectura de Datos Vectoriales > Ejemplo: Leyendo un Shapefile o GeoPackage
// Bloque: 01
use gdal::Dataset;
use gdal::vector::LayerAccess;

fn main() -> gdal::errors::Result<()> {
    // Abre el archivo (GDAL detecta el formato automáticamente)
    let dataset = Dataset::open("datos/puntos.gpkg")?;

    // Accede a una capa (por índice o nombre)
    let mut layer = dataset.layer(0)?;

    // Itera sobre las features
    for feature in layer.features() {
        // Obtener valores de atributos
        let nombre: String = feature.field_as_string_by_name("nombre")?
            .unwrap_or_else(|| "Desconocido".to_string());

        // Obtener la geometría
        if let Some(geometry) = feature.geometry() {
            println!("Feature: {}, Geometría: {:?}", nombre, geometry.wkt()?);
        }
    }
    Ok(())
}
