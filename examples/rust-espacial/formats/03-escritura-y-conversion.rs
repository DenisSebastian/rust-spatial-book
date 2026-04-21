// Fuente: formats.qmd:84-115
// Parte: Rust Espacial
// Seccion: Formatos Espaciales > Escritura y Conversión
// Bloque: 03
use gdal::DriverManager;

fn guardar_shapefile() -> gdal::errors::Result<()> {
    // Obtener el driver "ESRI Shapefile"
    let driver = DriverManager::get_driver_by_name("ESRI Shapefile")?;

    // Crear el dataset
    let mut dataset = driver.create_vector_only("salida.shp")?;

    // Crear la capa
    let mut layer = dataset.create_layer(Default::default())?;

    // Definir un campo de texto "nombre"
    let field_defn = gdal::vector::FieldDefn::new("nombre", gdal::vector::OGRFieldType::OFTString)?;
    field_defn.set_width(50);
    layer.create_defn_fields(&[field_defn])?;

    // Crear una feature con geometría de punto
    let mut feature = gdal::vector::Feature::new(layer.defn())?;

    // Punto: Santiago de Chile (WGS84)
    let punto = gdal::vector::Geometry::from_wkt("POINT (-70.6693 -33.4489)")?;
    feature.set_geometry(punto)?;
    feature.set_field_string("nombre", "Santiago")?;

    // Escribir la feature a la capa
    feature.create(&layer)?;

    Ok(())
}
