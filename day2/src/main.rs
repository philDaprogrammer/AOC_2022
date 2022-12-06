use std::fs;
use std::collections::HashMap;

fn part1(contents: String, score: HashMap<(char, char), i64>) -> i64 {
    let mut total: i64 = 0;

    for line in contents.lines() {
        let mut moves = line.chars();
        let tup   = (moves.nth(0).unwrap(), moves.nth(1).unwrap());
        total += score.get(&tup).unwrap();
    }

    total
}


fn part2(contents: String, score: HashMap<(char, char), i64>) -> i64 {
    let choices: HashMap<(char, char), char> = HashMap::from([
       (('A', 'Y'), 'X'),
       (('A', 'X'), 'Z'),
       (('A', 'Z'), 'Y'),
       (('B', 'Y'), 'Y'),
       (('B', 'X'), 'X'),
       (('B', 'Z'), 'Z'),
       (('C', 'Y'), 'Z'),
       (('C', 'X'), 'Y'),
       (('C', 'Z'), 'X')]);


    let mut allMoves: Vec<(char, char)> = Vec::new();
    let mut total: i64 = 0;

    for line in contents.lines() {
        let mut moves = line.chars();
        let tup = (moves.nth(0).unwrap(), moves.nth(1).unwrap());

        allMoves.push((tup.0, *choices.get(&tup).unwrap()));
    }

    for moves in allMoves {
        total += *score.get(&moves).unwrap();
    }

    total
}


fn main() {
    let contents: String = fs::read_to_string("input.txt")
        .expect("Error reading input file");

    let score  = HashMap::from([
        (('A', 'Y'), 8),
        (('A', 'X'), 4),
        (('A', 'Z'), 3),
        (('B', 'Y'), 5),
        (('B', 'X'), 1),
        (('B', 'Z'), 9),
        (('C', 'Y'), 2),
        (('C', 'X'), 7),
        (('C', 'Z'), 6)]);

    println!("{}", part1(contents.clone(), score.clone()));
    print!("{}", part2(contents.clone(), score.clone()));
}
