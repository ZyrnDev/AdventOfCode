const TEXT_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

// Get the first and last number in the line
fn get_digits(line: &str, include_words: bool) -> Option<(usize, usize)> {

    let mut text_matches: Vec<(usize, usize)> = Vec::new();
    if include_words {
        for i in 0..line.len() {
            let substr = &line[i..];
            for (value, digit_text) in TEXT_DIGITS.iter().enumerate() {
                if substr.starts_with(digit_text) {
                    text_matches.push((i, value));
                }
            }
        }
    }

    let numeric_matches: Vec<(usize, usize)> = line.chars()
        .enumerate()
        .filter_map(|(i, c)| c.to_digit(10).map(|c| (i, c as usize)))
        .collect();

    let mut all_matches = Vec::from_iter(text_matches.into_iter().chain(numeric_matches.into_iter()));
    all_matches.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

    let (_, first) = *all_matches.first()?;
    let (_, last) = *all_matches.last()?;

    Some((first, last))
}

fn get_calibration(line: &str, include_words: bool) -> Option<usize> {
    let (first, last) = get_digits(line, include_words)?;
    let calibration = first * 10 + last;

    Some(calibration)
}

pub fn section1(input: &str) -> u32 {
    input.lines()
        .filter_map(|line| get_calibration(line, false))
        .sum::<usize>() as u32
}

pub fn section2(input: &str) -> u32 {
    input.lines()
        .filter_map(|line| get_calibration(line, true))
        .sum::<usize>() as u32
}