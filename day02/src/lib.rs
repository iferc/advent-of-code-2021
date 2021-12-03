pub mod first;
pub mod second;

use eyre::{eyre, Result as EyreResult};

#[derive(Default)]
pub struct Position {
    pub depth: usize,
    pub horizontal: usize,
    pub aim: usize,
}

#[derive(Default)]
pub struct Submarine {
    pub position: Position,
}

impl Submarine {
    pub fn naive_movement<M>(self, movements: Vec<M>) -> EyreResult<Self>
    where
        M: TryInto<Movement>,
        M::Error: std::fmt::Display,
    {
        let mut position = self.position;

        for movement in movements {
            match movement.try_into() {
                Ok(Movement::Forward(amount)) => {
                    position.horizontal += amount;
                }
                Ok(Movement::Up(amount)) => {
                    position.depth -= amount;
                }
                Ok(Movement::Down(amount)) => {
                    position.depth += amount;
                }
                Err(error) => return Err(eyre!("{}", error)),
            }
        }

        Ok(Self { position })
    }

    pub fn aiming_movement<M>(self, movements: Vec<M>) -> EyreResult<Self>
    where
        M: TryInto<Movement>,
        M::Error: std::fmt::Display,
    {
        Ok(Self {
            position: movements
                .into_iter()
                .fold(Ok(self.position), |position, movement| {
                    if let Ok(mut position) = position {
                        match movement.try_into() {
                            Ok(Movement::Forward(amount)) => {
                                position.horizontal += amount;
                                position.depth += amount * position.aim;
                                Ok(position)
                            }
                            Ok(Movement::Up(amount)) => {
                                position.aim -= amount;
                                Ok(position)
                            }
                            Ok(Movement::Down(amount)) => {
                                position.aim += amount;
                                Ok(position)
                            }
                            Err(error) => Err(eyre!("{}", error)),
                        }
                    } else {
                        position
                    }
                })?,
        })
    }
}

#[derive(Clone)]
pub enum Movement {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl TryFrom<&str> for Movement {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components = value.split_whitespace().collect::<Vec<_>>();

        if components.len() != 2 {
            return Err(String::from(
                "malformed input, expected two components per line",
            ));
        }

        let amount = components[1]
            .parse()
            .map_err(|error| format!("{}", error))?;

        let movement = match (components[0], amount) {
            ("forward", amount) => Movement::Forward(amount),
            ("up", amount) => Movement::Up(amount),
            ("down", amount) => Movement::Down(amount),
            (direction, _) => {
                return Err(format!(
                    "malformed input, expected a direct of 'forward', 'up', or 'down' but got '{}'",
                    direction,
                ));
            }
        };

        Ok(movement)
    }
}
