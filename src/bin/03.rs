use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0u32, |acc, rucksack| {
        acc + priority(find_duplicate(rucksack))
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .tuples()
            .fold(0u32, |acc, (first, second, third)| {
                acc + priority(find_badge(first, second, third))
            }),
    )
}

pub fn find_badge(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> char {
    rucksack1
        .chars()
        .find(|item| {
            rucksack2.chars().any(|item2| *item == item2)
                && rucksack3.chars().any(|item3| *item == item3)
        })
        .unwrap()
}

pub fn find_duplicate(rucksack: &str) -> char {
    let compartments = rucksack.split_at(rucksack.len() / 2);
    compartments
        .0
        .chars()
        .find(|item| compartments.1.chars().any(|item2| *item == item2))
        .unwrap()
}

pub fn priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        (item as u32) - ('a' as u32) + 1
    } else if item.is_ascii_uppercase() {
        (item as u32) - ('A' as u32) + 27
    } else {
        panic!("Invalid item")
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate("abcdefghid"), 'd')
    }

    #[test]
    fn test_find_badge() {
        assert_eq!(find_badge("abcde", "fghic", "klmcn"), 'c')
    }
}
