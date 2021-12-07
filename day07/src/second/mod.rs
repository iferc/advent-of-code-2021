#[cfg(test)]
pub mod test;

use eyre::Result as EyreResult;

pub fn mean(list: &Vec<usize>) -> usize {
    let total_items = list.len();
    let total = list.iter().fold(0, |total, item| total + *item);

    // rounding jank ftw
    total / total_items
}

pub fn fuel_calculation(n: usize) -> usize {
    (n * (n + 1)) / 2
}

pub fn challenge(input: &str) -> EyreResult<usize> {
    let crab_submarines = input
        .split(',')
        .map(|ch| ch.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let position = mean(&crab_submarines);

    let fuel_cost = crab_submarines.into_iter().fold(0, |fuel, crab| {
        if crab < position {
            fuel + fuel_calculation(position - crab)
        } else {
            fuel + fuel_calculation(crab - position)
        }
    });

    Ok(fuel_cost)
}
