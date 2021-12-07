#[cfg(test)]
pub mod test;

use eyre::Result as EyreResult;

pub fn mean(list: &Vec<usize>) -> usize {
    let total_items = list.len();
    let total = list.iter().sum::<usize>();

    total / total_items
}

pub fn fuel_calculation(crab_position: usize, new_position: usize) -> usize {
    let difference = crab_position.abs_diff(new_position);
    difference * (difference + 1) / 2
}

pub fn challenge(input: &str) -> EyreResult<usize> {
    let crab_submarines = input
        .split(',')
        .map(|ch| ch.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let position = mean(&crab_submarines);

    let fuel_cost = crab_submarines
        .into_iter()
        .map(|crab| fuel_calculation(crab, position))
        .sum();

    Ok(fuel_cost)
}
