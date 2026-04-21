// Fuente: project_2.qmd:476-489
// Parte: Proyectos
// Seccion: Sistema de Optimización Espacial con Indexación H3 > Parte 7: Ejercicios para el Lector > Ejercicio 3: Benchmark Propio
// Bloque: 09
use criterion::{criterion_main, criterion_group, Criterion};

fn benchmark_h3(c: &mut Criterion) {
    let indice = H3Index::construir(&todos_los_pois, Resolution::Nine);

    c.bench_function("h3_benchmark", |b| {
        b.iter(|| indice.buscar_en_radio(-33.4489, -70.6693, 500.0))
    });
}

criterion_group!(benches, benchmark_h3);
criterion_main!(benches);
