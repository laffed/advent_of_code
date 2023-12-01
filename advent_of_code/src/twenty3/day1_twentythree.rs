#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solution_p1<P>(inputpath: P) -> usize
where
    P: AsRef<Path>,
{
    let reader = BufReader::new(File::open(inputpath).unwrap());

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<_>>()
        })
        .fold(0, |acc, l| {
            let first = l.first().and_then(|c| c.to_digit(10)).unwrap_or(0);
            let last = l.last().and_then(|c| c.to_digit(10)).unwrap_or(first);
            acc + (10 * first as usize + last as usize)
        })
}

const STR_DIGITS: [(&str, usize); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn helper(s: &str, rev: bool) -> usize {
    let mut matches: Vec<(usize, usize)> = vec![];
    for (word, d) in STR_DIGITS.into_iter() {
        if let Some(order) = s.find(word) {
            matches.push((order, d))
        }
    }

    matches.sort_by(|a, b| a.0.cmp(&b.0));
    let mut matches = matches.into_iter();

    return if !rev {
        matches.next().unwrap().1
    } else {
        matches.last().unwrap().1
    };
}

pub fn solution_p2<P>(inputpath: P) -> usize
where
    P: AsRef<Path>,
{
    let reader = BufReader::new(File::open(inputpath).unwrap());

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let first = helper(&line, false);
            let last = helper(&line, true);

            (10 * first) + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_solution_ex() {
        // assert_eq!(solution_p1("inputs/twenty3/d1_ex_1.txt"), 142);
    }

    #[test]
    fn p1_solution() {
        // assert_eq!(solution_p1("inputs/twenty3/d1.txt"), 55607);
    }

    #[test]
    fn p2_solution_ex() {
        assert_eq!(solution_p2("inputs/twenty3/d1_ex_2.txt"), 281);
    }

    #[test]
    fn p2_solution() {
        assert_eq!(solution_p2("inputs/twenty3/d1.txt"), 0);
    }

    #[test]
    fn test_test() {
        assert_eq!(helper("1abc2", false), 1);
        assert_eq!(helper("1abc2", true), 2);
        assert_eq!(helper("pqr3stu8vwx", false), 3);
        assert_eq!(helper("pqr3stu8vwx", true), 8);
        assert_eq!(helper("a1b2c3d4e5f", false), 1);
        assert_eq!(helper("a1b2c3d4e5f", true), 5);
        assert_eq!(helper("treb7uchet", false), 7);
        assert_eq!(helper("treb7uchet", true), 7);
        // assert_eq!(helper("pqr3stu8vwx"), Some(38));
        // assert_eq!(helper("a1b2c3d4e5f"), Some(15));
        // assert_eq!(helper("treb7uchet"), Some(77));
    }
}
