// Fuente: project_1.qmd:296-338
// Parte: Proyectos
// Seccion: Introducción > 6. Integración con Base de Datos (Simulación) > El Flujo Completo
// Bloque: 11
#[path = "support/project1.rs"]
mod project1;

use project1::{Point, Polygon};

fn main() {
    // Imaginemos que esto viene de leer un archivo CSV
    let lote_de_datos = vec![
        // Polígono 1: Válido (Triángulo)
        vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(5.0, 10.0),
            Point::new(0.0, 0.0),
        ],
        // Polígono 2: Inválido (Línea)
        vec![Point::new(0.0, 0.0), Point::new(10.0, 10.0)],
    ];

    println!("Iniciando procesamiento por lotes...\n");

    for (i, datos) in lote_de_datos.iter().enumerate() {
        println!("Procesando registro #{}...", i + 1);
        procesar_datos_entrada(datos.clone());
        println!("---");
    }
}

fn procesar_datos_entrada(datos_crudos: Vec<Point>) {
    // 1. Intentar crear el polígono (Validación)
    // El 'if let' es una forma corta y elegante de hacer match solo si es Some
    if let Some(poly) = Polygon::new_checked(datos_crudos) {
        // 2. Calcular propiedades (Cómputo)
        let area = poly.area();
        println!("  ✅ Geometría válida detectada.");
        println!("  📊 Área calculada: {:.2}", area);

        // 3. Serializar para DB (Persistencia)
        let wkt = poly.to_wkt();
        println!(
            "  💾 Guardando en DB: INSERT INTO geom_table VALUES ('{}');",
            wkt
        );
    } else {
        println!("  ❌ Error: Geometría corrupta o inválida. Registro descartado.");
    }
}
