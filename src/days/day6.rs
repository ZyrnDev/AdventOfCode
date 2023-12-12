#[derive(Debug)]
struct Race {
    time: usize,
    distance_record: usize,
}

impl Race {
    fn simulate(&self, button_held_duration: usize) -> usize {
        if self.time < button_held_duration {
            return 0;
        }

        let travel_duration = self.time - button_held_duration;    
        let speed = button_held_duration;
        let distance = speed * travel_duration;

        distance
    }

    fn does_break_record(&self, button_held_duration: usize) -> bool {
        self.simulate(button_held_duration) > self.distance_record
    }
}

// Parse a list of numbers seperated by whitespace
fn parse_number_list<Number: std::str::FromStr>(text: &str) -> Result<Vec<Number>, &'static str> {
    text
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<Number>().map_err(|_| "Invalid number"))
        .collect()
}

fn parser(input: &str) -> Result<Vec<Race>, &'static str> {
    let regexp = regex::Regex::new(r"^(?P<MeasurementLabel>\w+):\s*(?P<MeasurementValues>(\d+\s+)+\d+)\s*$").map_err(|_| "Invalid regexp")?;

    let captures = input.lines()
        .map(|line| 
            regexp.captures(line)
                .ok_or("Unable to parse line")
                .and_then(|cap|
                    Ok((
                        cap.name("MeasurementLabel").ok_or("Unable to find MeasurementLabel")?.as_str(),
                        cap.name("MeasurementValues").ok_or("Unable to find MeasurementValues")?.as_str()
                    ))
                )
                .and_then(|(label, values)| 
                    Ok((
                        label,
                        parse_number_list::<usize>(values)?
                    ))
                )
        )
        .collect::<Result<Vec<_>, _>>()?;

    let mut times = Vec::new();
    let mut distance_records = Vec::new();

    for (label, values) in captures {
        match label {
            "Time" => times.extend(values),
            "Distance" => distance_records.extend(values),
            _ => return Err("Invalid label")?,
        }
    }

    if times.len() != distance_records.len() {
        return Err("Invalid number of values")?;
    }

    Ok(
        times.into_iter()
            .zip(distance_records.into_iter())
            .map(|(time, distance_record)| 
                Race { time, distance_record }
            )
            .collect()
    )
}

pub fn section1(input: &str) -> u32 {
    let races = parser(input).unwrap();

    races.iter()
        .map(|race| 
            (0..race.time)
                .filter(|hold_duration| race.does_break_record(*hold_duration))
                .count()
        )
    .fold(1, |acc, x| acc * x as u32)
}

pub fn section2(input: &str) -> u32 {
    let races = parser(input).unwrap();

    let (time_str, distance_record_str) = races.iter()
        .fold((String::new(), String::new()), |(mut time, mut distance_record), race| {
            time.push_str(race.time.to_string().as_str());
            distance_record.push_str(race.distance_record.to_string().as_str());
            (time, distance_record)
        });

    let race = Race { 
        time: time_str.parse().unwrap(),
        distance_record: distance_record_str.parse().unwrap(),
    };

    (0..race.time).filter(|hold_duration| race.does_break_record(*hold_duration)).count() as u32

}