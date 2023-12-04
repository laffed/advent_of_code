#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const RGB_MAX: (usize, usize, usize) = (12, 13, 14);
enum RGB {
    RED(usize),
    GREEN(usize),
    BLUE(usize),
}

pub fn solution_a<P>(inputpath: P) -> usize
where
    P: AsRef<Path>,
{
    let mut res = 0usize;
    let reader = BufReader::new(File::open(inputpath).unwrap());

    let rgbs = reader.lines().map(|line| {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(':').collect();
        let rgbs: Vec<Vec<&str>> = line[1]
            .split(';')
            .collect()
            .map(|set| set.split(','))
            .collect();
        
        let rgbs: Vec<Vec<RGB> = rgbs.filter_map(|l| {
                let bag_pull = l.trim();

                if let Some((point, color)) = bag_pull.split_once(char::is_whitespace) {
                    if let Ok(p) = point.parse::<usize>() {
                        match color.to_lowercase().as_str() {
                            "red" => Some(RGB::RED(p)),
                            "green" => Some(RGB::GREEN(p)),
                            "blue" => Some(RGB::BLUE(p)),
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        rgb
    });

    for (i, l) in rgbs.into_iter().enumerate() {
        let game_val = i + 1;
        let mut game_count = (0usize, 0usize, 0usize);

        for rgb in l {
            match rgb {
                RGB::RED(r) => game_count.0 += r,
                RGB::GREEN(g) => game_count.1 += g,
                RGB::BLUE(b) => game_count.2 += b,
            }
        }
        println!("{:?}", game_count);
        if game_count.0 > RGB_MAX.0 || game_count.1 > RGB_MAX.1 || game_count.2 > RGB_MAX.2 {
            continue;
        }

        res += game_val;
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
        assert_eq!(solution_a("inputs/twenty3/d2_ex_1.txt"), 8);
    }

    #[test]
    fn test_solution_a() {
        assert_eq!(solution_a("inputs/twenty3/d2.txt"), 30);
    }

    #[test]
    fn test_solution_b() {
        todo!();
    }
}
