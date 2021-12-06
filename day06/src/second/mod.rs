#[cfg(test)]
pub mod test;

use eyre::Result as EyreResult;

pub fn challenge(input: &str, days: usize) -> EyreResult<usize> {
    super::first::challenge(input, days)
}
