pub mod day1;
// pub mod day2;
// pub mod day3;
// pub mod day4;
// pub mod day5;
// pub mod day6;
// pub mod day7;
// pub mod day8;
// pub mod day9;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

pub type SectionFn = fn(&str) -> u32;
pub type Day = (SectionFn, SectionFn);
pub const DAY_SECTIONS: [Day; 25] = [
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    (day1::section1, day1::section2),
    // (day2::section1, day2::section2),
    // (day3::section1, day3::section2),
    // (day4::section1, day4::section2),
    // (day5::section1, day5::section2),
    // (day6::section1, day6::section2),
    // (day7::section1, day7::section2),
    // (day8::section1, day8::section2),
    // (day9::section1, day9::section2),
    // (day10::section1, day10::section2),
    // (day11::section1, day11::section2),
    // (day12::section1, day12::section2),
    // (day13::section1, day13::section2),
    // (day14::section1, day14::section2),
    // (day15::section1, day15::section2),
    // (day16::section1, day16::section2),
    // (day17::section1, day17::section2),
    // (day18::section1, day18::section2),
    // (day19::section1, day19::section2),
    // (day20::section1, day20::section2),
    // (day21::section1, day21::section2),
    // (day22::section1, day22::section2),
    // (day23::section1, day23::section2),
    // (day24::section1, day24::section2),
    // (day25::section1, day25::section2),
];