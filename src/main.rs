use macroquad::prelude::*;

mod config;
mod grid;
use config::*;
use grid::{ExitCode, Grid, Solver};

fn window_config() -> Conf {
    Conf {
        window_title: "Sudoku Solver".to_owned(),
        window_width: WINDOW_SHAPE.x as i32,
        window_height: WINDOW_SHAPE.y as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut solver = Solver::new(Grid::random(10));

    let mut paused = true;
    let mut next_step = false;

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_key_pressed(KeyCode::Right) {
            next_step = true;
        }

        if !paused || next_step {
            next_step = false;

            if solver.solve_step() == ExitCode::Success {
                paused = true;
            }

            // println!("{:?}", solver.solve());
            // paused = true;
        }

        clear_background(WHITE);

        solver.draw((WINDOW_SHAPE.x as usize, WINDOW_SHAPE.y as usize));

        next_frame().await
    }
}
