use std::collections::{HashMap, HashSet};

const FILE_NAME: &str = "input.txt";

fn main() {
    let mut value_dict: HashMap<char, u32> = HashMap::new();
    let mut rucksacks: Vec<(Vec<char>, Vec<char>)> = Vec::new();

    let mut value = 1;
    for letter in 'a'..='z' {
        value_dict.insert(letter, value);
        value += 1;
    }
    for letter in 'A'..='Z' {
        value_dict.insert(letter, value);
        value += 1;
    }

    // Get input
    let input = std::fs::read_to_string(FILE_NAME).expect("Failed to read input.txt file");

    // Parse into
    for line in input.lines() {
        let compartments = line.split_at(line.len() / 2);
        let comp0 = compartments.0.chars().collect();
        let comp1 = compartments.1.chars().collect();
        rucksacks.push((comp0, comp1));
    }

    let mut total = 0;
    let mut group_total = 0;
    for group in rucksacks.chunks(3) {
        for item in group.iter() {
            for letter in item.0.iter() {
                if item.1.contains(&letter) {
                    total += value_dict[&letter];
                    break;
                }
            }
        }

        let mut group_letters: HashSet<char> = value_dict.keys().cloned().collect();
        for sack in group {
            for letter in group_letters.clone().iter() {
                if !(sack.0.contains(letter) || sack.1.contains(letter)) {
                    group_letters.remove(&letter);
                }
            }
        }

        group_total += value_dict[group_letters.iter().next().unwrap()];
    }

    println!("Total value: {total}");
    println!("Group total value: {group_total}");

}
