# support

Esta carpeta no contiene ejemplos del capitulo; contiene soporte para que algunos ejemplos de `project-1` puedan compilarse como archivos standalone con `rustc`.

## Como usarlo

No ejecutes `project1.rs` como ejemplo principal. Los scripts `04`, `07`, `09` y `11` lo importan automaticamente con:

```rust
#[path = "support/project1.rs"]
mod project1;
```

Desde la carpeta superior (`examples/proyectos/project-1`), compila y ejecuta un ejemplo asi:

```bash
rustc 04-probemoslo.rs
./04-probemoslo
```

`project1.rs` reune las definiciones acumuladas de `Point`, `Polygon` y `SpatialObject` que en el libro aparecen repartidas a lo largo del capitulo.
