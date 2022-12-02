use advent_of_code::common::{SectionSelection, fetch_input};

pub fn invoke(selection: SectionSelection) {
    println!("Day One: {:?}", selection);
    match selection {
        SectionSelection::All => {
            section1();
            section2();
        },
        SectionSelection::Single(1) => { section1(); },
        SectionSelection::Single(2) => { section2(); },
        _ => println!("Section {:#?} not implemented", selection),
    }
}

fn section1() -> u32 {
    let input = fetch_input(1, 1);
    println!("Section One Input is {} lines long", input.lines().count());
    0
}

fn section2() -> u32 {
    let input = fetch_input(1, 2);
    println!("Section Two Input is {} lines long", input.lines().count());
    0
}