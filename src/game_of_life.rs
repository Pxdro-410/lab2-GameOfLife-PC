use crate::framebuffer::Framebuffer;

pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<bool>,
    pub live_color: u32,
    pub dead_color: u32,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        GameOfLife {
            width,
            height,
            grid: vec![false; width * height],
            live_color: 0xFFFFFF, 
            dead_color: 0x000000, 
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = alive;
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x]
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            *cell = false;
        }
    }

    // Cuenta los 8 vecinos de la celda (x, y) 
    pub fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let w = self.width as i32;
        let h = self.height as i32;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = ((x as i32 + dx + w) % w) as usize;
                let ny = ((y as i32 + dy + h) % h) as usize;

                if self.grid[ny * self.width + nx] {
                    count += 1;
                }
            }
        }

        count
    }

    // Se aplican las 4 reglas del juego de la vida de conway
    pub fn update(&mut self) {
        let mut next_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let current_idx = y * self.width + x;
                let is_alive = self.grid[current_idx];

                next_grid[current_idx] = match (is_alive, neighbors) {
                    // Regla 1 y 3 muere por subpoblación cuando es menor a 2 o por sobrepoblacion, cuando es mayor a 3
                    (true, n) if n < 2 || n > 3 => false,
                    // regla 2, supervivencia cuando tiene 2 o 3 vecinos
                    (true, 2) | (true, 3) => true,
                    // Regla 4 nace por reproduccion cuando tiene exactamente 3 vecinoss
                    (false, 3) => true,
                    // De lo contrario mantiene su estado actual
                    (state, _) => state,
                };
            }
        }

        self.grid = next_grid;
    }

    // Renderiza el estado actual utilizando con la función point() del Framebuffer
    pub fn render(&self, framebuffer: &mut Framebuffer) {
        framebuffer.set_background_color(self.dead_color);
        framebuffer.clear();

        framebuffer.set_current_color(self.live_color);
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y * self.width + x] {
                    framebuffer.point(x as i32, y as i32);
                }
            }
        }
    }
}
