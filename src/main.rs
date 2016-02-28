use std::io;
use std::io::Read;
use std::char;
use std::collections::{HashMap,HashSet};


#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Constraint {
    Square { row: u8, col: u8 },
    Row { row: u8, value: u8 },
    Column { col: u8, value: u8 },
    Block { block: u8, value: u8 },
}


#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct SquareChoice {
    row: u8,
    col: u8,
    value: u8,
}


fn unpack(display: &str) -> (Vec<SquareChoice>,
                             HashMap<Constraint, HashSet<SquareChoice>>) {
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

            state.push(SquareChoice { row: row as u8, col: col as u8, value: value });
            solved_constraints.insert(Constraint::Square { row: row as u8, col: col as u8 });
            solved_constraints.insert(Constraint::Row { row: row as u8, value: value });
            solved_constraints.insert(Constraint::Column { col: col as u8, value: value });
            solved_constraints.insert(Constraint::Block { block: b as u8, value: value });
        }
    }

    for row in 0..9u8 {
        for col in 0..9u8 {
            for value in 1..10u8 {
                let choice = SquareChoice { row: row, col: col, value: value };

                let cons_r = Constraint::Row { row: row, value: value };
                if solved_constraints.contains(&cons_r) { continue; }

                let cons_c = Constraint::Column { col: col, value: value };
                if solved_constraints.contains(&cons_c) { continue; }

                let b = 3 * (row / 3) + (col / 3);
                let cons_b = Constraint::Block { block: b, value: value };
                if solved_constraints.contains(&cons_b) { continue; }

                let cons_s = Constraint::Square { row: row, col: col };
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


fn pack(state: &Vec<SquareChoice>) -> Vec<String> {
    let mut output = [['.'; 9]; 9];

    for choice in state {
        let row = choice.row as usize;
        let col = choice.col as usize;
        let value = choice.value as u32;

        output[row][col] = match char::from_digit(value, 10) {
            Some(c) => c,
            None    => continue,
        };
    }

    let output: Vec<String> = output.iter()
                                    .map(|x| x.iter().map(|c| c.to_string()).collect())
                                    .collect();

    output
}


fn cover(header: Constraint,
         constraints: &mut HashMap<Constraint,
                                   HashSet<SquareChoice>>)
         -> Vec<(SquareChoice, Vec<Constraint>)> {
    let column = constraints.remove(&header).unwrap();

    let mut removals = Vec::new();
    for row in column {
        let mut row_removal = Vec::new();
        for (other_header, other_col) in constraints.iter_mut() {
            if other_col.remove(&row) {
                row_removal.push(other_header.clone());
            }
        }
        removals.push((row, row_removal));
    }

    removals
}


fn uncover(header: Constraint,
           removals: Vec<(SquareChoice, Vec<Constraint>)>,
           constraints: &mut HashMap<Constraint,
                                     HashSet<SquareChoice>>) {
    for (choice, headers) in removals {
        constraints.entry(header)
                   .or_insert(HashSet::new())
                   .insert(choice);
        for other_header in headers {
            constraints.entry(other_header)
                       .or_insert(HashSet::new())
                       .insert(choice);
        }
    }
}


fn most_constrained(constraints: &HashMap<Constraint,
                                          HashSet<SquareChoice>>)
                    -> Constraint {
    let (&header, _) = constraints.iter()
                                  .min_by_key(|x| x.1.len())
                                  .unwrap();
    header
}


fn solve(state: &mut Vec<SquareChoice>,
         constraints: &mut HashMap<Constraint,
                                   HashSet<SquareChoice>>)
         -> bool {

    if constraints.is_empty() { return true; }

    let header = most_constrained(constraints);

    let removals = cover(header, constraints);

    for &(row, ref other_headers) in &removals {
        state.push(row);

        let mut row_removals = Vec::new();

        for h in other_headers {
            if constraints.contains_key(h) {
                row_removals.push((*h, cover(*h, constraints)));
            }
        }

        if solve(state, constraints) { return true; }

        for &(h, ref r) in &row_removals {
            uncover(h, r.clone(), constraints);
        }

        state.pop();
    }

    uncover(header, removals, constraints);

    false
}


fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)
               .ok()
               .expect("Failed to read input.");

    let (mut state, mut constraints) = unpack(&input);

    solve(&mut state, &mut constraints);

    let output = pack(&state);

    for line in output {
        println!("{}", line);
    }
}
