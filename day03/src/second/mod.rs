#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};

pub fn challenge(_input: &str) -> EyreResult<usize> {
    Err(eyre!("not yet implemented"))
}
