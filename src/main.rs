use clap::Parser;
use paste::paste;

mod days;
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

macro_rules! execute_section {
    ($day:literal, $section:literal, $input:expr) => {
        paste! {
            days::[<day $day>]::[<section $section>]($input)
        }
    };
}

fn execute_section(day: u8, section: u8, input: &str) {
    let result = match (day, section) {
        (1, 1) => execute_section!(1, 1, input),
        (1, 2) => execute_section!(1, 2, input),
        (2, 1) => execute_section!(2, 1, input),
        (2, 2) => execute_section!(2, 2, input),
        _ => panic!("Day {}, Section {} not implemented", day, section),
    };

    println!("Day {}, Section {} = {}", day, section, result);
}

fn main() {
    let args = Args::parse();

    let day = match args.day {
        1..=25 => args.day,
        _ => panic!("Day '{}' is not valid. Day must be 1 to 25 (inclusive)", args.day),
    };
    
    let input = fetch_input(day, args.test);

    match args.section {
        Some(section) => execute_section(args.day, section, &input),
        None => {
            execute_section(args.day, 1, &input);
            execute_section(args.day, 2, &input);
        }
    };
}
