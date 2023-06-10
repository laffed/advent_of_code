use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod day_four {
    use super::*;

    #[test]
    fn four_one() {
        assert_eq!(true, false);
    }

    type Board = [[u32; 5]; 5];

    #[derive(Debug)]
    struct FourInput {
        instructions: Vec<u32>,
        boards: Vec<Board>,
    }

    fn read_input() -> FourInput {
        let file = File::open("input/day_four_test.txt").expect("could not read day four input");
        let reader = BufReader::new(file);

        let mut instructions = Vec::new();
        let mut boards: Vec<Board> = Vec::new();

        for (i, line) in reader.lines().enumerate() {}

        FourInput {
            instructions,
            boards,
        }
    }
}

#[cfg(test)]
mod day_three {
    use super::*;

    #[test]
    fn three_two() {
        /*
        O2:
        - determine the MOST common value in the current bit position
        - keep only numbers with that bit in that position
        - if 0 and 1 are equally common, keep values with a 1 in the position

        CO2:
        - determine the LEAST common value in the current bit position
        - keep only numbers with that bit in that position
        - if 0 and 1 are equally common, keep values with a 0 in the position
        */
    }

    #[derive(Clone, Debug)]
    struct BitPositionQuant {
        one: usize,
        zero: usize,
    }

    #[test]
    fn three_one() {
        let lists = read_input();
        let mut record: Vec<BitPositionQuant> =
            vec![BitPositionQuant { one: 0, zero: 0 }; lists[0].len()];

        for list in lists {
            for (i, c) in list.iter().enumerate() {
                match c {
                    '1' => record[i].one += 1,
                    '0' => record[i].zero += 1,
                    _ => {}
                }
            }
        }

        let gamma = record
            .iter()
            .map(|bp| match bp.one > bp.zero {
                true => '1',
                false => '0',
            })
            .collect::<String>();

        let epsilon = record
            .iter()
            .map(|bp| match bp.one > bp.zero {
                true => '0',
                false => '1',
            })
            .collect::<String>();

        let gamma_dec = usize::from_str_radix(&gamma, 2).expect("could not parse gamma str");
        let epsilon_dec = usize::from_str_radix(&epsilon, 2).expect("could not parse epsilon str");

        let res = gamma_dec * epsilon_dec;

        println!("Gamma: {} >> {}", gamma, gamma_dec);
        println!("Epsilon: {} >> {}", epsilon, epsilon_dec);
        assert_eq!(res, 3374136);
    }

    fn read_input() -> Vec<Vec<char>> {
        let mut res = Vec::new();
        let file = File::open("input/day_three.txt").expect("could not read day three input");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let row = line.chars().collect();
            res.push(row);
        }

        res
    }
}

#[cfg(test)]
mod day_two {
    use super::*;

    #[test]
    fn two_two() {
        /*
        down X increases your aim by X units.
        up X decreases your aim by X units.
        forward X does two things:
        It increases your horizontal position by X units.
        It increases your depth by your aim multiplied by X.
        */
        let commands = read_input();
        let mut rad = 0;
        let mut x = 0;
        let mut z = 0;

        for command in commands.iter() {
            match command {
                CommandVector::Up(d) => rad -= d,
                CommandVector::Down(d) => rad += d,
                CommandVector::Forward(d) => {
                    x += d;
                    z += d * rad;
                }
                _ => {}
            }
        }

        let res = x * z;
        assert_eq!(res, 2015547716);
    }

    #[test]
    fn two_one() {
        let commands = read_input();
        let mut x = 0;
        let mut z = 0;

        for command in commands.iter() {
            match command {
                CommandVector::Up(d) => z -= d,
                CommandVector::Down(d) => z += d,
                CommandVector::Forward(d) => x += d,
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
