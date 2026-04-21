# Fuente: formats.qmd:220-227
# Parte: Rust Espacial
# Seccion: Formatos Espaciales > Equivalencia con R
# Bloque: 06
library(terra)
r <- rast("dem_chile.tif")
dim(r)          # filas, columnas, bandas
crs(r)          # sistema de coordenadas
res(r)          # resolución
values(r)[1:5]  # primeros valores
