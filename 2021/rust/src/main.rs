use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod day_one {
    use super::*;

    #[test]
    fn one_one() {
        let readings = read_input();

        let mut n_increases = 0;
        let mut prev_num: Option<&usize> = None;

        for num in readings.iter() {
            if let Some(p) = prev_num {
                if p < num {
                    n_increases += 1;
                }
            }
            prev_num = Some(num);
        }
        assert_eq!(n_increases, 1559);
    }

    fn read_input() -> Vec<usize> {
        let mut res = Vec::new();
        let file = File::open("input/day_one.txt").expect("File not found");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let num = line.parse::<usize>().unwrap();
            res.push(num);
        }

        res
    }
}
