# Fuente: h3o-R.qmd:98-104
# Parte: Integracion con R
# Seccion: Rendimiento (Benchmarks) > Creación de Polígonos
# Bloque: 07
h3_strs <- as.character(h3s)
bench::mark(
  h3o = st_as_sfc(h3s),
  h3jsr = h3jsr::cell_to_polygon(h3_strs)
)
