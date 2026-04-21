# Fuente: h3o-R.qmd:69-75
# Parte: Integracion con R
# Seccion: Ejemplo de Flujo de Trabajo > Centroides
# Bloque: 05
h3s <- h3_cells[["h3"]]
h3_centers <- h3_to_points(h3s)

plot(st_geometry(h3_cells))
plot(h3_centers, pch = 16, add = TRUE, col = "black")
