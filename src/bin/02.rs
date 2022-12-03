pub fn part_one(input: &str) -> Option<u32> {
    let rounds = parse_rounds(input);
    Some(rounds.fold(0, |acc, round| acc + evaluate_round(round)))
}

pub fn part_two(input: &str) -> Option<u32> {
    let recommended_result = parse_rounds(input);
    Some(recommended_result.fold(0, |acc, recommendation| {
        acc + evaluate_round(get_round(recommendation))
    }))
}

pub fn get_round(recommendation: (char, char)) -> (char, char) {
    match recommendation {
        (opp_move, 'X') if opp_move == 'A' => ('A', 'Z'),
        (opp_move, 'Y') if opp_move == 'A' => ('A', 'X'),
        (opp_move, 'Z') if opp_move == 'A' => ('A', 'Y'),
        (opp_move, 'X') if opp_move == 'B' => ('B', 'X'),
        (opp_move, 'Y') if opp_move == 'B' => ('B', 'Y'),
        (opp_move, 'Z') if opp_move == 'B' => ('B', 'Z'),
        (opp_move, 'X') if opp_move == 'C' => ('C', 'Y'),
        (opp_move, 'Y') if opp_move == 'C' => ('C', 'Z'),
        (opp_move, 'Z') if opp_move == 'C' => ('C', 'X'),
        _ => panic!("Not a valid round!"),
    }
}

pub fn evaluate_round(round: (char, char)) -> u32 {
    let choice = match round {
        (_, 'X') => 1,
        (_, 'Y') => 2,
        (_, 'Z') => 3,
        _ => panic!("Not a valid choice!"),
    };
    let outcome = match round {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
        _ => panic!("Not a valid round!"),
    };
    choice + outcome
}

pub fn parse_round(round: &str) -> (char, char) {
    let mut round_str = round.split(' ');
    (
        round_str.next().unwrap().parse::<char>().unwrap(),
        round_str.next().unwrap().parse::<char>().unwrap(),
    )
}

pub fn parse_rounds(input: &str) -> impl Iterator<Item = (char, char)> + '_ {
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
        assert_eq!(part_two(&input), Some(12));
    }
}
