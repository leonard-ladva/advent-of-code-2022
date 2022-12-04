use std::collections::HashMap;

fn match_winner(a: u32, b: u32) -> bool {
    if a == 1 {
        if b == 3 {
            return true;
        } else {
            return false;
        }
    } else if a == 2 {
        if b == 1 {
            return true;
        } else {
            return false;
        }
    } else {
        return b == 2;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sign_values = HashMap::from([
        ('A', 1), // Rock
        ('B', 2), // Paper
        ('C', 3), // Scissors
        ('X', 1),
        ('Y', 2),
        ('Z', 3),
    ]);

    let mut total_score: u32 = 0;

    for line in input.lines() {
        total_score += 3;

        let opponent = sign_values.get(&line.chars().nth(0).unwrap()).unwrap();
        let mine = sign_values.get(&line.chars().nth(2).unwrap()).unwrap();
        total_score += *mine;

        if mine == opponent {
            continue;
        }

        if match_winner(*mine, *opponent) {
            total_score += 3;
        } else {
            total_score -= 3;
        }
    }
    Some(total_score)
}

fn match_winner_sign(a: u32, b: u32) -> u32 {
    if b == 0 {
        // loss
        if a - 1 == 0 {
            return 3;
        }
        return a - 1;
    } else if b == 3 {
        // draw
        return a;
    } else {
        if a + 1 == 4 {
            return 1;
        }
        return a + 1;
    } //win
}

pub fn part_two(input: &str) -> Option<u32> {
    let sign_values = HashMap::from([
        ('A', 1), // Rock
        ('B', 2), // Paper
        ('C', 3), // Scissors
    ]);

    let results = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);
    let mut total_score: u32 = 0;

    for line in input.lines() {
        let result = results.get(&line.chars().nth(2).unwrap()).unwrap();
        total_score += *result;

        let opponent = sign_values.get(&line.chars().nth(0).unwrap()).unwrap();

        total_score += match_winner_sign(*opponent, *result)
    }
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
