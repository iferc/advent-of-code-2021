#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};

pub fn challenge(input: &str) -> EyreResult<usize> {
    let depths = input
        .split_whitespace()
        // this mapping is attempting to parse every line of text into a Result enum of a usize
        // value where each item becomes an Ok(usize) or a parse error
        .map(|line| line.parse::<usize>())
        // collect is performing magic here that converts a Vec of value Results into
        // a Result of a Vec of values so that we can validate the whole input at once
        .collect::<Result<Vec<_>, _>>()?;

    if depths.len() < 3 {
        return Err(eyre!("Challenge requires at least 3 lines of input"));
    }

    // zips will automatically truncate when one of the given iterators runs out of items
    // so this ensures we're matching the input patterns given by the challenge
    let zipped_depths = depths
        .iter()
        .zip(depths[1..].iter().zip(depths[2..].iter()))
        .map(|zipped_depths| zipped_depths.0 + zipped_depths.1 .0 + zipped_depths.1 .1)
        .collect::<Vec<_>>();

    let mut counter = 0;
    let mut previous_depth = None;
    for depth in zipped_depths {
        if let Some(previous_depth) = previous_depth {
            if previous_depth < depth {
                counter += 1;
            }
        }

        previous_depth = Some(depth);
    }

    Ok(counter)
}
