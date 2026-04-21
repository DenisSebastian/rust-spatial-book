# project_1

Fuente: `project_1.qmd`

## Como ejecutar

Los ejemplos de este proyecto no usan crates externas. Puedes compilarlos directamente con `rustc`.

Ejecucion rapida desde esta carpeta:

```bash
rustc 04-probemoslo.rs
./04-probemoslo
```

`rustc` compila el archivo y crea el ejecutable; la salida del programa aparece al ejecutar el binario generado.

Puedes repetir el mismo patron con cualquier otro archivo:

```bash
rustc 11-el-flujo-completo.rs
./11-el-flujo-completo
```

Archivos generados:

- `01-el-atomo-espacial-el-punto-point.rs` | `rust` | líneas 29-45 | Introducción > 1. El Átomo Espacial: El Punto (`Point`)
- `02-construyendo-geometrias-el-poligono-polygon.rs` | `rust` | líneas 56-63 | Introducción > 2. Construyendo Geometrías: El Polígono (`Polygon`)
- `03-dando-vida-calculo-de-area-impl.rs` | `rust` | líneas 73-109 | Introducción > 3. Dando Vida: Cálculo de Área (`impl`)
- `04-probemoslo.rs` | `rust` | líneas 115-130 | Introducción > 3. Dando Vida: Cálculo de Área (`impl`) > Probémoslo
- `05-el-contrato-espacial-trait.rs` | `rust` | líneas 140-148 | Introducción > 4. El Contrato Espacial (`trait`)
- `06-el-contrato-espacial-trait.rs` | `rust` | líneas 152-164 | Introducción > 4. El Contrato Espacial (`trait`)
- `07-probemoslo-en-main.rs` | `rust` | líneas 172-195 | Introducción > 4. El Contrato Espacial (`trait`) > Probémoslo en `main`
- `08-validacion-y-robustez-evitando-geometrias-imposibles.rs` | `rust` | líneas 208-230 | Introducción > 5. Validación y Robustez: Evitando Geometrías Imposibles
- `09-probemoslo-en-main.rs` | `rust` | líneas 238-266 | Introducción > 5. Validación y Robustez: Evitando Geometrías Imposibles > Probémoslo en `main`
- `10-integracion-con-base-de-datos-simulacion.rs` | `rust` | líneas 274-288 | Introducción > 6. Integración con Base de Datos (Simulación)
- `11-el-flujo-completo.rs` | `rust` | líneas 296-338 | Introducción > 6. Integración con Base de Datos (Simulación) > El Flujo Completo

Soporte para ejecucion standalone:

- `support/project1.rs` contiene las definiciones acumuladas de `Point`, `Polygon` y `SpatialObject` usadas por los ejemplos `04`, `07`, `09` y `11`, para que puedan compilarse directamente con `rustc`.
