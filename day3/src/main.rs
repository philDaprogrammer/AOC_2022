use std::fs;

fn part1(contents: String) -> i32 {
    let mut sum = 0;

    for line in contents.lines() {
        let lower = &line[0..line.len()/2];
        let upper = &line[line.len()/2..line.len()];

        for c in lower.chars() {
            if upper.contains(c) {
                sum += if c.is_ascii_uppercase() {c as i32 - 38 } else {c as i32 - 96};
                break;
            }
        }
    }

    sum
}

fn part2(contents: String) -> i32 {
    let mut lines: Vec<&str> = Vec::new();
    let mut sum: i32 = 0;

    for line in contents.lines() {
        if lines.len() < 2 {
            lines.push(line);
        } else {
            lines.push(line);

            for c in lines[0].chars() {
                if lines[1].contains(c) && lines[2].contains(c) {
                    sum += if c.is_ascii_uppercase() {c as i32 - 38 } else {c as i32 - 96};
                    break;
                }
            }

            lines = Vec::new();
        }
    }

    sum
}

fn main() {
    let sacks = fs::read_to_string("input.txt")
        .expect("Error reading input");

    println!("{}", part1(sacks.clone()));
    println!("{}", part2(sacks.clone()));
}