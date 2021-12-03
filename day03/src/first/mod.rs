#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};

pub struct Thing {}

pub fn challenge(input: &str) -> EyreResult<usize> {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '0' => Ok(0),
                    '1' => Ok(1),
                    _ => Err(eyre!("omg, we've been betrayed by the input!!!")),
                })
                .collect::<Result<Vec<usize>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let total_lines = lines.len();

    if total_lines == 0 {
        return Err(eyre!("No lines givens, dafuq bro?"));
    }

    let line_length = lines[0].len();
    let mut sums: Vec<usize> = (0..line_length).map(|_| 0).collect::<Vec<_>>();

    for line in lines {
        for (index, value) in line.iter().enumerate() {
            sums[index] += value;
        }
    }

    let mut gamma_bits = (0..line_length).map(|_| false).collect::<Vec<_>>();
    for (index, sum) in sums.iter().enumerate() {
        gamma_bits[index] = sum * 2 > total_lines;
    }

    println!("sums: {:#?}", sums);
    println!("gamma_bits: {:#?}", gamma_bits);

    let mut gamma = 0;
    let mut epsilon = 0;
    for (index, gamma_bit) in gamma_bits.iter().rev().enumerate() {
        gamma += 2usize.pow(index as u32) * if *gamma_bit { 1 } else { 0 };
        epsilon += 2usize.pow(index as u32) * if *gamma_bit { 0 } else { 1 };
    }
    println!("gamma: {:#?}", gamma);
    println!("epsilon: {:#?}", epsilon);

    Ok(gamma * epsilon)
}
