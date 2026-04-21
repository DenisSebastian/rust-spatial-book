# h3o-R

Fuente: `h3o-R.qmd`

## Como ejecutar

Todos los archivos de esta carpeta son scripts R. Instala primero los paquetes usados por el capitulo:

```r
install.packages(c("h3o", "sf", "dplyr", "bench"))
```

Luego ejecuta el archivo que quieras probar:

```bash
Rscript 03-de-puntos-a-celdas-h3.R
```

Los scripts `07` y `08` son benchmarks; pueden tardar mas y requieren `bench`. Los scripts con objetos `sf` necesitan que las dependencias nativas de `sf` esten instaladas en tu sistema.

Archivos generados:

- `01-instalacion.R` | `r` | líneas 14-16 | Instalación
- `02-instalacion.R` | `r` | líneas 20-23 | Instalación
- `03-de-puntos-a-celdas-h3.R` | `r` | líneas 31-47 | Ejemplo de Flujo de Trabajo > De Puntos a Celdas H3
- `04-visualizacion-celdas-a-poligonos.R` | `r` | líneas 53-63 | Ejemplo de Flujo de Trabajo > Visualización (Celdas a Polígonos)
- `05-centroides.R` | `r` | líneas 69-75 | Ejemplo de Flujo de Trabajo > Centroides
- `06-aristas-edges.R` | `r` | líneas 81-88 | Ejemplo de Flujo de Trabajo > Aristas (Edges)
- `07-creacion-de-poligonos.R` | `r` | líneas 98-104 | Rendimiento (Benchmarks) > Creación de Polígonos
- `08-poligonos-a-celdas-polyfill.R` | `r` | líneas 110-120 | Rendimiento (Benchmarks) > Polígonos a Celdas (Polyfill)
