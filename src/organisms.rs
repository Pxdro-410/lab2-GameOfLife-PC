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

// organismos oscillators

pub fn add_blinker(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [(0, 0), (1, 0), (2, 0)];
    for &(dx, dy) in &pattern {
        game.set_cell(x + dx, y + dy, true);
    }
}

pub fn add_toad(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [
                (1, 0), (2, 0), (3, 0),
        (0, 1), (1, 1), (2, 1),
    ];
    for &(dx, dy) in &pattern {
        game.set_cell(x + dx, y + dy, true);
    }
}

pub fn add_beacon(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [
        (0, 0), (1, 0),
        (0, 1), (1, 1),
                        (2, 2), (3, 2),
                        (2, 3), (3, 3),
    ];
    for &(dx, dy) in &pattern {
        game.set_cell(x + dx, y + dy, true);
    }
}

pub fn add_pulsar(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    for &(dx, dy) in &pattern {
        game.set_cell(x + dx, y + dy, true);
    }
}

