#[allow(dead_code)]

mod day2 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
