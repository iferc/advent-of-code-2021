#[cfg(test)]
pub mod test;

use super::Movement;
use eyre::{eyre, Result as EyreResult};

pub fn challenge(input: &str) -> EyreResult<usize> {
    let movements = input
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<Vec<Movement>, _>>();

    let result = match movements {
        Err(error) => return Err(eyre!("{}", error)),

        Ok(movements) => movements
            .iter()
            .fold((0, 0), |mut state, movement| match movement {
                Movement::Forward(amount) => {
                    state.0 += amount;
                    state
                }
                Movement::Up(amount) => {
                    state.1 -= amount;
                    state
                }
                Movement::Down(amount) => {
                    state.1 += amount;
                    state
                }
            }),
    };

    Ok(result.0 * result.1)
}
