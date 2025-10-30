# int64grep

Int64grep es un pequeño proyecto en Rust que busca cadenas dentro de un archivo (similar a `grep`) y expone funciones de búsqueda utilizadas por el binario del paquete.

Características principales
- Búsqueda normal (sensible a mayúsculas/minúsculas)
- Búsqueda insensible a mayúsculas
- Helpers para contar/numeroar resultados

Estado
- Código de ejemplo/ejercicio educativo. Ideal para aprender cómo escribir pruebas en Rust y cómo construir utilidades CLI pequeñas.

Contenido
- `src/main.rs` - Binary wrapper (usa las funciones del crate)
- `src/lib.rs` - Lógica de búsqueda y pruebas unitarias

Requisitos
- Rust (rustc + cargo). Versiones recientes de la toolchain estable.

Cómo compilar

En PowerShell (Windows):

```powershell
Set-Location -Path "e:\\carlos\\Learn\\Rust\\proyectos_rust\\int64grep"
cargo build --release
```

Cómo ejecutar (ejemplos)

Desde el directorio del proyecto puedes ejecutar el binario con argumentos:

```powershell
# Ejecutar con cargo (pasar argumentos después de --)
cargo run -- "query" "ruta/al/archivo.txt"

# O usar directamente el ejecutable compilado (release)
.\target\release\int64grep "query" "ruta\\al\\archivo.txt"
```

Pruebas

Ejecuta las pruebas unitarias con:

```powershell
cargo test
```

Contribuir

Por favor revisa `CONTRIBUTING.md` para pautas sobre cómo abrir issues y enviar pull requests.

Licencia

Este proyecto está bajo la licencia MIT — ver el fichero `LICENSE`.

Contacto

Si quieres ayudar o mejorar el proyecto, abre un issue o un pull request con descripciones claras y pruebas cuando corresponda.
