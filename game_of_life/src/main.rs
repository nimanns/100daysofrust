use std::fmt;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_cell(&self, x: isize, y: isize) -> Cell {
        let x = ((x % self.width as isize + self.width as isize) % self.width as isize) as usize;
        let y = ((y % self.height as isize + self.height as isize) % self.height as isize) as usize;
        self.cells[self.index(x, y)]
    }

    fn set_cell(&mut self, x: isize, y: isize, cell: Cell) {
        let x = ((x % self.width as isize + self.width as isize) % self.width as isize) as usize;
        let y = ((y % self.height as isize + self.height as isize) % self.height as isize) as usize;
        let idx = self.index(x, y);
        self.cells[idx] = cell;
    }

    fn next_generation(&self) -> Grid {
        let mut new_grid = Grid::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let live_neighbors = self.live_neighbor_count(x as isize, y as isize);
                let current_cell = self.get_cell(x as isize, y as isize);
                let new_cell = match (current_cell, live_neighbors) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Dead, _) => Cell::Dead,
                };
                new_grid.set_cell(x as isize, y as isize, new_cell);
            }
        }
        new_grid
    }

    fn live_neighbor_count(&self, x: isize, y: isize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx != 0 || dy != 0 {
                    if self.get_cell(x + dx, y + dy) == Cell::Alive {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = match self.get_cell(x as isize, y as isize) {
                    Cell::Alive => '█',
                    Cell::Dead => ' ',
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}

fn main() {
    let mut grid = Grid::new(20, 10);

    grid.set_cell(1, 0, Cell::Alive);
    grid.set_cell(2, 1, Cell::Alive);
    grid.set_cell(0, 2, Cell::Alive);
    grid.set_cell(1, 2, Cell::Alive);
    grid.set_cell(2, 2, Cell::Alive);

    loop {
        grid.print();
        grid = grid.next_generation();
        thread::sleep(Duration::from_millis(500));
        //clear
        println!("\x1B[2J\x1B[1;1H"); 
    }
}
