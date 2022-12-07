use std::collections::HashSet;

pub fn input_to_vec(input: &str) -> Vec<char> {
	input.chars().filter(|chr| *chr != '\n').collect()
}

pub fn find_packet_start(buffer: Vec<char>, packet_len: usize) -> Option<u32> {
	for index in (packet_len - 1)..(buffer.len() - 1) {
		let mut cur: HashSet<char> = HashSet::new();
		for i in (index - (packet_len - 1))..=index {
			cur.insert(buffer[i]);
		}
		if cur.len() == packet_len {
			return Some(index as u32 + 1);
		}
	}
	None
}

pub fn part_one(input: &str) -> Option<u32> {
	let buffer = input_to_vec(input);
	find_packet_start(buffer, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
	let buffer = input_to_vec(input);
	find_packet_start(buffer, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
