use std::collections::HashSet;

#[allow(unused_variables)]
pub fn section1(input: &str) -> u32 {
    let rucksacks = parse_rucksacks(input);
    let rucksack_sets: Vec<(HashSet<char>, HashSet<char>)> = rucksacks
        .iter()
        .map(|r| (HashSet::from_iter(r.compartments[0].clone()), HashSet::from_iter(r.compartments[1].clone())))
        .collect();

    let overlap: Vec<char> = rucksack_sets
        .iter()
        .map(|(a, b)| a.intersection(b).map(|c| *c).collect::<Vec<char>>())
        .map(|set| set.into_iter().nth(0).unwrap())
        .collect();

    overlap.into_iter()
        .map(|c| get_priority(c))
        .sum()
}

#[allow(unused_variables)]
pub fn section2(input: &str) -> u32 {
    let raw_rucksacks = parse_raw_rucksacks(input);
    let rucksack_sets: Vec<HashSet<char>> = raw_rucksacks
        .into_iter()
        .map(|r| HashSet::from_iter(r.items))
        .collect();

    let overlap_items: Vec<char> = rucksack_sets.chunks(3)
        .map(|group_sets| {
            let first_intersect: HashSet<char> = group_sets[0].intersection(&group_sets[1]).map(|c| *c).collect();
            let full_intersect: HashSet<char> = first_intersect.intersection(&group_sets[2]).map(|c| *c).collect();
            full_intersect
        })
        .map(|overlap| overlap.into_iter().nth(0).unwrap())
        .collect();
    
    overlap_items.into_iter()
        .map(|c| get_priority(c))
        .sum()
}

type Item = char;
#[derive(Debug)]
struct Rucksack {
    compartments: [Vec<Item>; 2],
}

impl Rucksack {
    fn from_raw(raw: &RawRucksack) -> Self {
        let compartment_length = raw.items.len() / 2;

        let compartments: Vec<Vec<Item>> = raw
            .items
            .chunks(compartment_length)
            .map(|compartment| compartment.to_vec())
            .collect();

        Self {
            compartments: [compartments[0].clone(), compartments[1].clone()],
        }
    }
}

#[derive(Debug)]
struct RawRucksack {
    items: Vec<Item>,
}

impl RawRucksack {
    fn from_str(input: &str) -> Self {
        let items = input.chars().collect();
        Self { items }
    }
}

fn parse_raw_rucksacks(input: &str) -> Vec<RawRucksack> {
    input.lines().map(RawRucksack::from_str).collect()
}

fn parse_rucksacks(input: &str) -> Vec<Rucksack> {
    parse_raw_rucksacks(input)
        .iter()
        .map(Rucksack::from_raw)
        .collect()
}

fn get_priority(c: char) -> u32 {
    match c {
        // a - z = 1 - 26
        'a'..='z' => c as u32 - 96,
        // A - Z = 27 - 52
        'A'..='Z' => c as u32 - 64 + 26,
        _ => 0,
    }
}