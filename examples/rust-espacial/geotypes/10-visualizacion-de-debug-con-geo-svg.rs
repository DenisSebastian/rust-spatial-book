// Fuente: geotypes.qmd:168-194
// Parte: Rust Espacial
// Seccion: Datos Geoespaciales > Visualización de Debug con `geo-svg`
// Bloque: 10
use geo::{polygon, point};
use geo_svg::ToSvg;
use std::fs;

fn main() {
    // Polígono que representa una manzana en Santiago Centro
    let manzana = polygon![
        (x: -70.654, y: -33.441),
        (x: -70.651, y: -33.441),
        (x: -70.651, y: -33.444),
        (x: -70.654, y: -33.444),
        (x: -70.654, y: -33.441),
    ];

    let punto = point!(x: -70.652, y: -33.443);

    // Generar SVG y guardar en archivo
    let svg = manzana.to_svg()
        .with_stroke_width(0.0001)
        .with_fill_color("steelblue")
        .with_fill_opacity(0.4);

    fs::write("debug.svg", svg.to_string()).expect("No se pudo escribir SVG");
    println!("Abre debug.svg en el navegador para ver la geometría");
}
