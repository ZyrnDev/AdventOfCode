struct Food {
    calories: u32,
}

impl Food {
    fn new(calories: u32) -> Food {
        Food {
            calories,
        }
    }
}

struct Elf {
    food: Vec<Food>
}

impl Elf {
    fn new() -> Elf {
        Elf {
            food: Vec::new()
        }
    }

    fn add_food(&mut self, calories: u32) {
        self.food.push(Food::new(calories));
    }

    fn total_calories(&self) -> u32 {
        self.food.iter().map(|f| f.calories).sum()
    }
}

fn parse_elf_rations(input: &str) -> Vec<Elf> {
    let mut elfs: Vec<Elf> = Vec::new();
    {
        let mut next_elf = Elf::new();
        for line in input.lines() {
            if line.is_empty() {
                elfs.push(next_elf);
                next_elf = Elf::new();
            } else if let Some(calories) = line.parse::<u32>().ok() {
                next_elf.add_food(calories);
            }
        }
        elfs.push(next_elf);
    }
    elfs
}

pub fn section1(input: &str) -> u32 {
    let elfs = parse_elf_rations(&input);  

    

    elfs.iter()
        .map(|elf| elf.total_calories())
        .reduce(u32::max)
        .unwrap_or(0)
}

pub fn section2(input: &str) -> u32 {
    let elfs = parse_elf_rations(&input);

    let mut calories: Vec<u32> = elfs.iter().map(|elf| elf.total_calories()).collect();
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum()
}