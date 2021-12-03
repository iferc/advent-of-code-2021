#[cfg(test)]
pub mod test;

use super::Submarine;
use eyre::Result as EyreResult;

pub fn challenge(input: &str) -> EyreResult<usize> {
    let submarine = Submarine::default().aiming_movement(input.lines().collect::<Vec<_>>())?;

    Ok(submarine.position.horizontal * submarine.position.depth)
}
