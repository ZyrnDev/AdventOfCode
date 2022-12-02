use clap::Parser;

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
}
fn main() {
    let args = Args::parse();

    let section = match args.section {
        Some(section) => SectionSelection::Single(section),
        None => SectionSelection::All,
    };
    
    let invoke: fn(SectionSelection) = match args.day {
        1 => days::day_01::invoke,
        // 2 => days::day_02::invoke,
        // 3 => days::day_03::invoke,
        // 4 => days::day_04::invoke,
        // 5 => days::day_05::invoke,
        // 6 => days::day_06::invoke,
        // 7 => days::day_07::invoke,
        // 8 => days::day_08::invoke,
        // 9 => days::day_09::invoke,
        // 10 => days::day_10::invoke,
        // 11 => days::day_11::invoke,
        // 12 => days::day_12::invoke,
        // 13 => days::day_13::invoke,
        // 14 => days::day_14::invoke,
        // 15 => days::day_15::invoke,
        // 16 => days::day_16::invoke,
        // 17 => days::day_17::invoke,
        // 18 => days::day_18::invoke,
        // 19 => days::day_19::invoke,
        // 20 => days::day_20::invoke,
        // 21 => days::day_21::invoke,
        // 22 => days::day_22::invoke,
        // 23 => days::day_23::invoke,
        // 24 => days::day_24::invoke,
        // 25 => days::day_25::invoke,
        _ => panic!("Day {} not implemented", args.day),
    };

    invoke(section);
    
}
