use clap::Parser;

mod days;
use days::SectionFn;
use advent_of_code::common::*;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The day to run
    #[arg(short, long)]
    day: u8,

    /// The part to run
    #[arg(short, long)]
    section: Option<u8>,

    /// The test case to run
    #[arg(short, long)]
    test: Option<u8>,
}

#[derive(Debug)]
enum SectionSelection {
    All,
    One,
    Two
}

fn time_section(section_fn: SectionFn, input: &str) -> (u32, std::time::Duration) {
    let start = std::time::Instant::now();
    let result = section_fn(input);
    let duration = start.elapsed();

    (result, duration)
}

fn main() {
    let args = Args::parse();

    let day = match args.day {
        1..=25 => args.day,
        _ => panic!("Day '{}' is not valid. Day must be 1 to 25 (inclusive)", args.day),
    };

    let section = match args.section {
        Some(1) => SectionSelection::One,
        Some(2) => SectionSelection::Two,
        None => SectionSelection::All,
        _ => panic!("Section '{}' is not valid. Section must be 1 or 2, or blank for all", args.section.unwrap()),
    };

    let (section1, section2) = days::DAY_SECTIONS[(day - 1) as usize];
    
    let input = fetch_input(day, args.test);
    
    match section {
        SectionSelection::All => {
            let (result, duration) = time_section(section1, &input);
            println!("Day {}, Section {} = {} in {}ms", day, 1, result, duration.as_millis());
            let (result, duration) = time_section(section2, &input);
            println!("Day {}, Section {} = {} in {}ms", day, 2, result, duration.as_millis());
        },
        SectionSelection::One => {
            let (result, duration) = time_section(section1, &input);
            println!("Day {}, Section {} = {} in {}ms", day, 1, result, duration.as_millis());
        },
        SectionSelection::Two => {
            let (result, duration) = time_section(section2, &input);
            println!("Day {}, Section {} = {} in {}ms", day, 2, result, duration.as_millis());
        },
    };
}
