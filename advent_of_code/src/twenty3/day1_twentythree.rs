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
fn filter_digits(line: &str) -> Vec<u32> {
    line.chars()
        .enumerate()
        .filter_map(|(i, c)| {
            c.to_digit(10).or_else(|| match c {
                'o' => map(&line[i..], "one", 1),
                't' => map(&line[i..], "two", 2).or_else(|| map(&line[i..], "three", 3)),
                'f' => map(&line[i..], "four", 4).or_else(|| map(&line[i..], "five", 5)),
                's' => map(&line[i..], "six", 6).or_else(|| map(&line[i..], "seven", 7)),
                'e' => map(&line[i..], "eight", 8),
                'n' => map(&line[i..], "nine", 9),
                _ => None,
            })
        })
        .collect()
}
fn map(l: &str, s: &str, n: u32) -> Option<u32> {
    if l.len() < s.len() {
        return None;
    }
    if s == &l[..s.len()] {
        return Some(n);
    }
    None
}
fn aoc_1_2(data: &str) -> usize {
    let mut sum = 0;
    for line in data.split_whitespace() {
        let v = filter_digits(line);
        // for correctness, unnecessary with the supplied data
        if v.is_empty() {
            continue;
        }
        sum += v[0] * 10 + v.last().unwrap();
    }
    sum as usize
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
        // assert_eq!(solution_p2("inputs/twenty3/d1.txt"), 0);
        // let data = std::fs::read_to_string("inputs/twenty3/d1.txt").unwrap();
        // assert_eq!(aoc_1_2(&data), 0);
    }
}
