use crate::framebuffer::Framebuffer;

pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<u8>, 
    pub live_color: u32, 
    pub born_color: u32, 
    pub dead_color: u32, 
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        GameOfLife {
            width,
            height,
            grid: vec![0; width * height],
            live_color: 0x75CAF7, // Celeste brillante para los vivos
            born_color: 0x0066FF, // Azul para los recien creados
            dead_color: 0x050B14, // Fondo oscuro para la pista
        }
    }

    #[allow(dead_code)]
    pub fn set_colors(&mut self, live_color: u32, born_color: u32, dead_color: u32) {
        self.live_color = live_color;
        self.born_color = born_color;
        self.dead_color = dead_color;
    }

    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = if alive { 2 } else { 0 };
        }
    }

    #[allow(dead_code)]
    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] > 0
        } else {
            false
        }
    }


    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            *cell = 0;
        }
    }

    // Cuenta los 8 vecinos de la celda (x, y) usando envolvente Toroidal
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

                if self.grid[ny * self.width + nx] > 0 {
                    count += 1;
                }
            }
        }

        count
    }

    // Se aplican las 4 reglas del juego de la vida de conway con diferenciación de color por edad
    pub fn update(&mut self) {
        let mut next_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let current_idx = y * self.width + x;
                let age = self.grid[current_idx];
                let is_alive = age > 0;

                next_grid[current_idx] = match (is_alive, neighbors) {
                    // Regla 1 y 3, muere por subpoblación cuando es menor a 2 o sobrepoblación cuando es mayor a 3
                    (true, n) if n < 2 || n > 3 => 0,
                    // Regla 2, supervivencia cuando tiene 2 o 3 vecinos mantiene vida
                    (true, 2) | (true, 3) => age.saturating_add(1),
                    // Regla 4, reproducción cuando tiene exactamente 3 vecinos, se crea una celda recién nacida
                    (false, 3) => 1,
                    // Permanece muerta
                    _ => 0,
                };
            }
        }

        self.grid = next_grid;
    }

    // Renderiza el estado actual utilizando EXCLUSIVAMENTE la función point() del Framebuffer
    pub fn render(&self, framebuffer: &mut Framebuffer) {
        framebuffer.set_background_color(self.dead_color);
        framebuffer.clear();

        let scale = (framebuffer.width / self.width).max(1);

        for y in 0..self.height {
            for x in 0..self.width {
                let age = self.grid[y * self.width + x];
                if age > 0 {
                    let color = if age == 1 { self.born_color } else { self.live_color };
                    framebuffer.set_current_color(color);

                    if scale == 1 {
                        framebuffer.point(x as i32, y as i32);
                    } else {
                        let base_x = (x * scale) as i32;
                        let base_y = (y * scale) as i32;
                        for dy in 0..scale as i32 {
                            for dx in 0..scale as i32 {
                                framebuffer.point(base_x + dx, base_y + dy);
                            }
                        }
                    }
                }
            }
        }
    }
}

