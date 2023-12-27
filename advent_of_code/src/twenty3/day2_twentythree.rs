#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const RGB_MAX: (usize, usize, usize) = (12, 13, 14);

pub fn solution_a<P>(inputpath: P) -> usize
where
    P: AsRef<Path>,
{
    let mut res = 0usize;
    let reader = BufReader::new(File::open(inputpath).unwrap());

    let rgbs = reader.lines().map(|line| {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(':').collect();
        let rgbs: Vec<&str> = line[1].split(';').collect();

        let rgbs: Vec<Vec<&str>> = rgbs.iter().map(|set| set.split(',').collect()).collect();

        let rgbs: Vec<bool> = rgbs
            .iter()
            .flat_map(|l| {
                let set: Vec<bool> = l
                    .iter()
                    .map(|bag_pull| {
                        let mut set_points = (0usize, 0usize, 0usize);
                        if let Some((point, color)) =
                            bag_pull.trim().split_once(char::is_whitespace)
                        {
                            // println!("point:{}, color:{}", point, color);
                            if let Ok(p) = point.parse::<usize>() {
                                match color.to_lowercase().as_str() {
                                    "red" => {
                                        set_points.0 += p;
                                        set_points.0 <= RGB_MAX.0
                                    }
                                    "green" => {
                                        set_points.1 += p;
                                        set_points.0 <= RGB_MAX.0
                                    }
                                    "blue" => {
                                        set_points.2 += p;
                                        set_points.0 <= RGB_MAX.0
                                    }
                                    _ => false,
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .collect();

                set
            })
            .collect();

        rgbs
    });

    'loop1: for (i, l) in rgbs.into_iter().enumerate() {
        let game_val = i + 1;

        for rgb in l {
            if !rgb {
                println!("breaking index: {}", i);
                continue 'loop1;
            }
        }
        res += game_val;
        println!("gv:{}, res:{}", game_val, res);
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
        // assert_eq!(solution_a("inputs/twenty3/d2_ex_1.txt"), 8);
    }

    #[test]
    fn test_solution_a() {
        // assert_eq!(solution_a("inputs/twenty3/d2.txt"), 30);
    }

    #[test]
    fn test_solution_b() {
        // todo!();
    }
}
