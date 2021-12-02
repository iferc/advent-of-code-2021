#[cfg(test)]
pub mod test;

use super::Movement;
use eyre::{eyre, Result as EyreResult};

#[derive(Default)]
pub struct Position {
    depth: usize,
    horizontal_position: usize,
    aim: usize,
}

pub fn challenge(input: &str) -> EyreResult<usize> {
    let movements = input
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<Vec<Movement>, _>>();

    let result = match movements {
        Err(error) => return Err(eyre!("{}", error)),

        Ok(movements) => movements
            .iter()
            .fold(Position::default(), |mut state, movement| match movement {
                Movement::Forward(amount) => {
                    state.horizontal_position += amount;
                    state.depth += amount * state.aim;
                    state
                }
                Movement::Up(amount) => {
                    state.aim -= amount;
                    state
                }
                Movement::Down(amount) => {
                    state.aim += amount;
                    state
                }
            }),
    };

    Ok(result.horizontal_position * result.depth)
}
