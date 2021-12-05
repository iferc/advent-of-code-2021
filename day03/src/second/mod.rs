#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};

pub fn challenge(input: &str) -> EyreResult<usize> {
    let lines = input
        .clone()
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

    let mut oxygen_lines = lines.clone();
    let mut scrubber_lines = lines.clone();

    // iterator through a copy of the given lines of input, filtering out records until only one remains
    let mut char_position = 0;
    while oxygen_lines.len() > 1 {
        let total_rows = oxygen_lines.len();
        let transposed_bits = crate::collect_transpose::collect_transposed(oxygen_lines.clone());
        let total_columns = transposed_bits.len();

        if total_columns < char_position {
            return Err(eyre!(
                "run out of input columns before filtering to one value"
            ));
        }

        let sums = transposed_bits
            .iter()
            .map(|col| col.iter().sum::<usize>())
            .collect::<Vec<_>>();

        let mut majority_high_bits: Vec<_> = Vec::with_capacity(total_columns);
        for sum in sums {
            let double_sum = sum * 2;
            majority_high_bits.push(if double_sum >= total_rows { 1 } else { 0 });
        }

        if let Some(bit) = majority_high_bits.iter().nth(char_position) {
            oxygen_lines = oxygen_lines
                .into_iter()
                .filter_map(|line| match line.iter().nth(char_position) {
                    Some(digit) => {
                        if *digit == *bit {
                            Some(line)
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect::<Vec<_>>();
        }

        char_position += 1;
    }

    // iterator through a copy of the given lines of input, filtering out records until only one remains
    let mut char_position = 0;
    while scrubber_lines.len() > 1 {
        let total_rows = scrubber_lines.len();
        let transposed_bits = crate::collect_transpose::collect_transposed(scrubber_lines.clone());
        let total_columns = transposed_bits.len();

        if total_columns < char_position {
            return Err(eyre!(
                "run out of input columns before filtering to one value"
            ));
        }

        let sums = transposed_bits
            .iter()
            .map(|col| col.iter().sum::<usize>())
            .collect::<Vec<_>>();

        let mut majority_low_bits: Vec<_> = Vec::with_capacity(total_columns);
        for sum in sums {
            let double_sum = sum * 2;
            majority_low_bits.push(if double_sum < total_rows { 1 } else { 0 });
        }

        if let Some(bit) = majority_low_bits.iter().nth(char_position) {
            scrubber_lines = scrubber_lines
                .into_iter()
                .filter_map(|line| match line.iter().nth(char_position) {
                    Some(digit) => {
                        if *digit == *bit {
                            Some(line)
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect::<Vec<_>>();
        }

        char_position += 1;
    }

    // array of binary digits to final values
    let oxygen_rating = {
        let mut result = 0;
        let mut pow = 1;
        for bit in oxygen_lines[0].iter().rev() {
            result += pow * bit;
            pow *= 2;
        }
        result
    };
    let scrubber_rating = {
        let mut result = 0;
        let mut pow = 1;
        for bit in scrubber_lines[0].iter().rev() {
            result += pow * bit;
            pow *= 2;
        }
        result
    };

    Ok(scrubber_rating * oxygen_rating)
}
