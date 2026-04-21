// Fuente: duckdb.qmd:29-91
// Parte: Integracion con DDBB
// Seccion: El Poder de `spatial` Extension > Ejemplo: Inserción de Objetos Espaciales
// Bloque: 01
use duckdb::{params, Connection, Result};
use geo::{Point, Polygon, LineString};
use wkt::ToWkt;

fn main() -> Result<()> {
    // 1. Conexión
    let conn = Connection::open_in_memory()?;

    // 2. Instalar y cargar extensión espacial
    // Nota: Esto requiere acceso a internet la primera vez para descargar la extensión
    conn.execute_batch("
        INSTALL spatial;
        LOAD spatial;
    ")?;

    // 3. Crear tabla con tipo GEOMETRY
    conn.execute_batch("
        CREATE TABLE spatial_data (
            id INTEGER,
            name VARCHAR,
            geom GEOMETRY
        );
    ")?;

    // --- Insertar un PUNTO ---
    let p = Point::new(-70.6, -33.4); // Santiago aprox
    let p_wkt = p.wkt_string(); // "POINT(-70.6 -33.4)"

    // Usamos ST_GeomFromText para convertir el string WKT a geometría nativa de DuckDB
    conn.execute(
        "INSERT INTO spatial_data VALUES (?, ?, ST_GeomFromText(?))",
        params![1, "Santiago", p_wkt],
    )?;

    // --- Insertar un POLÍGONO ---
    let poly = Polygon::new(
        LineString::from(vec![
            (0., 0.), (5., 0.), (5., 5.), (0., 5.), (0., 0.)
        ]),
        vec![],
    );
    let poly_wkt = poly.wkt_string();

    conn.execute(
        "INSERT INTO spatial_data VALUES (?, ?, ST_GeomFromText(?))",
        params![2, "Cuadrado Mágico", poly_wkt],
    )?;

    // 4. Verificación: Calcular área dentro de la DB
    // ¡DuckDB hace el cálculo, no Rust!
    let area: f64 = conn.query_row(
        "SELECT ST_Area(geom) FROM spatial_data WHERE id = 2",
        [],
        |row| row.get(0),
    )?;

    println!("El área del polígono calculada por DuckDB es: {}", area);
    // Output: 25.0

    Ok(())
}
