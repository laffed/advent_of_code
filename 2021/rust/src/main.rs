use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod day_two {
    use super::*;

    #[test]
    fn two_two() {}

    #[test]
    fn two_one() {
        let commands = read_input();
        let mut x = 0;
        let mut z = 0;

        for command in commands.iter() {
            match command {
                CommandVector::Forward(d) => x += d,
                CommandVector::Up(d) => z -= d,
                CommandVector::Down(d) => z += d,
                _ => {}
            }
        }

        let res = x * z;
        assert_eq!(res, 2036120);
    }

    #[derive(Debug, PartialEq)]
    enum CommandVector {
        Forward(isize),
        Up(isize),
        Down(isize),
        Unknown,
    }

    fn read_input() -> Vec<CommandVector> {
        let mut res = Vec::new();
        let file = File::open("input/day_two.txt").expect("File not found");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let vector: Vec<&str> = line.split(' ').collect();
            let dir = vector[0];
            let mag = vector[1]
                .parse::<isize>()
                .expect("could not parse command magnitude");

            let command = match dir {
                "forward" => CommandVector::Forward(mag),
                "up" => CommandVector::Up(mag),
                "down" => CommandVector::Down(mag),
                _ => CommandVector::Unknown,
            };

            res.push(command);
        }

        res
    }
}

#[cfg(test)]
mod day_one {
    use super::*;

    #[test]
    fn one_two() {
        let readings = read_input();
        let mut n_increases = 0;
        let mut window: [usize; 3] = [0; 3];
        let mut prev_window_sum: Option<usize> = None;

        for num in readings.iter() {
            let [f, s, t] = window;
            let window_sum = f + s + t;
            println!(
                "cn: {}, pws: {:?}, ws: {}",
                num, prev_window_sum, window_sum
            );
            if !(f > 0 && s > 0 && t > 0) {
                shift(num, &mut window);
                continue;
            }

            if let Some(pws) = prev_window_sum {
                if pws < window_sum {
                    n_increases += 1;
                }
            }

            prev_window_sum = Some(window_sum);
            shift(num, &mut window);
        }

        let [f, s, t] = window;
        let window_sum = f + s + t;
        if let Some(pws) = prev_window_sum {
            if pws < window_sum {
                n_increases += 1;
            }
        }
        assert_eq!(n_increases, 1600);
    }

    fn shift(next: &usize, window: &mut [usize; 3]) {
        window[0] = window[1];
        window[1] = window[2];
        window[2] = next.clone();
    }

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
