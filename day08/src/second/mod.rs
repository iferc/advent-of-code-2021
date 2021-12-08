#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};
use std::collections::HashMap;

pub fn challenge(input: &str) -> EyreResult<usize> {
    let after_pipe_lines = input.lines().map(|line| {
        line.split('|')
            .map(|pipe_side| pipe_side.trim().split_whitespace())
    });

    for mut line in after_pipe_lines {
        let (mut left, right) = match (line.next(), line.next()) {
            (Some(left), Some(right)) => (left.collect::<Vec<_>>(), right),

            _ => return Err(eyre!("Bad input data!")),
        };

        left.sort();

        let mut map = HashMap::new();
        for digits in left {
            let digit_count = digits.len();
            for digit in digits.chars() {
                println!("digit: {}", digit);

                match digit_count {
                    n @ 2 | n @ 4 | n @ 3 | n @ 7 => {
                        let entry = map.entry(digit).or_insert(Vec::new());
                        entry.push((n, digits)) // replace (n, digits)
                    }

                    n @ _ => {
                        let entry = map.entry(digit).or_insert(Vec::new());
                        entry.push((n, digits))
                    }
                }
            }
        }

        // todo: make a char -> segment

        for digits in right {
            let possible_numbers = digits
                .chars()
                .map(|digit| map.get(&digit))
                .collect::<Vec<_>>();

            // println!("possible_numbers: {} -> {:#?}", digits, possible_numbers);
        }
    }

    todo!()
}

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg
