use itertools::Itertools;
use regex::Regex;
use std::ops::Index;

pub struct Stacks {
    data: Vec<Vec<char>>,
}

impl Stacks {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        Self { data }
    }

    pub fn add_stack(&mut self) {
        self.data.push(vec![]);
    }

    pub fn add_container(&mut self, stack_nr: usize, container: char) {
        self.data[stack_nr].push(container);
    }

    pub fn move_container(&mut self, from: usize, to: usize) {
        let container = self.data[from].pop().unwrap();
        self.data[to].push(container);
    }

    pub fn move_containers(&mut self, nr_to_move: u32, from: usize, to: usize) {
        let mut crane = Vec::new();
        for _ in 0..nr_to_move {
            crane.push(self.data[from].pop().unwrap());
        }
        for _ in 0..nr_to_move {
            self.data[to].push(crane.pop().unwrap());
        }
    }

    pub fn top_containers(&self) -> String {
        self.data.iter().map(|stack| stack.last().unwrap()).join("")
    }
}

impl Index<usize> for Stacks {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub struct Move {
    nr_to_move: u32,
    from: usize,
    to: usize,
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for mv in moves {
        for _ in 0..mv.nr_to_move {
            stacks.move_container(mv.from, mv.to);
        }
    }
    Some(stacks.top_containers())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for mv in moves {
        stacks.move_containers(mv.nr_to_move, mv.from, mv.to);
    }
    Some(stacks.top_containers())
}

fn parse_input(input: &str) -> (Stacks, Vec<Move>) {
    let mut rows: Vec<&str> = vec![];
    let mut moves: Vec<Move> = Vec::new();
    for line in input.lines() {
        if line.contains('[') {
            rows.push(line);
        } else if line.starts_with("move") {
            let re = Regex::new(r"move (\d{1,2}) from (\d{1,2}) to (\d{1,2})").unwrap();
            let m = re.captures(line);
            let capture = m.unwrap();
            moves.push(Move {
                nr_to_move: capture[1].parse::<u32>().expect("Wrong nr of moves"),
                from: capture[2].parse::<usize>().expect("Wrong move from") - 1,
                to: capture[3].parse::<usize>().expect("Wrong move to") - 1,
            });
        }
    }

    let mut stacks = Stacks::new(Vec::new());
    let nr_of_stacks = (rows.first().expect("Error in row").len() + 1) / 4;
    for _ in 0..nr_of_stacks {
        stacks.add_stack();
    }

    rows.reverse();
    for line in rows {
        let bytes = line.as_bytes();
        (0..nr_of_stacks).for_each(|i| {
            let container = bytes[i * 4 + 1] as char;
            if container != ' ' {
                stacks.add_container(i, container);
            }
        });
    }

    (stacks, moves)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
