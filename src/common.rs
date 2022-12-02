use std::fs;

fn format_input_file_path(day: u8, test: Option<u8>) -> String {
    if let Some(test) = test {
        format!("data/days/{day_no}/test{test_no}", day_no = day, test_no = test)
    } else {
        format!("data/days/{day_no}/input", day_no = day)
    }
}

fn load_input_file(day: u8, test: Option<u8>) -> Result<String, std::io::Error> {
    fs::read_to_string(format_input_file_path(day, test))
}

pub fn fetch_input(day: u8, test: Option<u8>) -> String {
    load_input_file(day, test)
    .expect(
        format!("Failed to load input file '{}'", format_input_file_path(day, test))
            .as_str()
    )
}