use crate::game_of_life::GameOfLife;

// creacion de organismos clasicos del juegod e la vida
// Cada función recibe la referencia mutable al juego y las coordenadas (x, y) del origen o esquina superior izquierda.


// organismos still lives 
pub fn add_block(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [
        (0, 0), (1, 0),
        (0, 1), (1, 1),
    ];
    for &(dx, dy) in &pattern {
        game.set_cell(x + dx, y + dy, true);
    }
}

pub fn add_beehive(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [
                (1, 0), (2, 0),
        (0, 1),                 (3, 1),
                (1, 2), (2, 2),
    ];
    for &(dx, dy) in &pattern {
        game.set_cell(x + dx, y + dy, true);
    }
}


