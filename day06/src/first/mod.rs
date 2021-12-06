#[cfg(test)]
pub mod test;

use eyre::Result as EyreResult;
use itertools::Itertools;

#[derive(Debug)]
pub struct FishCollection {
    days_until_birth: usize,
    count_of_fish: usize,
}

pub fn group_fishies_by_days_remaining(input: &str) -> EyreResult<Vec<FishCollection>> {
    let mut unsorted_fish_days_remaining_until_next_birth = input
        .split(',')
        .map(|fish| fish.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    // this sort is required because itertools::group_by is weird
    // worse yet, sort can't be chained 😢
    unsorted_fish_days_remaining_until_next_birth.sort();

    let grouped_sorted_fish_days_remaining_until_next_birth =
        unsorted_fish_days_remaining_until_next_birth
            .into_iter()
            .group_by(|elt| *elt)
            .into_iter()
            .map(|(days_until_birth, grouping_of_fish)| FishCollection {
                days_until_birth,
                count_of_fish: grouping_of_fish.count(),
            })
            .collect::<Vec<_>>();

    Ok(grouped_sorted_fish_days_remaining_until_next_birth)
}

pub fn challenge(input: &str, days: usize) -> EyreResult<usize> {
    let mut fishies = group_fishies_by_days_remaining(input)?;

    for _day in 0..days {
        let mut births = 0;
        for fish in fishies.iter_mut() {
            if fish.days_until_birth == 0 {
                births += fish.count_of_fish;
                fish.days_until_birth = 6;
            } else {
                fish.days_until_birth -= 1;
            }
        }

        if births > 0 {
            fishies.push(FishCollection {
                days_until_birth: 8,
                count_of_fish: births,
            });
        }
    }

    Ok(fishies.iter().fold(0, |total, fish_grouping| {
        total + fish_grouping.count_of_fish
    }))
}
