#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};
use std::collections::HashMap;
use std::num::ParseIntError;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

pub fn challenge(input: &str) -> EyreResult<usize> {
    let mut blocks = input.split("\n\n");

    let picks = blocks.nth(0).map(|picks| {
        picks
            .trim()
            .split(',')
            .map(|pick| pick.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
    });

    let boards = blocks
        .map(|block| {
            block
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.split_whitespace()
                        .enumerate()
                        .map(|(x, square)| Ok((square.parse::<usize>()?, Position { x, y })))
                        .collect::<Result<Vec<_>, ParseIntError>>()
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|board| board.into_iter().flatten().collect::<HashMap<_, _>>())
        .collect::<Vec<_>>();

    let mut board_results: Vec<Vec<Position>> = (0..boards.len()).map(|_| Vec::new()).collect();
    for pick in picks.unwrap()? {
        for (index, board) in boards.iter().enumerate() {
            if let Some(position) = board.get(&pick) {
                board_results[index].push(*position);
            }

            let result = find_winning_pattern(&board_results[index]);
            if result {
                let matched_sum = board.iter().fold(0, |total, (value, position)| {
                    total
                        + if board_results[index].contains(&position) {
                            0
                        } else {
                            *value
                        }
                });
                return Ok(matched_sum * pick);
            }
        }
    }

    Err(eyre!("did not find a winner winner chicken dinner"))
}

pub fn find_winning_pattern(board: &Vec<Position>) -> bool {
    for x in 0..5 {
        let vertical_match = (0..5).fold(true, |state, y| {
            if !board.contains(&Position { x, y }) {
                false
            } else {
                state
            }
        });

        if vertical_match {
            return true;
        }
    }

    for y in 0..5 {
        let horizontal_match = (0..5).fold(true, |state, x| {
            if !board.contains(&Position { x, y }) {
                false
            } else {
                state
            }
        });

        if horizontal_match {
            return true;
        }
    }

    false
}
