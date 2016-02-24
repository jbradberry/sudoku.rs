use std::char;
use std::collections::{HashMap,HashSet};


fn unpack(display: &str) -> (Vec<(usize, usize, u8)>,
                             HashMap<(&str, u8, u8), HashSet<(u8, u8, u8)>>) {
    let mut state = Vec::new();
    let mut solved_constraints = HashSet::new();
    let mut constraints = HashMap::new();

    for (row, line) in display.lines().enumerate() {
        assert!(row < 9);

        for (col, c) in line.chars().enumerate() {
            assert!(col < 9);

            let value = match c.to_digit(10) {
                Some(num) => num as u8,
                None => continue,
            };

            let b = 3 * (row / 3) + (col / 3);

            state.push((row, col, value));
            solved_constraints.insert(("square", row as u8, col as u8));
            solved_constraints.insert(("row", row as u8, value));
            solved_constraints.insert(("column", col as u8, value));
            solved_constraints.insert(("box", b as u8, value));
        }
    }

    for row in 0..9u8 {
        for col in 0..9u8 {
            for value in 1..10u8 {
                let choice = (row, col, value);

                let cons_r = ("row", row, value);
                if solved_constraints.contains(&cons_r) { continue; }

                let cons_c = ("column", col, value);
                if solved_constraints.contains(&cons_c) { continue; }

                let cons_b = ("box", 3 * (row / 3) + (col / 3), value);
                if solved_constraints.contains(&cons_b) { continue; }

                let cons_s = ("square", row, col);
                if solved_constraints.contains(&cons_s) { continue; }

                constraints.entry(cons_r)
                           .or_insert(HashSet::new())
                           .insert(choice);

                constraints.entry(cons_c)
                           .or_insert(HashSet::new())
                           .insert(choice);

                constraints.entry(cons_b)
                           .or_insert(HashSet::new())
                           .insert(choice);

                constraints.entry(cons_s)
                           .or_insert(HashSet::new())
                           .insert(choice);
            }
        }
    }

    (state, constraints)
}


fn pack(state: &Vec<(usize, usize, u8)>) -> Vec<String> {
    let mut output = [['.'; 9]; 9];

    for &(row, col, value) in state {
        output[row][col] = match char::from_digit(value as u32, 10) {
            Some(c) => c,
            None    => continue,
        };
    }

    let output: Vec<String> = output.iter()
                                    .map(|x| x.iter().map(|c| c.to_string()).collect())
                                    .collect();

    output
}


fn main() {
    let input = "...84...9\n..1.....5\n8...2146.\n7.8....9.\n.........\n.5....3.1\n.2491...7\n9.....5..\n3...84...\n";
    let (state, constraints) = unpack(&input);
    let output = pack(&state);

    for line in output {
        println!("{}", line);
    }
}
