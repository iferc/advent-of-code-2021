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

    let total_rows = lines.len();
    if total_rows == 0 {
        return Err(eyre!("no input give"));
    }

    let transposed_bits = crate::collect_transpose::collect_transposed(lines);
    let total_columns = transposed_bits.len();
    let sums = transposed_bits
        .iter()
        .map(|col| col.iter().sum::<usize>())
        .collect::<Vec<_>>();

    let mut majority_high_bits: Vec<_> = Vec::with_capacity(total_columns);
    for sum in sums {
        majority_high_bits.push(if sum * 2 > total_rows { 1 } else { 0 });
    }

    // array of binary digits to final values
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut pow = 1;
    for gamma_bit in majority_high_bits.into_iter().rev() {
        gamma += pow * gamma_bit;
        epsilon += pow * if gamma_bit == 1 { 0 } else { 1 };
        pow *= 2;
    }

    Ok(gamma * epsilon)
}
