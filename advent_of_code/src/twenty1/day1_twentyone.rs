#![allow(dead_code)]

pub fn solution() -> bool {
    true
}

fn read_file() -> Vec<usize> {
    vec![1, 2, 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = read_file();
        assert_eq!(input, vec![1, 2, 3]);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(), true);
    }
}
