use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {}

mod day_five {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(true, false);
    }

    fn read_input() {}
}

mod day_four {
    use super::*;

    #[test]
    fn solutionp2() {
        let mut res = 0;

        for [left, right] in read_input() {
            let left_overlapped = (left[0] >= right[0] && left[0] <= right[1])
                || (left[1] >= right[0] && left[1] <= right[1]);

            // right side calculation is short circuted if already left overlapped for efficiency
            let right_overlapped = left_overlapped
                || (right[0] >= left[0] && right[0] <= left[1])
                || (right[1] >= left[0] && right[1] <= left[1]);

            if left_overlapped || right_overlapped {
                res += 1;
            }
        }

        assert_eq!(res, 928);
    }

    #[test]
    fn solution() {
        let mut res = 0;

        for [left, right] in read_input() {
            let left_contains = left[0] >= right[0] && left[1] <= right[1];
            // right side calculation is short circuted if already left contained for efficiency
            let right_contains = left_contains || (right[0] >= left[0] && right[1] <= left[1]);

            if left_contains || right_contains {
                res += 1;
            }
        }

        assert_eq!(res, 599);
    }

    fn read_input() -> Vec<[[usize; 2]; 2]> {
        let mut vector = Vec::new();
        let file = File::open("inputs/day_four.txt").unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            let parsed_line = line
                .split(&[',', '-'][..])
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            vector.push([
                [parsed_line[0], parsed_line[1]],
                [parsed_line[2], parsed_line[3]],
            ]);
        }

        vector
    }
}

mod day_three {
    use super::*;

    #[test]
    fn solutionp2() {
        let mut res = 0;

        for chunk in read_input().chunks(3) {
            let first = chunk[0].clone();
            let sec = chunk[1].clone();
            let third = chunk[2].clone();

            let mut badge: Option<char> = None;
            'floop: for f in first.chars() {
                for s in sec.chars() {
                    if f != s {
                        continue;
                    }
                    for t in third.chars() {
                        if f == s && s == t {
                            badge = Some(f);
                            break 'floop;
                        }
                    }
                }
            }

            let to_add = match badge {
                Some(c) => find_priority(c),
                None => 0,
            };

            res += to_add;
        }

        assert_eq!(res, 2752);
    }

    #[test]
    fn solution() {
        let input = read_input();
        let mut res = 0;
        for line in input {
            let split_at = line.chars().count() / 2;

            let (left, right) = line.split_at(split_at);

            let mut duplicate_char: Option<char> = None;

            'outer: for l in left.chars() {
                for r in right.chars() {
                    if l == r {
                        duplicate_char = Some(l);
                        break 'outer;
                    }
                }
            }

            let to_add = match duplicate_char {
                Some(c) => find_priority(c),
                None => 0,
            };

            res += to_add;
        }

        assert_eq!(res, 7821);
    }

    fn find_priority(c: char) -> usize {
        const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let res = PRIORITIES.find(c).unwrap();

        res + 1
    }

    fn read_input() -> Vec<String> {
        let file = File::open("inputs/day_three.txt").unwrap();
        let reader = BufReader::new(file);
        let mut res = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            res.push(line);
        }

        res
    }
}

mod day_two {
    use super::*;

    struct Cypher {
        char: char,
        value: i32,
        add: char,
        loss: char,
        eq: char,
    }

    const CYPHERS: [Cypher; 3] = [
        Cypher {
            char: 'X',
            value: 1,
            add: 'C',
            loss: 'B',
            eq: 'A',
        },
        Cypher {
            char: 'Y',
            value: 2,
            add: 'A',
            loss: 'C',
            eq: 'B',
        },
        Cypher {
            char: 'Z',
            value: 3,
            add: 'B',
            loss: 'A',
            eq: 'C',
        },
    ];

    const POINTS: ((char, i32), (char, i32), (char, i32)) = (('A', 1), ('B', 2), ('C', 3));

    struct Cypher2 {
        char: char,
        lose: (char, i32),
        win: (char, i32),
        eq: (char, i32),
    }

    const CYPHERS2: [Cypher2; 3] = [
        Cypher2 {
            char: 'A',
            lose: POINTS.2,
            win: POINTS.1,
            eq: POINTS.0,
        },
        Cypher2 {
            char: 'B',
            lose: POINTS.0,
            win: POINTS.2,
            eq: POINTS.1,
        },
        Cypher2 {
            char: 'C',
            lose: POINTS.1,
            win: POINTS.0,
            eq: POINTS.2,
        },
    ];

    #[test]
    fn solution() {
        let input = read_input();
        println!("{:?}", input[0]);
        let mut final_score = 0;

        for row in input {
            let opp = row[0];
            let outcome = row[1];

            let cyp = match opp {
                'A' => &CYPHERS2[0],
                'B' => &CYPHERS2[1],
                'C' => &CYPHERS2[2],
                _ => panic!("Invalid opp char"),
            };

            let to_add = match outcome {
                'X' => &cyp.lose.1 + 0,
                'Y' => &cyp.eq.1 + 3,
                'Z' => &cyp.win.1 + 6,
                _ => panic!("Invalid outcome char"),
            };

            final_score += to_add;
        }

        assert_eq!(final_score, 12091);
    }

    fn read_input() -> Vec<Vec<char>> {
        let mut res: Vec<Vec<char>> = Vec::new();

        let file = File::open("inputs/day_two.txt").unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let mut row = Vec::new();
            for c in line.chars() {
                if c == ' ' {
                    continue;
                }
                row.push(c);
            }
            res.push(row);
        }

        res
    }
}

mod day_one {
    use super::*;

    #[test]
    fn solution() {
        let input = read_file("inputs/day_one.txt");

        let mut highest_sums = (0, 0, 0);

        for row in input {
            let mut current_sum: i32 = 0;
            for num in row {
                current_sum += num;
            }
            if current_sum > highest_sums.0 {
                highest_sums.2 = highest_sums.1;
                highest_sums.1 = highest_sums.0;
                highest_sums.0 = current_sum;
            } else if current_sum > highest_sums.1 {
                highest_sums.2 = highest_sums.1;
                highest_sums.1 = current_sum;
            } else if current_sum > highest_sums.2 {
                highest_sums.2 = current_sum;
            }
        }

        let (f, s, t) = highest_sums;
        let top_three = f + s + t;
        println!("{:?}", highest_sums);
        println!("{:?}", top_three);
    }

    fn read_file(filepath: &str) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        let mut row: Vec<i32> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                res.push(row);
                row = Vec::new();
            } else {
                let number = line.parse::<i32>().unwrap();
                row.push(number);
            }
        }

        if !row.is_empty() {
            res.push(row);
        }
        res
    }
}
