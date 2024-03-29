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


struct Row {
    choice: SquareChoice,
    headers: Vec<Constraint>,
}


fn unpack(display: &str) -> (Vec<SquareChoice>,
                             HashMap<Constraint, Vec<SquareChoice>>) {
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

            state.push(SquareChoice { row: row as u8, col: col as u8, value });
            solved_constraints.insert(Constraint::Square { row: row as u8, col: col as u8 });
            solved_constraints.insert(Constraint::Row { row: row as u8, value });
            solved_constraints.insert(Constraint::Column { col: col as u8, value });
            solved_constraints.insert(Constraint::Block { block: b as u8, value });
        }
    }

    for row in 0..9u8 {
        for col in 0..9u8 {
            for value in 1..10u8 {
                let choice = SquareChoice { row, col, value };

                let cons_r = Constraint::Row { row, value };
                if solved_constraints.contains(&cons_r) { continue; }

                let cons_c = Constraint::Column { col, value };
                if solved_constraints.contains(&cons_c) { continue; }

                let b = 3 * (row / 3) + (col / 3);
                let cons_b = Constraint::Block { block: b, value };
                if solved_constraints.contains(&cons_b) { continue; }

                let cons_s = Constraint::Square { row, col };
                if solved_constraints.contains(&cons_s) { continue; }

                constraints.entry(cons_r)
                           .or_insert_with(Vec::new)
                           .push(choice);

                constraints.entry(cons_c)
                           .or_insert_with(Vec::new)
                           .push(choice);

                constraints.entry(cons_b)
                           .or_insert_with(Vec::new)
                           .push(choice);

                constraints.entry(cons_s)
                           .or_insert_with(Vec::new)
                           .push(choice);
            }
        }
    }

    (state, constraints)
}


fn pack(state: &[SquareChoice]) -> Vec<String> {
    let mut output = [['.'; 9]; 9];

    for choice in state {
        let row = choice.row as usize;
        let col = choice.col as usize;
        let value = u32::from(choice.value);

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
                                   Vec<SquareChoice>>)
         -> Vec<Row> {
    // column is a HashSet<SquareChoice>
    let column = constraints.remove(&header).unwrap();

    let mut removals = Vec::new();

    // destructively iterate over column
    for choice in column {
        let mut row = Row { choice, headers: Vec::new() };

        // using the implicit .into_iter would move our constraints ref
        for (other_header, other_col) in constraints.iter_mut() {
            let length = other_col.len();

            other_col.retain(|item| item != &row.choice);

            if other_col.len() != length {
                row.headers.push(other_header.clone());
            }
        }
        removals.push(row);
    }

    removals
}


fn uncover(header: Constraint,
           removals: &[Row],
           constraints: &mut HashMap<Constraint,
                                     Vec<SquareChoice>>) {
    for row in removals {
        constraints.entry(header)
                   .or_insert_with(Vec::new)
                   .push(row.choice);
        for &other_header in row.headers.iter() {
            constraints.entry(other_header)
                       .or_insert_with(Vec::new)
                       .push(row.choice);
        }
    }
}


fn most_constrained(constraints: &HashMap<Constraint,
                                          Vec<SquareChoice>>)
                    -> Constraint {
    let (&header, _) = constraints.iter()
                                  .min_by_key(|x| x.1.len())
                                  .unwrap();
    header
}


fn solve(state: &mut Vec<SquareChoice>,
         constraints: &mut HashMap<Constraint,
                                   Vec<SquareChoice>>)
         -> bool {

    if constraints.is_empty() { return true; }

    let header = most_constrained(constraints);

    let removals = cover(header, constraints);

    for row in &removals {
        state.push(row.choice);

        let mut row_removals = Vec::new();

        for &h in &row.headers {
            if constraints.contains_key(&h) {
                row_removals.push((h, cover(h, constraints)));
            }
        }

        if solve(state, constraints) { return true; }

        for &(h, ref r) in &row_removals {
            uncover(h, r, constraints);
        }

        state.pop();
    }

    uncover(header, &removals, constraints);

    false
}


fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)
               .expect("Failed to read input.");

    let (mut state, mut constraints) = unpack(&input);

    solve(&mut state, &mut constraints);

    let output = pack(&state);

    for line in output {
        println!("{}", line);
    }
}
