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
    None
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
        assert_eq!(part_two(&input), None);
    }
}
