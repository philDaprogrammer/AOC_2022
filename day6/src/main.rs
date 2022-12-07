use std::fs;
use std::collections::LinkedList;
use std::collections::HashSet;

fn solve(signal: String, size: usize) -> i32 {
    let mut  bits = signal.chars().collect::<Vec<char>>();
    let mut  header: LinkedList<char> =  LinkedList::new();
    let mut items: HashSet<char>;

    for i in 0..bits.len() {
        header.push_back(bits[i]);

        if i < size - 1 { continue; }

        items = HashSet::new();
        for bit in header.iter() {
            items.insert(*bit);
        }

        if items.len() == size {
            return (i + 1) as i32;
        }

        header.pop_front();
    }

    -1
}

fn main() {
    let signal: String = fs::read_to_string("input.txt")
        .expect("Error reading input");

    println!("{}", solve(signal.clone(), 4));
    println!("{}", solve(signal.clone(), 14));
}