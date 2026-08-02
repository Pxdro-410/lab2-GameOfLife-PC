# Laboratorio 2: Conway's Game of Life (F1 Theme)

Este laboratorio implementa el **Juego de la Vida de Conway** en tiempo real sobre un `Framebuffer` personalizado en **Rust**, visualizado mediante la crate `minifb` con una temática inspirada en la **Fórmula 1**.

## Creado por Pedro Caso - 241286

---

## Demostración de Ejecución

*<img width="800" height="411" alt="GameOfLifeF1-PC" src="https://github.com/user-attachments/assets/037901c9-6057-437e-b5bc-3d29bcde1808" />
*

![Game of Life F1 Demo](demo.gif)


## Diseño de la pantalla inicial
<img width="997" height="980" alt="image" src="https://github.com/user-attachments/assets/b7ef1ffc-a7a0-4730-9db5-2227b525a2c5" />


## pixelart creado para generar el diseño
<img width="100" height="100" alt="pixil-frame-0 (3)" src="https://github.com/user-attachments/assets/2aa71bd3-cdaf-417e-add3-8d750222961e" />


---

## Características Principales

1. **Renderizado en Tiempo Real**: Matriz lógica de **100x100 celdas** escalada a **800x800 píxeles** a ~30 FPS usando exclusivamente la función `point(x, y)` del `Framebuffer`.
2. **Bordes Toroidales (`Wrap-Around`)**: Simulación continua sin fronteras finitas (las celdas que salen por un borde reaparecen en el lado opuesto).
3. **Colorido Dinámico por Edad**:
   - **Celdas Recién Nacidas**: Azul Eléctrico (`0x0066FF`).
   - **Celdas Vivas Maduras**: Celeste brillante (`0x75CAF7`).
   - **Fondo (Pista Nocturna)**: Azul Noche oscuro (`0x050B14`).
4. **Controles de Teclado**:
   - `Espacio`: Pausar / Reanudar la animación.
   - `R`: Reiniciar el tablero al estado inicial.
   - `ESC`: Salir del programa.

---

## Organismos Clásicos Implementados (`src/organisms.rs`)

El proyecto incluye **11 organismos genéricos** del Juego de la Vida clasificados en:

* **Vidas Fijas (*Still Lifes*)**:
  - `Block` (Bloque 2x2)
  - `Beehive` (Colmena)
  - `Loaf` (Pan / Hogaza)
  - `Boat` (Bote)
* **Osciladores (*Oscillators*)**:
  - `Blinker` (Parpadeador)
  - `Toad` (Sapo)
  - `Beacon` (Faro)
  - `Pulsar` (Púlsar de 48 celdas)
* **Naves / Espacianaves (*Spaceships & Gliders*)**:
  - `Glider` (Planeador diagonal)
  - `LWSS` (Lightweight Spaceship)
  - `MWSS` (Mediumweight Spaceship)

---

## Elementos Temáticos de F1 (`src/pattern_f1.rs`)

- **Monoplazas de F1**: Siluetas detalladas de autos de F1 (alerones, nariz, cockpit y 4 ruedas).
- **Logotipo de F1**: Emblema icónico "F1" entramado.
- **Trofeos F1**: Copas de premio distribuidas en el circuito.
- **Línea de Meta**: Franja a cuadros horizontal a la derecha.
- **Línea Divisoria Central**: Pista vertical punteada que divide el tablero ($x = 41$).

---

## Reglas de Conway Aplicadas

Para cada turno / generación:
1. **Subpoblación**: Una celda viva con menos de 2 vecinos vivos muere.
2. **Supervivencia**: Una celda viva con 2 o 3 vecinos vivos sobrevive (pasa a estado maduro Celeste).
3. **Sobrepoblación**: Una celda viva con más de 3 vecinos vivos muere.
4. **Reproducción**: Una celda muerta con exactamente 3 vecinos vivos nace (se crea en estado recién nacido Azul).

---

## Requisitos y Ejecución

### Pre-requisitos
- Tener instalado [Rust y Cargo](https://www.rust-lang.org/).

### Ejecutar el Proyecto
```bash
git clone https://github.com/Pxdro-410/lab2-GameOfLife-PC.git
cd lab2-GameOfLife-PC
cargo run
```

---

## Estructura del Código

```text
lab2-GameOfLife-PC/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs          # Bucle principal de juego, ventana minifb y eventos
    ├── framebuffer.rs   # Estructura del buffer de pantalla y función point(x, y)
    ├── game_of_life.rs  # Motor de Conway, reglas, matriz toroidal y renderizado
    ├── organisms.rs     # Biblioteca modular de los 11 organismos de Conway
    └── pattern_f1.rs    # Dibujos y layout gráfico exclusivo con temática F1
```
