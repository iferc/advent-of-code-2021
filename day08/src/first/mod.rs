#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};

pub fn challenge(input: &str) -> EyreResult<usize> {
    let after_pipe_lines = input
        .lines()
        .map(|line| {
            line.split('|')
                .nth(1)
                .map(|after_pipe| after_pipe.trim().split_whitespace())
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| eyre!("Bad input data!"))?;

    let idunno = after_pipe_lines
        .into_iter()
        .map(|line| {
            line.filter_map(|display| {
                let segments_count = display.len();

                match segments_count {
                    2 | 4 | 3 | 7 => Some(segments_count),

                    _ => None,
                }
            })
            .count()
        })
        // .collect::<Vec<_>>();
        .sum();

    Ok(idunno)
}
