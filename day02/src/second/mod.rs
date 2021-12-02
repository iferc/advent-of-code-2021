#[cfg(test)]
pub mod test;

use super::{Movement, Position};
use eyre::{eyre, Result as EyreResult};

pub fn challenge(input: &str) -> EyreResult<usize> {
    let movements = input
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<Vec<Movement>, _>>()
        .map_err(|error| eyre!("{}", error))?;

    let result = movements.iter().fold(
        Position::default(),
        |mut position, movement| match movement {
            Movement::Forward(amount) => {
                position.horizontal += amount;
                position.depth += amount * position.aim;
                position
            }
            Movement::Up(amount) => {
                position.aim -= amount;
                position
            }
            Movement::Down(amount) => {
                position.aim += amount;
                position
            }
        },
    );

    Ok(result.horizontal * result.depth)
}
