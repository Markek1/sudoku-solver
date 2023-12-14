use macroquad::prelude::*;

use crate::config::FONT_SIZE;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExitCode {
    Success,
    Failure,
    InProgress,
}

pub struct Grid {
    cells: [Option<i32>; 81],
    fixed: [bool; 81],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [None; 81],
            fixed: [false; 81],
        }
    }

    pub fn random(n_fixed: i32) -> Self {
        let mut grid = Self::new();

        for _ in 0..n_fixed {
            let mut index = rand::gen_range(0, 81);

            while grid.fixed[index] {
                index = rand::gen_range(0, 81);
            }

            let mut value = rand::gen_range(1, 10);

            while !grid.row_can_add(index % 9, index / 9, value)
                || !grid.column_can_add(index % 9, index / 9, value)
                || !grid.square_can_add(index % 9, index / 9, value)
            {
                value = rand::gen_range(1, 10);
            }

            grid.cells[index] = Some(value);
            grid.fixed[index] = true;
        }

        grid
    }

    fn two_d_to_index(x: usize, y: usize) -> usize {
        x + y * 9
    }

    fn row_can_add(&self, x: usize, y: usize, value: i32) -> bool {
        for i in 0..9 {
            let index = Grid::two_d_to_index(i, y);
            if self.cells[index] == Some(value) {
                return false;
            }
        }
        true
    }

    fn column_can_add(&self, x: usize, y: usize, value: i32) -> bool {
        for i in 0..9 {
            let index = Grid::two_d_to_index(x, i);
            if self.cells[index] == Some(value) {
                return false;
            }
        }
        true
    }

    fn square_can_add(&self, x: usize, y: usize, value: i32) -> bool {
        let square_x = x / 3;
        let square_y = y / 3;
        for i in 0..3 {
            for j in 0..3 {
                let index = Grid::two_d_to_index(square_x * 3 + j, square_y * 3 + i);
                if self.cells[index] == Some(value) {
                    return false;
                }
            }
        }
        true
    }
}

pub struct Solver {
    grid: Grid,
    current: (usize, usize),
    start_nums: [i32; 81],
}

impl Solver {
    pub fn new(grid: Grid) -> Solver {
        Solver {
            grid,
            current: (0, 0),
            start_nums: [1; 81],
        }
    }

    pub fn try_add(&mut self, x: usize, y: usize, value: i32) -> ExitCode {
        let g = &mut self.grid;
        let index = Grid::two_d_to_index(x, y);

        if g.cells[index].is_some()
            || !g.row_can_add(x, y, value)
            || !g.column_can_add(x, y, value)
            || !g.square_can_add(x, y, value)
        {
            return ExitCode::Failure;
        }

        g.cells[index] = Some(value);

        ExitCode::Success
    }

    pub fn solve(&mut self) -> ExitCode {
        loop {
            match self.solve_step() {
                ExitCode::Success => return ExitCode::Success,
                ExitCode::Failure => {
                    if self.current == (0, 0) {
                        return ExitCode::Failure;
                    }
                }
                ExitCode::InProgress => (),
            }
        }
    }

    fn increment_indices(&mut self) {
        self.current = match self.current {
            (8, y) => (0, y + 1),
            (x, y) => (x + 1, y),
        };
    }

    fn decrement_indices(&mut self) {
        self.current = match self.current {
            (0, y) => (8, y - 1),
            (x, y) => (x - 1, y),
        };
    }

    pub fn solve_step(&mut self) -> ExitCode {
        let (x, y) = self.current;
        let index = Grid::two_d_to_index(x, y);

        println!("{:?}, start_num: {}", self.current, self.start_nums[index]);

        if self.current == (8, 8) && self.grid.cells[index].is_some() {
            return ExitCode::Success;
        }

        if self.grid.fixed[index] {
            self.increment_indices();
            return ExitCode::InProgress;
        }

        for num in self.start_nums[index]..=9 {
            if self.try_add(x, y, num) == ExitCode::Success {
                println!("Added {} to {:?}", num, self.current);
                self.start_nums[index] = num + 1;

                if self.current == (8, 8) {
                    return ExitCode::Success;
                }

                self.increment_indices();

                return ExitCode::InProgress;
            }
        }

        self.start_nums[index] = 1;
        self.decrement_indices();
        let (x, y) = self.current;
        let mut index = Grid::two_d_to_index(x, y);

        while self.grid.fixed[index] {
            self.decrement_indices();
            let (x, y) = self.current;
            index = Grid::two_d_to_index(x, y);
        }

        self.grid.cells[index] = None;

        ExitCode::Failure
    }

    pub fn draw(&self, window_dims: (usize, usize)) {
        let (window_width, window_height) = window_dims;
        let cell_width = window_width / 9;
        let cell_height = window_height / 9;

        for i in 0..9 {
            for j in 0..9 {
                let index = Grid::two_d_to_index(j, i);
                let cell = self.grid.cells[index];
                let cell_x = j * cell_width;
                let cell_y = i * cell_height;

                if let Some(value) = cell {
                    let text = format!("{}", value);
                    let text_size = measure_text(&text, None, FONT_SIZE as u16, 1.0);
                    let text_x = cell_x + (cell_width - text_size.width as usize) / 2;
                    let text_y = cell_y + (cell_height - text_size.height as usize);

                    if self.grid.fixed[index] {
                        draw_text(&text, text_x as f32, text_y as f32, FONT_SIZE, BLUE);
                    } else {
                        draw_text(&text, text_x as f32, text_y as f32, FONT_SIZE, BLACK);
                    }
                }

                draw_rectangle_lines(
                    cell_x as f32,
                    cell_y as f32,
                    cell_width as f32,
                    cell_height as f32,
                    1.0,
                    BLACK,
                );
            }
        }
    }
}
