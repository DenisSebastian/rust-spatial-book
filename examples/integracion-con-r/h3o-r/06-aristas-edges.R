# Fuente: h3o-R.qmd:81-88
# Parte: Integracion con R
# Seccion: Ejemplo de Flujo de Trabajo > Aristas (Edges)
# Bloque: 06
# Obtener aristas de las primeras 3 celdas
cell_edges <- h3_edges(h3s[1:3])

# Aplanar la lista y convertir a líneas sf
flat_edges <- flatten_edges(cell_edges)
st_as_sfc(flat_edges)
