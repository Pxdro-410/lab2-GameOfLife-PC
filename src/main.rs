mod framebuffer;
mod game_of_life;
mod organisms;
mod pattern_f1;


use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use minifb::{Key, Scale, Window, WindowOptions};
use std::thread;
use std::time::Duration;

fn init_scene(game: &mut GameOfLife) {
    game.clear();

    // se carga la estructura temática base de F1 (Logo F1, pista, división central y meta)
    pattern_f1::load_f1_proposal(game);

    // se siembran los 11 organismos clásicos definidos en organisms.rs de forma distribuida:
    
    // organismos still lifes
    organisms::add_block(game, 5, 80);
    organisms::add_beehive(game, 15, 85);
    organisms::add_loaf(game, 30, 80);
    organisms::add_boat(game, 35, 90);

    // organismos oscillators
    organisms::add_blinker(game, 8, 40);
    organisms::add_toad(game, 12, 65);
    organisms::add_beacon(game, 28, 60);
    organisms::add_pulsar(game, 10, 48);

    // organismos spaceships
    organisms::add_glider(game, 2, 15);
    organisms::add_lwss(game, 50, 10);
    organisms::add_mwss(game, 50, 30);
}

fn main() {
    let grid_width = 100;
    let grid_height = 100;
    let window_width = 800;
    let window_height = 800;

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut game = GameOfLife::new(grid_width, grid_height);

    // Inicializar escena F1 con la distribución completa de organismos
    init_scene(&mut game);

    let mut window = Window::new(
        "Lab 2 - Game of Life F1 - PC",
        window_width,
        window_height,
        WindowOptions {
            resize: false,
            scale: Scale::X1, // Renderizado directo 800x800 para aspecto 1:1 perfectamente cuadrado
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("Error al abrir ventana con minifb: {}", e);
    });

    // Se limita la tasa de actualizacion a ~30 FPS
    window.limit_update_rate(Some(Duration::from_micros(33000)));


    let mut paused = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Permitir pausar con la tecla Espacio
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
        }

        // Permitir reiniciar el patron con la tecla R
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            init_scene(&mut game);
        }

        // Se renderiza el estado actual al framebuffer con la funcion point
        game.render(&mut framebuffer);

        // Se transmite el buffer de pixeles a la ventana de minifb
        window
            .update_with_buffer(&framebuffer.buffer, window_width, window_height)
            .unwrap();


        // Se avanza a la siguiente generacion si no esta pausado
        if !paused {
            game.update();
        }

        // Se añade un pequeño delay para observar la animacion fluidamente
        thread::sleep(Duration::from_millis(150));
    }
}
