# Fuente: h3o-R.qmd:110-120
# Parte: Integracion con R
# Seccion: Rendimiento (Benchmarks) > Polígonos a Celdas (Polyfill)
# Bloque: 08
nc <- st_read(system.file("gpkg/nc.gpkg", package = "sf"), quiet = TRUE) |>
  st_transform(4326) |>
  st_geometry()

bench::mark(
  h3o = sfc_to_cells(nc, 5, "centroid"),
  h3jsr = h3jsr::polygon_to_cells(nc, 5),
  check = FALSE
)
