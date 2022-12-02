use std::fs;

#[derive(Debug)]
pub enum SectionSelection {
    All,
    Single(u8),
}

fn format_input_file_path(day: u8, section: u8) -> String {
    format!("data/days/{day}/sections/{section}/input", day=day, section=section)
}

fn load_input_file(day: u8, section: u8) -> Result<String, std::io::Error> {
    fs::read_to_string(format_input_file_path(day, section))
}

pub fn fetch_input(day: u8, section: u8) -> String {
    load_input_file(day, section)
    .expect(
        format!("Failed to load input file '{}'", format_input_file_path(day, section))
            .as_str()
    )
}