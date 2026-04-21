// Fuente: geos.qmd:155-197
// Parte: Rust Espacial
// Seccion: Spatial Join: Puntos en Polígonos
// Bloque: 12
use geos::Geometry;

fn main() -> geos::GResult<()> {
    // --- Comunas de Santiago (simplificadas como polígonos WKT) ---
    let comunas: Vec<(&str, Geometry)> = vec![
        ("Santiago Centro", Geometry::new_from_wkt(
            "POLYGON ((-70.68 -33.43, -70.63 -33.43, -70.63 -33.46, -70.68 -33.46, -70.68 -33.43))"
        )?),
        ("Providencia", Geometry::new_from_wkt(
            "POLYGON ((-70.63 -33.42, -70.59 -33.42, -70.59 -33.45, -70.63 -33.45, -70.63 -33.42))"
        )?),
        ("Ñuñoa", Geometry::new_from_wkt(
            "POLYGON ((-70.63 -33.45, -70.59 -33.45, -70.59 -33.48, -70.63 -33.48, -70.63 -33.45))"
        )?),
    ];

    // --- Puntos de interés ---
    let puntos: Vec<(&str, Geometry)> = vec![
        ("Palacio La Moneda",    Geometry::new_from_wkt("POINT (-70.654 -33.443)")?),
        ("Plaza Italia",         Geometry::new_from_wkt("POINT (-70.638 -33.438)")?),
        ("Estadio Nacional",     Geometry::new_from_wkt("POINT (-70.609 -33.466)")?),
        ("Aeropuerto (fuera)",   Geometry::new_from_wkt("POINT (-70.795 -33.393)")?),
    ];

    // --- Spatial join: O(n·m), suficiente para datasets pequeños ---
    println!("{:<25} | {}", "Punto", "Comuna");
    println!("{}", "-".repeat(45));

    for (nombre_punto, punto) in &puntos {
        let comuna = comunas
            .iter()
            .find(|(_, poligono)| poligono.contains(punto).unwrap_or(false));

        match comuna {
            Some((nombre_comuna, _)) => println!("{:<25} | {}", nombre_punto, nombre_comuna),
            None                     => println!("{:<25} | (sin cobertura)", nombre_punto),
        }
    }

    Ok(())
}
