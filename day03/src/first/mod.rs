#[cfg(test)]
pub mod test;

use eyre::Result as EyreResult;

pub struct Thing {}

pub fn challenge(input: &str) -> EyreResult<usize> {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("omg, we've been betrayed by the input!!!"),
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let total_lines = lines.len();
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
