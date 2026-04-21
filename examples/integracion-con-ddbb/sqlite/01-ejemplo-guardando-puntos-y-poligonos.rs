// Fuente: sqlite.qmd:25-69
// Parte: Integracion con DDBB
// Seccion: SQLite en Rust > La Crate `rusqlite` > Ejemplo: Guardando Puntos y Polígonos
// Bloque: 01
use rusqlite::{params, Connection, Result};
use geo::{Point, Polygon, LineString};
use wkt::ToWkt; // Trait para convertir geometrías a texto

fn main() -> Result<()> {
    // 1. Conexión (en memoria para el ejemplo)
    let conn = Connection::open_in_memory()?;

    // 2. Crear tabla
    conn.execute(
        "CREATE TABLE areas (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            geom TEXT NOT NULL
        )",
        [],
    )?;

    // --- Guardar un PUNTO ---
    let p = Point::new(10.0, 20.0);
    // Convertimos a WKT: "POINT(10 20)"
    let p_wkt = p.wkt_string(); 

    conn.execute(
        "INSERT INTO areas (name, geom) VALUES (?1, ?2)",
        params!["Estación Central", p_wkt],
    )?;

    // --- Guardar un POLÍGONO ---
    let exterior = LineString::from(vec![
        (0., 0.), (10., 0.), (10., 10.), (0., 10.), (0., 0.)
    ]);
    let poly = Polygon::new(exterior, vec![]);
    let poly_wkt = poly.wkt_string();

    conn.execute(
        "INSERT INTO areas (name, geom) VALUES (?1, ?2)",
        params!["Zona de Entrega", poly_wkt],
    )?;

    println!("Datos guardados exitosamente en SQLite.");
    Ok(())
}
