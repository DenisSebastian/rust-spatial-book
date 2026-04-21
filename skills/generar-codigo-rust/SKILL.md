---
name: generar-codigo-rust
description: Genera código Rust para el libro "Rust Espacial". Usa este skill cuando necesites crear ejemplos de código Rust para explicar conceptos del libro, implementar funciones específicas, o desarrollar ejemplos prácticos de análisis espacial. El skill orquesta 4 agentes expertos que colaboran para producir código correcto, pedagógicamente adecuado, espacialmente relevante y bien documentado.
---

# Generar Código Rust para el Libro "Rust Espacial"

Esta skill genera código Rust de alta calidad para el libro digital, asegurando que cada ejemplo sea:
- **Pedagógicamente adecuado** para el nivel del capítulo
- **Técnicamente correcto** y idiomático en Rust
- **Espacialmente relevante** para análisis geoespacial
- **Bien documentado** en español

## Flujo de trabajo

Cuando solicites generar código Rust, el siguiente proceso se ejecuta automáticamente:

### 1. R-to-Rust-Bridge (Contextualización desde R)
Analiza el contexto desde la perspectiva de un usuario de R:
- Identifica equivalencias conceptuales entre R y Rust
- Sugiere analogías pedagógicas útiles
- Detecta posibles saltos cognitivos abruptos
- Propone comparaciones R ↔ Rust cuando sea relevante

### 2. Spatial-Domain-Expert (Validación espacial)
Valida que el código sea espacialmente correcto:
- Verifica que la operación espacial sea metodológicamente correcta
- Asegura que el caso de uso sea relevante para análisis espacial real
- Sugiere datasets o escenarios representativos
- Valida conceptos geoespaciales (geometrías, índices, superposiciones, etc.)

### 3. Rust-Expert-Reviewer (Implementación técnica)
Genera el código Rust idiomático y correcto:
- Valida sintaxis, semántica y estilo idiomático
- Implementa soluciones seguras y eficientes
- Usa crates apropiadas del ecosistema GeoRust
- Añade documentación técnica precisa

### 4. Pedagogical-Architect (Revisión pedagógica)
Valida la calidad pedagógica del código:
- Verifica que el nivel técnico coincida con el capítulo
- Asegura progresión didáctica adecuada
- Sugiere mejoras de explicación cuando sea necesario
- Valida que la documentación sea clara para el lector objetivo

## Formato de salida

El código generado sigue este formato:

```rust
// Comentario explicativo en español
// Indica el propósito de la función o estructura

use geo_types::{Point, Coordinate};
use geo::{Area, Coordinate as GeoCoordinate};

/// Título breve de la función
///
/// # Ejemplo
///
/// ```
/// // Ejemplo de uso con datos de ejemplo
/// let resultado = funcion_ejemplo(data);
/// ```
///
/// # Explicación técnica
/// Descripción del algoritmo o patrón usado
fn funcion_ejemplo(parametros: Tipo) -> Result<Tipo, Error> {
    // Implementación con comentarios explicativos
    todo!()
}
```

## Ejemplos de uso

**Ejemplo 1 - Estructura de datos:**
> "Necesito un ejemplo de estructura Point con métodos básicos para el capítulo de tipos geométricos"

**Ejemplo 2 - Algoritmo espacial:**
> "Genera código para calcular buffer alrededor de polígonos usando geos"

**Ejemplo 3 - Integración R-Rust:**
> "Crea una función extendr que reciba un sf y devuelva área calculada en Rust"

## Casos de uso

Usa esta skill cuando necesites:
- Ejemplos de código para secciones del libro
- Implementaciones de estructuras geométricas (Point, Polygon, LineString)
- Algoritmos de análisis espacial (buffer, convex_hull, intersection)
- Funciones de optimización y paralelización
- Integración R ↔ Rust con extendr/rextendr
- Ejemplos de manejo de errores y tipos de datos

## Restricciones

- El código debe ser compilable y funcional
- La documentación debe estar en español
- El nivel técnico debe coincidir con el capítulo objetivo
- Las operaciones espaciales deben ser metodológicamente correctas
- El código debe ser idiomático en Rust (no patrones de R trasplantados)
