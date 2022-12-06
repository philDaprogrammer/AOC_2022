use std::fs;

fn part_one(sums: Vec<i64>) -> i64 {
    *sums.iter().max().unwrap()
}

fn part_two(mut sums: Vec<i64>) -> i64 {
    sums.sort();
    sums.pop().unwrap() + sums.pop().unwrap() + sums.pop().unwrap()
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Error reading input");

    let mut sums: Vec<i64> = Vec::new();
    let mut sum: i64;

    /* sum up the entries for both parts */
    for ent in contents.split("\n\n") {
        sum = 0;

        for x in ent.lines() {
            sum += x.parse::<i64>().unwrap();
        }

        sums.push(sum)
    }

    println!("{}", part_one(sums.clone()));
    println!("{}", part_two(sums.clone()));
}