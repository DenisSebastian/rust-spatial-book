// Fuente: formats.qmd:173-215
// Parte: Rust Espacial
// Seccion: Formatos Espaciales > Lectura de Datos Raster
// Bloque: 05
use gdal::Dataset;

fn leer_raster(ruta: &str) -> gdal::errors::Result<()> {
    let dataset = Dataset::open(ruta)?;

    // --- Metadatos del raster ---
    let (ancho, alto) = dataset.raster_size();
    let num_bandas = dataset.raster_count();

    println!("Dimensiones: {} x {} píxeles", ancho, alto);
    println!("Bandas: {}", num_bandas);

    // CRS en formato WKT
    let crs = dataset.spatial_ref()?;
    println!("EPSG: {:?}", crs.auth_code().ok());

    // Transformación geoespacial: [origen_x, res_x, rot_x, origen_y, rot_y, res_y]
    let gt = dataset.geo_transform()?;
    println!("Resolución: {:.4} x {:.4} grados", gt[1], gt[5].abs());
    println!("Esquina NW: ({:.4}, {:.4})", gt[0], gt[3]);

    // --- Leer valores de la banda 1 ---
    let banda = dataset.rasterband(1)?;
    println!("Tipo de dato: {:?}", banda.band_type());

    // Leer un bloque de 5x5 píxeles desde el origen
    let buffer = banda.read_as::<f32>((0, 0), (5, 5), (5, 5), None)?;
    println!("Primeros valores: {:?}", &buffer.data[..5]);

    // Estadísticas (si están precalculadas en el archivo)
    if let Ok((min, max, media, std)) = banda.compute_raster_statistics(false, None) {
        println!("Min={:.2}, Max={:.2}, Media={:.2}, Std={:.2}", min, max, media, std);
    }

    Ok(())
}

fn main() {
    // Ejemplo con un DEM de Chile (cualquier GeoTIFF funciona)
    leer_raster("dem_chile.tif").expect("No se pudo abrir el raster");
}
