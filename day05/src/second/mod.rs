#[cfg(test)]
pub mod test;

use super::{vector::Vector, Grid, Line, Position};
use eyre::Result as EyreResult;
use regex::Regex;
use std::iter::repeat;
use std::num::ParseIntError;

pub fn challenge(input: &str) -> EyreResult<usize> {
    let pattern = Regex::new(include_str!("../pattern.regex"))?;

    let lines = input
        .lines()
        .flat_map(|line| {
            pattern.captures_iter(line).map(|captures| {
                Ok(Line {
                    a: Position {
                        x: captures["a_x"].parse()?,
                        y: captures["a_y"].parse()?,
                    },
                    b: Position {
                        x: captures["b_x"].parse()?,
                        y: captures["b_y"].parse()?,
                    },
                })
            })
        })
        .collect::<Result<Vec<Line>, ParseIntError>>()?;

    let mut grid = Grid::default();
    for line in lines {
        let x_vector = Vector::new(line.a.x, line.b.x).iter();
        let y_vector = Vector::new(line.a.y, line.b.y).iter();

        // vertical line
        if line.a.x == line.b.x {
            let range = repeat(line.a.x).zip(y_vector);
            for (x, y) in range {
                grid.increment_at(Position { x, y });
            }
        }
        // horizontal line
        else if line.a.y == line.b.y {
            let range = x_vector.zip(repeat(line.a.y));
            for (x, y) in range {
                grid.increment_at(Position { x, y });
            }
        }
        // diagonal line
        else if i32::abs(line.a.x as i32 - line.b.x as i32)
            == i32::abs(line.a.y as i32 - line.b.y as i32)
        {
            let range = (x_vector).zip(y_vector);
            for (x, y) in range {
                grid.increment_at(Position { x, y });
            }
        }
    }

    let count_2s_and_over: usize = grid
        .map
        .iter()
        .filter(|(_position, &tile)| tile > 1)
        .count();

    Ok(count_2s_and_over)
}
