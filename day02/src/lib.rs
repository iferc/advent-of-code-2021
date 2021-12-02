pub mod first;
pub mod second;

#[derive(Default)]
pub struct Position {
    pub depth: usize,
    pub horizontal: usize,
    pub aim: usize,
}

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
