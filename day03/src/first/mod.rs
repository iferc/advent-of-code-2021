#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};

pub fn challenge(input: &str) -> EyreResult<usize> {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '0' => Ok(0),
                    '1' => Ok(1),
                    _ => Err(eyre!("betrayed by invalid input")),
                })
                .collect::<Result<Vec<usize>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    if lines.len() == 0 {
        return Err(eyre!("no input give"));
    }

    let line_length = lines[0].len();
    let mut sums: Vec<usize> = (0..line_length).map(|_| 0).collect::<Vec<_>>();

    for line in &lines {
        for (index, value) in line.iter().enumerate() {
            sums[index] += value;
        }
    }

    let mut gamma_bits = (0..line_length).map(|_| 0).collect::<Vec<_>>();
    for (index, sum) in sums.iter().enumerate() {
        gamma_bits[index] = if sum * 2 > lines.len() { 1 } else { 0 };
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let mut pow = 1;
    for gamma_bit in gamma_bits.into_iter().rev() {
        gamma += pow * gamma_bit;
        epsilon += pow * if gamma_bit == 1 { 0 } else { 1 };
        pow *= 2usize;
    }

    Ok(gamma * epsilon)
}
