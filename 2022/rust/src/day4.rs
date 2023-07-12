#[allow(dead_code)]

mod day4 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
