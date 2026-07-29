mod bmp;
mod framebuffer;
mod game_of_life;
mod line;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use minifb::{Key, Scale, Window, WindowOptions};
use std::thread;
use std::time::Duration;

fn main() {
    let grid_width = 100;
    let grid_height = 100;

    let mut framebuffer = Framebuffer::new(grid_width, grid_height);
    let mut game = GameOfLife::new(grid_width, grid_height);

    // prueba basica con Glider para verificar funcionamiento
    game.set_cell(1, 0, true);
    game.set_cell(2, 1, true);
    game.set_cell(0, 2, true);
    game.set_cell(1, 2, true);
    game.set_cell(2, 2, true);

    let mut window = Window::new(
        "Lab 2 - Game of Life F1 - PC",
        grid_width,
        grid_height,
        WindowOptions {
            resize: false,
            scale: Scale::X8, // Escala 100x100 vista en 800x800 para la visualizacion
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("Error al abrir ventana con minifb: {}", e);
    });

    // se limita la tasa de actualizacion a 30 FPS
    window.limit_update_rate(Some(Duration::from_micros(33000)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // se renderiza el estado actual al framebuffer con la funcion point
        game.render(&mut framebuffer);

        // se transmite el buffer de pixeles a la ventana de minifb
        window
            .update_with_buffer(&framebuffer.buffer, grid_width, grid_height)
            .unwrap();

        // se avanza a la siguiente generacion de conway
        game.update();

        // se añade un pequeño delay para observar la animación
        thread::sleep(Duration::from_millis(80));
    }
}
