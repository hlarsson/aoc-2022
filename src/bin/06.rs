use std::collections::HashSet;

pub struct SetWindows {
    data: Vec<char>,
}

impl SetWindows {
    pub fn new(data: Vec<char>) -> Self {
        Self { data }
    }

    pub fn iter(&self, window_size: usize) -> impl Iterator<Item = HashSet<char>> + '_ {
        (0..(self.data.len() - window_size)).map(move |i| {
            let mut set = HashSet::new();
            (i..(i + window_size)).for_each(|j| {
                set.insert(self.data[j]);
            });
            set
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_marker(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_marker(input, 14))
}

fn find_marker(message: &str, window_size: usize) -> u32 {
    let windows = SetWindows::new(message.chars().collect());
    let mut index = window_size;
    for window in windows.iter(window_size) {
        if window.len() == window_size {
            break;
        }
        index += 1;
    }
    index as u32
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        let part_one_input = input.lines().find(|_| true).unwrap();
        assert_eq!(part_one(part_one_input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        let part_two_input = input.lines().last().unwrap();
        assert_eq!(part_two(part_two_input), Some(29));
    }
}
