use std::fmt::Display;

struct Grid([[char; 9]; 9]);

impl Grid {
    fn new() -> Self {
        let mut grid = [['+'; 9]; 9];
        grid[4][4] = 'o';
        Self(grid)
    }
    fn set_impact(&mut self, x: f64, y: f64) {
        let x = x as usize;
        let y = y as usize;
        self.0[y][x] = 'x';
    }
}

pub struct Target {
    #[allow(dead_code)]
    x: f64,
    #[allow(dead_code)]
    y: f64,
    grid: Grid,
}

impl Target {
    pub fn new(x: f64, y: f64) -> Self {
        let mut grid: Grid = Grid::new();
        grid.set_impact(x, y);
        Self { x, y, grid }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        for row in self.grid.0.iter() {
            for cell in row.iter() {
                result.push(*cell);
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}
