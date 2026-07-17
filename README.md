# Graficas_Bresenham

Proyecto en Rust que implementa un renderizador de framebuffer y una demostración de dibujo de líneas mediante el algoritmo de Bresenham.

## Descripción

Este proyecto crea un framebuffer de 800x600 píxeles, dibuja varios puntos y genera un archivo BMP llamado `output.bmp` como salida del programa.

## Estructura del proyecto

- `bresenham/`: carpeta principal del proyecto Rust.
  - `src/main.rs`: punto de entrada del programa.
  - `src/framebuffer.rs`: implementación del framebuffer.
  - `src/bmp.rs`: escritura de archivos BMP.
  - `src/line.rs`: implementación del algoritmo de Bresenham.
  - `Cargo.toml`: configuración del paquete.

## Requisitos

- Rust instalado.
- Cargo incluido con Rust.

## Ejecución

Desde la carpeta `bresenham`, ejecuta:

```bash
cargo run
```

Esto generará el archivo `output.bmp` en la carpeta del proyecto.

## Funcionalidades

- Crear un framebuffer en memoria.
- Limpiar el fondo del framebuffer.
- Dibujar puntos individuales.
- Renderizar el contenido a un archivo BMP.
- Mostrar una línea con el algoritmo de Bresenham.

## Notas

El programa actual genera una imagen BMP simple como demostración del funcionamiento del framebuffer y del dibujo de líneas.
