#[cfg(test)]
pub mod test;

use super::group_by::group_by;
use eyre::Result as EyreResult;

#[derive(Debug)]
pub struct FishCollection {
    days_until_birth: usize,
    count_of_fish: usize,
}

pub fn parse_fish_days_until_birthing(input: &str) -> EyreResult<Vec<FishCollection>> {
    let days_until_birthing = input
        .split(',')
        .map(|fish| fish.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let grouped_by_days_until_birthing = group_by(days_until_birthing.into_iter(), |item| *item)
        .map(|(days_until_birth, grouping_of_fish)| FishCollection {
            days_until_birth,
            count_of_fish: grouping_of_fish.len(),
        })
        .collect::<Vec<_>>();

    Ok(grouped_by_days_until_birthing)
}

pub fn challenge(input: &str, days: usize) -> EyreResult<usize> {
    let mut fishies = parse_fish_days_until_birthing(input)?;

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

            // regroup array to keep above iterations low rather than having better
            // iteration logic that doesn't create new vec elements every birthing
            fishies = group_by(fishies.into_iter(), |item| item.days_until_birth)
                .map(|(days_until_birth, grouping_of_fish)| FishCollection {
                    days_until_birth,
                    count_of_fish: grouping_of_fish.len(),
                })
                .collect::<Vec<_>>();
        }
    }

    Ok(fishies.iter().fold(0, |total, fish_grouping| {
        total + fish_grouping.count_of_fish
    }))
}
