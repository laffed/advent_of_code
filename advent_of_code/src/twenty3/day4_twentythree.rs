#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solution_a<P>(filepath: P) -> usize
where
    P: AsRef<Path>,
{
    let mut res = 0usize;
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| {
        let line = line.unwrap();
        // let line: Vec<&str> = line.split(':').collect();
        let (left, right) = line.split_once(':').unwrap().1.split_once("|").unwrap();
        let left: Vec<usize> = left
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let right: Vec<usize> = right
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        (left, right)
    });

    for (left, right) in lines {
        let mut local_res = 0usize;
        for l in left {
            for r in &right {
                if l == *r {
                    local_res = if local_res == 0 { 1 } else { local_res * 2 };
                }
            }
        }
        res += local_res;
    }

    res
}

pub fn solution_b() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_a_ex() {
        assert_eq!(solution_a("inputs/twenty3/d4_ex_1.txt"), 13);
    }

    #[test]
    fn test_solution_a() {
        assert_eq!(solution_a("inputs/twenty3/d4.txt"), 23_750);
    }

    #[test]
    fn test_solution_b() {
        // todo!();
    }
}
