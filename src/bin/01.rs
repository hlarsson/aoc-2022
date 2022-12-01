pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|item| item.parse::<u32>().unwrap_or_default())
                .reduce(|acc: u32, cal: u32| cal + acc)
        })
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = input
        .split("\n\n")
        .fold((0u32, 0u32, 0u32), |highest_calories, elf| {
            let calories = elf
                .split('\n')
                .map(|item| item.parse::<u32>().unwrap_or_default())
                .reduce(|acc: u32, cal: u32| cal + acc)
                .unwrap();
            update_highest_calories(highest_calories, calories)
        });
    Some(elves.0 + elves.1 + elves.2)
}

pub fn update_highest_calories(
    highest_calories: (u32, u32, u32),
    new_calories: u32,
) -> (u32, u32, u32) {
    match highest_calories {
        (a, b, _) if new_calories > a => (new_calories, a, b),
        (a, b, _) if new_calories > b => (a, new_calories, b),
        (a, b, c) if new_calories > c => (a, b, new_calories),
        _ => highest_calories,
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
