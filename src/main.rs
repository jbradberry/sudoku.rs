use std::char;

fn unpack(display: &str) -> Vec<(usize, usize, u8)> {
    let mut vec = Vec::new();

    for (row, line) in display.lines().enumerate() {
        assert!(row < 9);

        for (col, c) in line.chars().enumerate() {
            assert!(col < 9);

            let value = match c.to_digit(10) {
                Some(num) => num as u8,
                None => continue,
            };

            vec.push((row, col, value));
        }
    }
    vec
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
    let state = unpack(&input);
    let output = pack(&state);

    println!("{:?}", state);
    println!("{:?}", output);

    for line in output {
        println!("{}", line);
    }
}
