pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_pair)
            .filter(is_full_overlap)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_pair)
            .filter(is_some_overlap)
            .count() as u32,
    )
}

fn parse_pair(line: &str) -> (u32, u32, u32, u32) {
    let mut sections = line.split(&[',', '-']);
    (
        sections
            .next()
            .expect("Not a valid pair")
            .parse::<u32>()
            .expect("Not a valid section"),
        sections
            .next()
            .expect("Not a valid pair")
            .parse::<u32>()
            .expect("Not a valid section"),
        sections
            .next()
            .expect("Not a valid pair")
            .parse::<u32>()
            .expect("Not a valid section"),
        sections
            .next()
            .expect("Not a valid pair")
            .parse::<u32>()
            .expect("Not a valid section"),
    )
}

fn is_full_overlap(
    (sections1_start, sections1_end, sections2_start, sections2_end): &(u32, u32, u32, u32),
) -> bool {
    (sections1_start <= sections2_start && sections1_end >= sections2_end)
        || (sections2_start <= sections1_start && sections2_end >= sections1_end)
}

fn is_some_overlap(
    (sections1_start, sections1_end, sections2_start, sections2_end): &(u32, u32, u32, u32),
) -> bool {
    sections1_start <= sections2_end && sections1_end >= sections2_start
}
fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn test_is_full_overlap() {
        assert!(!is_full_overlap(&(1, 2, 2, 3)));
        assert!(is_full_overlap(&(1, 2, 2, 2)));
        assert!(is_full_overlap(&(1, 2, 1, 2)));
        assert!(is_full_overlap(&(2, 3, 1, 4)));
    }

    #[test]
    fn test_is_some_overlap() {
        assert!(!is_some_overlap(&(1, 2, 3, 4)));
        assert!(is_some_overlap(&(1, 2, 2, 3)));
        assert!(is_some_overlap(&(2, 3, 1, 2)));
        assert!(is_some_overlap(&(2, 5, 1, 4)));
    }
}
