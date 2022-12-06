use regex::Regex;
use std::collections::VecDeque;
use itertools::Itertools;

fn stacks_to_vectors(input: &str) -> Vec<VecDeque<char>> {
	let (empty_row_index, _) = input.lines()
		.enumerate()
		.find(|(_i, line)| *line == "")
		.unwrap();
	let stack_rows = input.lines().take(empty_row_index - 1);

	let mut stacks: Vec<VecDeque<char>> = Vec::new();

	stack_rows.for_each(|line| {
		line.chars()
			.skip(1)
			.enumerate()
			.filter_map(|(i, chr)| if i % 4 == 0 { Some(chr)} else { None })
			.enumerate()
			.for_each(|(i, chr)| {
				if i >= stacks.len() {
					stacks.push(VecDeque::new());
				}
				if chr != ' ' {
					stacks[i].push_back(chr)
				}
			})
	});	
	stacks
}

fn get_moving_instructions(input: &str) -> Vec<Vec<u32>> {
	let re = Regex::new(r"^move.(\d+).from.(\d+).to.(\d+)$").unwrap();
	input.lines()
		.filter_map(|line| {
			if re.is_match(line) {
				let cap = re.captures(line).unwrap();
				Some(vec![
					cap.get(1).unwrap().as_str().parse::<u32>().unwrap(), 
					cap.get(2).unwrap().as_str().parse::<u32>().unwrap(), 
					cap.get(3).unwrap().as_str().parse::<u32>().unwrap()
				])
			} else {
				None
			}
		})
		.collect()
}
pub fn part_one(input: &str) -> Option<String> {
	let mut stacks = stacks_to_vectors(input);
	get_moving_instructions(input).iter()
		.for_each(|nums| {
			let (amount, from, to) = nums.into_iter().tuples().next().unwrap();
			for _ in 0..*amount {
				let package = stacks[(from - 1)as usize].pop_front().unwrap();
				stacks[(to - 1) as usize].push_front(package);
			}
		});
	
	Some(stacks.iter().map(|crates| crates[0]).collect())
}

pub fn part_two(input: &str) -> Option<String> {
	let mut stacks = stacks_to_vectors(input);
	get_moving_instructions(input).iter()
		.for_each(|nums| {
			let (amount, from, to) = nums.into_iter().tuples().next().unwrap();
			let mut packages = VecDeque::new();
			for _ in 0..*amount {
				packages.push_front(stacks[(from - 1)as usize].pop_front().unwrap());
			};
			packages.iter().for_each(|chr| 
				stacks[(to - 1) as usize].push_front(*chr)
			);
		});
	Some(stacks.iter().map(|crates| crates[0]).collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
