#[cfg(test)]
pub mod test;

use eyre::{eyre, Result as EyreResult};

pub fn median(list: &Vec<usize>) -> Option<usize> {
    let mut items = list.clone();
    items.sort();
    let mut items = items.into_iter();

    if items.len() % 2 == 0 {
        let middle = (items.len() / 2) - 1;
        let first = items.nth(middle);
        let second = items.nth(0);

        match (first, second) {
            (Some(first), Some(second)) => Some((first + second) / 2),
            _ => None,
        }
    } else {
        let middle = ((items.len() + 1) / 2) - 1;
        items.nth(middle)
    }
}

pub fn challenge(input: &str) -> EyreResult<usize> {
    let crab_submarines = input
        .split(',')
        .map(|ch| ch.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let position = match median(&crab_submarines) {
        Some(result) => result,

        None => return Err(eyre!("sadface")),
    };

    let fuel_cost = crab_submarines.into_iter().fold(0, |fuel, crab| {
        if crab < position {
            fuel + position - crab
        } else {
            fuel + crab - position
        }
    });

    Ok(fuel_cost)
}
