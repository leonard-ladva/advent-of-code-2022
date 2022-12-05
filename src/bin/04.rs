use regex::Regex;

fn make_assignment_groups(line: &str, re: &Regex) -> Option<((u32, u32), (u32, u32))> {
	let assignment_groups: ((u32, u32), (u32, u32));
	for cap in re.captures_iter(line) {
		assignment_groups = (
			(cap[1].parse().unwrap(), cap[2].parse().unwrap()), 
			(cap[3].parse().unwrap(), cap[4].parse().unwrap())
		);
		return Some(assignment_groups);
	};
	None	
}

fn is_complete_overlap(groups: ((u32, u32), (u32, u32))) -> bool {
    let ((start_1, end_1), (start_2, end_2)) = groups;
	return
	start_1 >= start_2 && end_1 <= end_2
	|| 
	start_2 >= start_1 && end_2 <= end_1
}

fn is_partial_overlap(groups: ((u32, u32), (u32, u32))) -> bool {
    let ((start_1, end_1), (start_2, end_2)) = groups;
	return start_1 <= end_2 && end_1 >= start_2
}

pub fn part_one(input: &str) -> Option<u32> {
	let re: Regex = Regex::new( r"^(\d+)-(\d+),(\d+)-(\d+)$" ).unwrap();
	Some(
		input.lines()
		.map(|line| make_assignment_groups(line, &re).unwrap())
		.filter_map(|group| if is_complete_overlap(group) { Some(true) } else { None } )
		.count() as u32
	)
}

pub fn part_two(input: &str) -> Option<u32> {
	let re: Regex = Regex::new( r"^(\d+)-(\d+),(\d+)-(\d+)$" ).unwrap();
	Some(
		input.lines()
		.map(|line| make_assignment_groups(line, &re).unwrap())
		.filter_map(|group| if is_partial_overlap(group) { Some(true) } else { None } )
		.count() as u32
	)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
