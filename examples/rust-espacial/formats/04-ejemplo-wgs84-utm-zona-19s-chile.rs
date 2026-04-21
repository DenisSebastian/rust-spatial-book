// Fuente: formats.qmd:132-151
// Parte: Rust Espacial
// Seccion: Formatos Espaciales > Reproyección de CRS > Ejemplo: WGS84 → UTM Zona 19S (Chile)
// Bloque: 04
use proj::Proj;

fn main() -> Result<(), proj::ProjError> {
    // Crear conversor WGS84 → UTM 19S
    let conversor = Proj::new_known_crs("EPSG:4326", "EPSG:32719", None)?;

    // Santiago de Chile: (latitud, longitud) en WGS84
    // Nota: proj espera (longitud, latitud) — orden (x, y)
    let santiago_wgs84 = (-70.6693_f64, -33.4489_f64);

    let santiago_utm = conversor.convert(santiago_wgs84)?;

    println!("WGS84:   lon={:.4}, lat={:.4}", santiago_wgs84.0, santiago_wgs84.1);
    println!("UTM19S:  E={:.1} m, N={:.1} m", santiago_utm.0, santiago_utm.1);
    // UTM19S:  E=344871.2 m, N=6299148.3 m

    Ok(())
}
