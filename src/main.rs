use std::char;
use std::collections::{HashMap,HashSet};


fn unpack(display: &str) -> (Vec<(usize, usize, u8)>,
                             HashMap<(String, u8, u8), HashSet<(u8, u8, u8)>>) {
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
            solved_constraints.insert((String::from("square"), row as u8, col as u8));
            solved_constraints.insert((String::from("row"), row as u8, value));
            solved_constraints.insert((String::from("column"), col as u8, value));
            solved_constraints.insert((String::from("box"), b as u8, value));
        }
    }

    for row in 0..9u8 {
        for col in 0..9u8 {
            for value in 1..10u8 {
                let choice = (row, col, value);

                let cons_r = (String::from("row"), row, value);
                if solved_constraints.contains(&cons_r) { continue; }

                let cons_c = (String::from("column"), col, value);
                if solved_constraints.contains(&cons_c) { continue; }

                let cons_b = (String::from("box"), 3 * (row / 3) + (col / 3), value);
                if solved_constraints.contains(&cons_b) { continue; }

                let cons_s = (String::from("square"), row, col);
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


fn cover(header: &(String, u8, u8),
         constraints: &mut HashMap<(String, u8, u8),
                                   HashSet<(u8, u8, u8)>>)
         -> HashMap<(u8, u8, u8), Vec<(String, u8, u8)>> {
    let column = constraints.remove(&header).unwrap();

    let mut removals = HashMap::new();
    for row in column {
        for (other_header, other_col) in constraints.iter_mut() {
            if other_col.remove(&row) {
                removals.entry(row)
                        .or_insert(Vec::new())
                        .push(other_header.clone());
            }
        }
    }

    removals
}


fn uncover(header: &(String, u8, u8),
           removals: &HashMap<(u8, u8, u8), Vec<(String, u8, u8)>>,
           constraints: &mut HashMap<(String, u8, u8),
                                     HashSet<(u8, u8, u8)>>) {
    for (choice, headers) in removals.iter() {
        constraints.entry(header.clone())
                   .or_insert(HashSet::new())
                   .insert(choice.clone());
        for other_header in headers {
            constraints.entry(other_header.clone())
                       .or_insert(HashSet::new())
                       .insert(choice.clone());
        }
    }
}


fn solve(state: &mut Vec<(usize, usize, u8)>,
         constraints: &mut HashMap<(String, u8, u8),
                                   HashSet<(u8, u8, u8)>>)
         -> bool {

    if constraints.is_empty() { return true; }

    let (_count, header) = constraints.iter()
                                      .fold((100, ("".to_string(), 0, 0)),
                                            |(c, h_acc), (h, rows)| {
                                                let count = rows.len();
                                                if c > count { (count, h.clone()) }
                                                else { (c, h_acc) }
                                            });

    let removals = cover(&header, constraints);

    uncover(&header, &removals, constraints);

    false
}


fn main() {
    let input = "...84...9\n..1.....5\n8...2146.\n7.8....9.\n.........\n.5....3.1\n.2491...7\n9.....5..\n3...84...\n";
    let (mut state, mut constraints) = unpack(&input);

    solve(&mut state, &mut constraints);

    let output = pack(&state);

    for line in output {
        println!("{}", line);
    }
}
