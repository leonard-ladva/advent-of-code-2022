use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(comp_a, comp_b)| {
            let set: HashSet<char> = comp_a.chars().collect();
            comp_b.chars().find(|c| set.contains(c))
        })
        .map(|c| c.unwrap() as u32)
        .map(|ascii| {
            if ascii >= 'a' as u32 {
                return ascii - 96;
            } else {
                return ascii - 38;
            }
        })
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    for chunk in &input.lines().chunks(3) {
        chunk
            .collect_tuple::<(_, _, _)>()
            .map(|(a, b, c)| {
                let a_set: HashSet<char> = a.chars().collect();
                let mut b_set: HashSet<char> = HashSet::new();
                b.chars().for_each(|ch| {
                    if a_set.contains(&ch) {
                        b_set.insert(ch);
                    }
                });
                return c.chars().find(|ch| b_set.contains(ch));
            })
            .map(|c| c.unwrap() as u32)
            .map(|ascii| {
                if ascii >= 'a' as u32 {
                    // return
                    total += ascii - 96;
                } else {
                    total += ascii - 38;
                }
            });
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
