// Fuente: project_2.qmd:311-359
// Parte: Proyectos
// Seccion: Sistema de Optimización Espacial con Indexación H3 > Parte 4: Implementación en Rust > Paso 4: Ejemplo de Uso
// Bloque: 06
fn main() {
    // Cargar POIs desde base de datos (simulado)
    let todos_los_pois = vec![
        Poi {
            id: 1,
            nombre: "La Cuadra".to_string(),
            lat: -33.4489,
            lon: -70.6693,
            tipo: "restaurante".to_string(),
        },
        Poi {
            id: 2,
            nombre: "Farmacias Ahumada".to_string(),
            lat: -33.4520,
            lon: -70.6700,
            tipo: "farmacia".to_string(),
        },
        // ... 2.3 MILLONES más
    ];

    // 1. Construir índice H3 (se ejecuta UNA VEZ al iniciar)
    println!("Construyendo índice H3...");
    let indice = H3Index::construir(&todos_los_pois, Resolution::Nine);
    println!("¡Índice construido!");

    // 2. Realizar consultas rápidas
    println!("\nBuscando restaurantes cerca de Providencia...");
    let restaurantes = indice.buscar_en_radio(
        -33.4489,  // latitud
        -70.6693,  // longitud
        500.0,     // 500 metros
    );

    println!("Encontrados {} restaurantes:", restaurantes.len());
    for restaurante in restaurantes {
        println!("  - {}", restaurante.nombre);
    }

    // 3. Consultas repetidas son instantáneas
    println!("\nBuscando farmacias cerca de Santiago Centro...");
    let farmacias = indice.buscar_en_radio(
        -33.4489,
        -70.6693,
        1000.0,
    );
    println!("Encontradas {} farmacias:", farmacias.len());
}
