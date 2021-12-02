#[cfg(test)]
pub mod test;

use eyre::Result as EyreResult;

pub fn challenge(input: &str) -> EyreResult<usize> {
    let depths = input
        .split_whitespace()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut counter = 0;
    let mut previous_depth = None;
    for depth in depths {
        if let Some(previous_depth) = previous_depth {
            if previous_depth < depth {
                counter += 1;
            }
        }

        previous_depth = Some(depth);
    }

    Ok(counter)
}
