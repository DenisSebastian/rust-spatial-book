# Fuente: h3o-R.qmd:31-47
# Parte: Integracion con R
# Seccion: Ejemplo de Flujo de Trabajo > De Puntos a Celdas H3
# Bloque: 03
library(h3o)
library(dplyr)
library(sf)
library(tibble)

# 1. Crear datos de ejemplo — coordenadas en Chile (área Santiago-Valparaíso)
xy <- data.frame(
  x = runif(100, -71.5, -69.5),
  y = runif(100, -34.5, -32.5)
)

pnts <- st_as_sf(xy, coords = c("x", "y"), crs = 4326)

# 2. Convertir puntos a H3 (resolución 5)
pnts |> mutate(h3 = h3_from_points(geometry, 5))
