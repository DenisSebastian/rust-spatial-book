# Fuente: geos.qmd:213-217
# Parte: Rust Espacial
# Seccion: Equivalencia con R
# Bloque: 13
library(sf)
# sf::st_join une por el predicado "intersects" por defecto
resultado <- st_join(puntos_sf, comunas_sf)
