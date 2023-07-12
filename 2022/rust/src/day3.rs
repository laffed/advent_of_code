#[allow(dead_code)]

mod day3 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
