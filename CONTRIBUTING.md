# Contribuir a int64grep

Gracias por querer contribuir. Estas pautas ayudan a mantener el proyecto ordenado y a facilitar la revisión.

1) Reportar un bug o proponer una característica
- Abre un _issue_ con una descripción clara del problema o la propuesta.
- Incluye pasos para reproducir, la salida esperada y la salida actual.

2) Trabajar en una rama
- Haz un fork y crea una rama con nombre descriptivo: `feature/nombre-descriptivo` o `fix/descripcion`.

3) Formato y linting
- Aplica formato con `rustfmt` antes de enviar PRs:

```bash
cargo fmt
```

- Considera ejecutar `cargo clippy` para advertencias y mejorar la calidad del código:

```bash
cargo clippy -- -D warnings
```

4) Tests
- Añade/actualiza pruebas unitarias cuando añadas funcionalidad nueva o fixes bugs.
- Ejecuta las pruebas localmente antes de enviar el PR:

```bash
cargo test
```

5) Commits y PR
- Escribe commits atómicos y mensajes claros. Ejemplos: `fix: corregir conteo de líneas` o `feat: añadir búsqueda insensible a mayúsculas`.
- En el PR, describe qué hace el cambio, por qué y cómo verificarlo.

6) Revisión
- Se aceptan PRs cuando pasan las pruebas y la revisión de al menos una persona.

Gracias por tu aporte — ¡las contribuciones pequeñas también importan!
