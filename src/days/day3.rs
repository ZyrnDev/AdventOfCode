#[derive(Debug)]
struct SchematicNumber {
    x: usize,
    y: usize,
    length: usize,
    value: u32,
}

#[derive(Debug)]
struct SchematicSymbol {
    x: usize,
    y: usize,
    symbol: char,
}

#[derive(Debug)]
struct EngineSchematic {
    numbers: Vec<SchematicNumber>,
    symbols: Vec<SchematicSymbol>,
    width: usize,
    height: usize,
}

static NEIGHBOUR_RANGE: std::ops::RangeInclusive<i32> = -1..=1;

impl SchematicNumber {
    fn neighbours(&self, x: usize, y: usize) -> bool {
        let cells_occupied: Vec<(i32, i32)> = (0..self.length).map(|i| ((self.x + i) as i32, self.y as i32)).collect();
        let neighbour_positions: Vec<(i32, i32)> = NEIGHBOUR_RANGE.clone().flat_map(|x| NEIGHBOUR_RANGE.clone().map(move |y| (x,y))).collect();

        cells_occupied.iter()
            .flat_map(|(x, y)| neighbour_positions.iter().map(move |(dx, dy)| (x + dx, y + dy))) // Get all neighbour positions
            .filter(|(x, y)| !cells_occupied.iter().any(|(cx, cy)| cx == x && cy == y)) // Remove cells occupied by the number
            .any(|(nx, ny)| nx == x as i32 && ny == y as i32)
    }

}

fn parser(input: &str) -> Result<EngineSchematic, &'static str> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap_or(input).len();

    if input.lines().any(|row| row.len() != width) {
        return Err("Invalid input, all rows must be the same length");
    }

    let mut schematic = EngineSchematic {
        numbers: Vec::new(),
        symbols: Vec::new(),
        width,
        height,
    };

    for (y, line) in input.lines().enumerate() {
        let mut chars: Vec<(usize, char)> = line.chars().enumerate().collect();

        loop {
            chars = chars.into_iter().skip_while(|(_, c)| c == &'.').collect();
            if chars.is_empty() {
                break;
            }

            match chars.first().map(|(_, c)| c) {
                Some('0'..='9') => {
                    let number = chars.iter().cloned().take_while(|(_, c)| c.is_digit(10)).collect::<Vec<(usize, char)>>();
                    let length = number.len();
                    schematic.numbers.push(SchematicNumber {
                        x: number.first().map(|(i, c)| *i).expect("Invalid input, must be a number"),
                        y: y,
                        length: number.len(),
                        value: number.iter().map(|(_, c)| c.to_digit(10).unwrap()).fold(0, |acc, digit| acc * 10 + digit),
                    });
                    chars = chars.into_iter().skip(length).collect();
                }
                Some(c) => {
                    let symbol = chars.first();
                    schematic.symbols.push(SchematicSymbol {
                        x: symbol.map(|(i, _)| *i).expect("Could not get symbol position"),
                        y: y,
                        symbol: symbol.map(|(_, c)| *c).expect("Could not get symbol"),
                    });
                    chars = chars.into_iter().skip(1).collect();
                },
                None => return Err("Invalid parser state during parsing"),
            }
        }
    }  

    Ok(schematic)
}

pub fn section1(input: &str) -> u32 {
    let schematic = parser(input).unwrap();

    let mut sum = 0;

    for (x, y, symbol) in schematic.symbols.iter().map(|s| (s.x, s.y, s.symbol)) {
        sum += schematic.numbers.iter().filter(|n| n.neighbours(x, y)).map(|n| n.value).sum::<u32>();
    }

    sum
}

pub fn section2(input: &str) -> u32 {
    let schematic = parser(input).unwrap();
    
    schematic.symbols.iter()
        .filter(|s| s.symbol == '*')
        .map(|s| schematic.numbers.iter().filter(|n| n.neighbours(s.x, s.y)).collect::<Vec<&SchematicNumber>>())
        .filter(|n| n.len() == 2)
        .map(|n| n.iter().fold(1, |acc, n| acc * n.value))
        .sum::<u32>() as u32
}