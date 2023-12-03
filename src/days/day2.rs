use regex::Regex;

#[derive(Debug)]
struct Game {
    id: usize,

    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    blue: usize,
    red: usize,
    green: usize,
}


impl Game {
    fn max_set(&self) -> Set {
        self.sets.iter().fold(Set::new(), |current_max: Set, set| current_max.max(set))
    }
}

impl Set {
    fn new() -> Set {
        Set {
            blue: 0,
            red: 0,
            green: 0,
        }
    }

    fn can_contain(&self, other: &Set) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn max(&self, other: &Set) -> Set {
        Set {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

// Implement the TryFrom trait for the Game struct
impl TryFrom<&str> for Game {
    type Error = &'static str;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let rexexp = Regex::new(r"^Game (\d+)$").unwrap();

        let sections = text.split(':').collect::<Vec<&str>>();
        if sections.len() != 2 {
            return Err("Invalid input, must contain two sections separated by a colon");
        }

        let captures = rexexp.captures(sections[0]).ok_or("Invalid input, must start with 'Game <number>'")?;
        let id = captures.get(1).ok_or("Invalid input, must start with 'Game <number>'")?.as_str().parse::<usize>().unwrap();

        Ok(Game {
            id,
            sets: sections[1].trim().split(';').map(|s| s.trim()).map(|s| Set::try_from(s)).collect::<Result<Vec<Set>, &'static str>>()?,
        })
    }
}

// Implement the TryFrom trait for the Set struct
impl TryFrom<&str> for Set {
    type Error = &'static str;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let regexp = Regex::new(r"^(\d+) (red|blue|green)$").unwrap();

        let mut set = Self::new();

        for pull in text.split(',').map(|s| s.trim()) {
            let captures = regexp.captures(pull).ok_or("Invalid input, must be '<number> <colour>'")?;
            let number = captures.get(1).ok_or("Invalid input, must be '<number> <colour>'")?.as_str().parse::<usize>().unwrap();
            let colour = captures.get(2).ok_or("Invalid input, must be '<number> <colour>'")?.as_str();

            match colour {
                "blue" => set.blue = number,
                "red" => set.red = number,
                "green" => set.green = number,
                _ => return Err("Invalid colour, must be 'red', 'blue' or 'green'"),
            }
        }

        Ok(set)
    }
}

pub fn section1(input: &str) -> u32 {
    let games = input.lines().map(|line| Game::try_from(line)).collect::<Result<Vec<Game>, &'static str>>().expect("Invalid input");

    let max_set_total = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    games.iter()
        .filter(|game| max_set_total.can_contain(&game.max_set()))
        .map(|game| game.id)
        .sum::<usize>() as u32
}

pub fn section2(input: &str) -> u32 {
    let games = input.lines().map(|line| Game::try_from(line)).collect::<Result<Vec<Game>, &'static str>>().expect("Invalid input");

    games.iter()
        .map(|game| game.max_set())
        .map(|set| set.red * set.green * set.blue)
        .sum::<usize>() as u32
}