#![allow(dead_code)]

pub fn solution(input: Vec<usize>) -> usize {
    let mut highest_sums = (0, 0, 0);

    for num in input {
        if num > highest_sums.0 {
            highest_sums.2 = highest_sums.1;
            highest_sums.1 = highest_sums.0;
            highest_sums.0 = num;
        } else if num > highest_sums.1 {
            highest_sums.2 = highest_sums.1;
            highest_sums.1 = num;
        } else if num > highest_sums.2 {
            highest_sums.2 = num;
        }
    }

    let (f, s, t) = highest_sums;

    f + s + t
}

fn read_file() -> Vec<usize> {
    include_str!("../../inputs/twenty2/day_one.txt")
        .split("\n\n")
        .map(|l| l.lines().map(|n| n.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = read_file();
        assert_eq!(input.first(), Some(&17034));
        assert_eq!(input.get(1), Some(&34768));
        assert_eq!(input.last(), Some(&53630));
    }

    #[test]
    fn test_solution() {
        let input = read_file();
        assert_eq!(solution(input), 206289);
    }
}
