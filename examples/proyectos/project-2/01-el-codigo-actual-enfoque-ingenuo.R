# Fuente: project_2.qmd:33-49
# Parte: Proyectos
# Seccion: Sistema de Optimización Espacial con Indexación H3 > Parte 1: Diagnóstico del Problema > El Código Actual (Enfoque Ingenuo)
# Bloque: 01
buscar_pois_cercanos <- function(latitud, longitud, radio_metros) {
  # Busca POIs cercanos - VERSIÓN ACTUAL (LENTA)
  # Tiempo promedio: 45 segundos para 2.3M POIs

  punto_usuario <- sf::st_sfc(
    sf::st_point(c(longitud, latitud)),
    crs = 4326
  )

  # 2.3 MILLONES de cálculos de distancia — sf::st_distance() es C++
  # por dentro, pero sigue iterando sobre cada fila del data frame
  distancias <- sf::st_distance(base_datos_pois, punto_usuario)

  base_datos_pois[as.numeric(distancias) <= radio_metros, ]
}
