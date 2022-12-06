use std::fs;
use std::str;

#[derive(Clone)]
struct Arrangement {
    num: usize,
    src: usize,
    dest: usize
}

/* actually need to parse input this time into data structures */
fn parse(file_name: &str) -> (Vec<Vec<String>>, Vec<Arrangement>) {
    let contents: String = fs::read_to_string(file_name)
        .expect("Error reading input");

    /* split up the different sections to parse */
    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let num_stacks = (parts[0].lines().collect::<Vec<&str>>()[0].len() / 4) + 1;

    /* Data structures to return */
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let mut arrangements: Vec<Arrangement> = Vec::new();

    /* initialize all the needed stacks */
    for _ in 0..num_stacks { stacks.push(Vec::new()); }

    /* push all elements in to corresponding stacks */
    for line in parts[0].lines() {
        let elements = line.as_bytes().chunks(4)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        for i in 0..elements.len() {
            if !elements[i].trim().is_empty() {
                stacks[i].insert(0, String::from(elements[i].trim()))
            }
        }
    }

    /* get all of the arrangements as structs */
    for line in parts[1].lines() {
        let splits = line.split_whitespace().collect::<Vec<&str>>();

        arrangements.push(Arrangement{
            num:  splits[1].parse::<usize>().unwrap(),
            src:  splits[3].parse::<usize>().unwrap(),
            dest: splits[5].parse::<usize>().unwrap()
        })
    }

    (stacks, arrangements)
}

/* dump the output to STDOUT */
fn dump(stacks: Vec<Vec<String>>) {
    for mut stack in stacks {
        print!("{}", stack.pop().unwrap().chars().collect::<Vec<char>>()[1])
    }

    println!()
}

fn part1(mut stacks: Vec<Vec<String>>, arrangements: Vec<Arrangement>) {
    for arr in arrangements {
        for _ in 0..arr.num {
            let out = stacks[arr.src - 1].pop().unwrap();
            stacks[arr.dest - 1].push(out);
        }
    }

    dump(stacks.clone());
}

fn part2(mut stacks: Vec<Vec<String>>, arrangements: Vec<Arrangement>) {
    for arr in arrangements {
        let mut crane: Vec<String> = Vec::new();

        for _ in 0..arr.num {
            crane.push(stacks[arr.src - 1].pop().unwrap());
        }

        for _ in 0..arr.num {
            stacks[arr.dest - 1].push(crane.pop().unwrap())
        }
    }

    dump(stacks.clone());
}

fn main() {
    let ( stacks, arrangements) = parse("input.txt");
    part1(stacks.clone(), arrangements.clone());
    part2(stacks.clone(), arrangements.clone());
}