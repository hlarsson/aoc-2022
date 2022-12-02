pub fn part_one(input: &str) -> Option<u32> {
    let rounds = parse_rounds(input);
    Some(rounds.fold(0, |acc, round| acc + evaluate_round(round)))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn evaluate_round(round: (&str, &str)) -> u32 {
    let choice = match round {
        (_, "X") => 1,
        (_, "Y") => 2,
        (_, "Z") => 3,
        _ => panic!("Not a valid choice!"),
    };
    let outcome = match round {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => panic!("Not a valid hand!"),
    };
    choice + outcome
}

pub fn parse_round(round: &str) -> (&str, &str) {
    let mut round_str = round.split(' ');
    (round_str.next().unwrap(), round_str.next().unwrap())
}

pub fn parse_rounds(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.split('\n').map(parse_round)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
