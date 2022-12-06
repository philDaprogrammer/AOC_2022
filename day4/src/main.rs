use std::fs;

fn does_overlap_entirely(p1_start: i32, p1_end: i32, p2_start: i32, p2_end: i32) -> bool {
    ((p1_start >= p2_start) && (p1_end <= p2_end)) || ((p2_start >= p1_start) && (p2_end <= p1_end))
}

fn does_not_overlap(p1_start: i32, p1_end: i32, p2_start: i32, p2_end: i32) -> bool {
    (p1_end < p2_start) || (p2_end < p1_start)
}

fn solve(contents: String) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for line in contents.lines() {
        let pairs = line.split(",").collect::<Vec<&str>>();

        let pair1= pairs[0].split("-").collect::<Vec<&str>>();
        let pair2= pairs[1].split("-").collect::<Vec<&str>>();

        let p1_start = pair1[0].parse::<i32>().unwrap();
        let p1_end = pair1[1].parse::<i32>().unwrap();
        let p2_start = pair2[0].parse::<i32>().unwrap();
        let p2_end = pair2[1].parse::<i32>().unwrap();

        if does_overlap_entirely(p1_start, p1_end, p2_start, p2_end) {
            part1 += 1;
        }

        if !does_not_overlap(p1_start, p1_end, p2_start, p2_end) {
            part2 += 1;
        }
    }

    println!("part1={}, part2={}", part1 ,part2);
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Error reading input file");

    solve(contents);
}
