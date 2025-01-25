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

/// A simple "target" that manages an internal 9×9 `Grid`.
///
/// # Details
///
/// The `Target` object stores:
/// - `x, y` floats, which represent a particular impact location in the grid.
/// - An internal `Grid`, which is used to display or track that impact visually.
///
/// # Usage
///
/// 1. Create a new target via `Target::new(x, y)`.
/// 2. Print or display the target, which will show `'+'` for empty cells, `'o'` for the center, and `'x'` for the impact.
pub struct Target {
    #[allow(dead_code)]
    x: f64,
    #[allow(dead_code)]
    y: f64,
    grid: Grid,
}

impl Target {
    /// Creates a new `Target` at the specified `(x, y)` coordinates.
    ///
    /// # Parameters
    ///
    /// - `x` - The x-coordinate in the grid. Typically in the range `[0.0..8.0]`.
    /// - `y` - The y-coordinate in the grid. Typically in the range `[0.0..8.0]`.
    ///
    /// # Behavior
    /// - Internally calls `Grid::new()` to initialize a 9×9 grid.
    /// - Marks `'o'` at the center (4,4).
    /// - Calls `set_impact(x, y)` to place `'x'` at the impact coordinates.
    ///
    /// # Example
    /// ```
    /// use moa_quizz::Target;
    /// let target = Target::new(2.0, 3.0);
    /// println!("{}", target);
    /// // This will print a grid with 'x' at (2,3) and 'o' at (4,4).
    /// ```
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
