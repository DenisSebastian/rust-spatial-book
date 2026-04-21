# Fuente: h3o-R.qmd:53-63
# Parte: Integracion con R
# Seccion: Ejemplo de Flujo de Trabajo > Visualización (Celdas a Polígonos)
# Bloque: 04
# Reemplazar la geometría de puntos con los hexágonos
h3_cells <- pnts |>
  mutate(
    h3 = h3_from_points(geometry, 4),
    geometry = st_as_sfc(h3)
  )

# Plotear los hexágonos
plot(st_geometry(h3_cells))
