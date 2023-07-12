#[allow(dead_code)]

mod day1 {
    pub use std::fs::File;
    pub use std::io::{BufRead, BufReader};

    #[test]
    fn solution() {
        let input = read_file();

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
        let top_three = f + s + t;
        dbg!(top_three);
        assert_eq!(top_three, 206289);
    }

    fn read_file() -> Vec<usize> {
        include_str!("../inputs/day_one.txt")
            .split("\n\n")
            .map(|l| l.lines().map(|n| n.parse::<usize>().unwrap()).sum())
            .collect::<Vec<usize>>()
    }
}
