use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::fmt;
use std::ops::RangeInclusive;
use std::{fmt::Display, io::BufRead};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Cell {
    Floor,
    OccupiedSeat,
    FreeSeat,
}

impl Cell {
    fn parse(c: char) -> Result<Cell> {
        match c {
            'L' => Ok(Cell::FreeSeat),
            '#' => Ok(Cell::OccupiedSeat),
            '.' => Ok(Cell::Floor),
            _ => Err(anyhow!("Failed to parse a Cell from: {}", c)),
        }
    }
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Cell>,
    width: i32,
    height: i32,
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get(x, y).unwrap();
                let char = match cell {
                    Cell::OccupiedSeat => '#',
                    Cell::FreeSeat => 'L',
                    Cell::Floor => '.',
                };
                write!(f, "{}", char)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: vec![],
            width: 0,
            height: 0,
        }
    }

    fn add_line(&mut self, line: &str) -> Result<()> {
        if self.width == 0 {
            self.width = line.len() as i32;
        }
        if line.len() as i32 != self.width {
            return Err(anyhow!("Adding a grid line with invalid length"));
        }
        for cell in line.chars().map(Cell::parse) {
            self.cells.push(cell?);
        }
        self.height += 1;
        Ok(())
    }

    fn get(&self, x: i32, y: i32) -> Option<Cell> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            let index = (y * self.width + x) as usize;
            Some(self.cells[index])
        }
    }

    fn set(&mut self, x: i32, y: i32, c: Cell) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        let index = (y * self.width + x) as usize;
        self.cells[index] = c;
    }

    fn map_grid<F>(&self, output: &mut Grid, mut f: F) -> bool
    where
        F: FnMut(&Grid, i32, i32, Cell) -> Cell,
    {
        let mut modified = false;
        self.cells().for_each(|(x, y, cell)| {
            let new_cell = f(self, x, y, cell);
            if cell != new_cell {
                modified = true
            }
            output.set(x, y, new_cell);
        });
        modified
    }

    fn part1_step(&self, output: &mut Grid) -> bool {
        self.map_grid(output, |grid, x, y, cell| {
            let occupied_neighbours = grid
                .neighbour_points(x, y)
                .map(|(x, y)| self.get(x, y).unwrap().clone())
                .filter(|n| *n == Cell::OccupiedSeat)
                .count();
            match cell {
                Cell::Floor => Cell::Floor,
                Cell::OccupiedSeat => {
                    if occupied_neighbours >= 4 {
                        Cell::FreeSeat
                    } else {
                        Cell::OccupiedSeat
                    }
                }
                Cell::FreeSeat => {
                    if occupied_neighbours > 0 {
                        Cell::FreeSeat
                    } else {
                        Cell::OccupiedSeat
                    }
                }
            }
        })
    }

    fn part2_step(&self, output: &mut Grid) -> bool {
        self.map_grid(output, |grid, x, y, cell| {
            let occupied_neighbours = grid
                .neighbour_points(x, y)
                .map(|(nx, ny)| self.first_seat_in_dir(x, y, nx - x, ny - y).clone())
                .filter(|n| *n == Cell::OccupiedSeat)
                .count();
            match cell {
                Cell::Floor => Cell::Floor,
                Cell::OccupiedSeat => {
                    if occupied_neighbours >= 5 {
                        Cell::FreeSeat
                    } else {
                        Cell::OccupiedSeat
                    }
                }
                Cell::FreeSeat => {
                    if occupied_neighbours > 0 {
                        Cell::FreeSeat
                    } else {
                        Cell::OccupiedSeat
                    }
                }
            }
        })
    }

    fn neighbour_points(&self, x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
        let x_range: RangeInclusive<i32> = (x - 1)..=(x + 1);
        let y_range: RangeInclusive<i32> = (y - 1)..=(y + 1);
        let width = self.width;
        let height = self.height;
        let is_in_grid = move |(x, y): &(i32, i32)| *x >= 0 && *x < width && *y >= 0 && *y < height;
        y_range
            .cartesian_product(x_range)
            .map(|(y, x)| (x, y))
            .filter(is_in_grid)
            .filter(move |(x_n, y_n)| *x_n != x || *y_n != y)
    }

    fn first_seat_in_dir(&self, src_x: i32, src_y: i32, dx: i32, dy: i32) -> Cell {
        let mut x = src_x + dx;
        let mut y = src_y + dy;
        while let Some(cell) = self.get(x, y) {
            match cell {
                Cell::OccupiedSeat => return Cell::OccupiedSeat,
                Cell::FreeSeat => return Cell::FreeSeat,
                Cell::Floor => {}
            };
            x += dx;
            y += dy;
        }
        Cell::Floor
    }

    fn cells<'a>(&'a self) -> impl Iterator<Item = (i32, i32, Cell)> + 'a {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| (x, y))
            .map(move |(x, y)| (x, y, self.get(x, y).unwrap().clone()))
    }

    fn count_occupied(&self) -> usize {
        self.cells()
            .filter(|(_, _, c)| *c == Cell::OccupiedSeat)
            .count()
    }
}

fn main() -> Result<()> {
    let mut initial_grid = Grid::new();
    for line in std::io::stdin().lock().lines() {
        initial_grid.add_line(&line?)?;
    }

    let mut grid = initial_grid.clone();
    let mut buffer = grid.clone();
    while grid.part1_step(&mut buffer) {
        std::mem::swap(&mut grid, &mut buffer);
    }
    println!(
        "Occupied seats at part 1 fixed point: {}",
        grid.count_occupied()
    );

    grid = initial_grid.clone();
    while grid.part2_step(&mut buffer) {
        std::mem::swap(&mut grid, &mut buffer);
    }
    println!(
        "Occupied seats at part 2 fixed point: {}",
        grid.count_occupied()
    );

    Ok(())
}
