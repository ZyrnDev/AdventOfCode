use regex::Regex;

type Number = isize;

struct Almanac {
    query_category: String,
    query: Vec<Number>,

    categories: Vec<CategoryMap>,
}

struct CategoryMap {
    source_category: String,
    target_category: String,

    mappings: Vec<RangeMapping>,
}

#[derive(Debug, Clone, Copy)]
struct RangeMapping {
    source: Number,
    destination: Number,
    length: Number,
}


impl Almanac {
    fn new(query_category: String, query: Vec<Number>, categories: Vec<CategoryMap>) -> Self {
        Self { query_category, query, categories }
    }

    fn get_category(&self, label: &str) -> Option<&CategoryMap> {
        self.categories.iter()
            .find(|category| category.source_category == label)
    }

    fn categories(&self) -> Vec<&CategoryMap> {
        let mut categories = Vec::new();

        let mut current_category = self.query_category.as_str();

        while let Some(category) = self.get_category(current_category) {
            categories.push(category);
            current_category = category.target_category.as_str();
        }

        categories
    }
}

impl TryFrom<&str> for Almanac {
    type Error = &'static str;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let paragraphs = text.lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .split(|line| line.is_empty())
            .map(|lines| lines.join("\n"))
            .collect::<Vec<_>>();

        if paragraphs.len() < 2 {
            return Err("Invalid number of paragraphs");
        }

        let regexp = Regex::new(r"^(\w+)s:\s+((\d+\s*)+)").unwrap();
        let captures = regexp.captures(&paragraphs[0]).ok_or("Invalid query")?;
        let query_name = captures.get(1).ok_or("Missing query label")?.as_str();
        let query_values = captures.get(2).ok_or("Missing query values")?.as_str()
            .split_whitespace()
            .map(|word| word.parse::<Number>())
            .collect::<Result<Vec<_>, _>>().map_err(|_| "Invalid query")?;

        let categories = paragraphs[1..].iter()
            .map(|paragraph| CategoryMap::try_from(paragraph.as_str()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(query_name.to_string(), query_values, categories))
    }
}

impl CategoryMap {
    fn new(source_category: String, target_category: String, mappings: Vec<RangeMapping>) -> Self {
        Self { source_category, target_category, mappings }
    }

    fn apply(&self, value: Number) -> Number {
        self.mappings.iter()
            .find_map(|mapping| mapping.apply(value))
            .unwrap_or(value)
    }
}

impl TryFrom<&str> for CategoryMap {
    type Error = &'static str;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let lines = text.lines().collect::<Vec<_>>();

        if lines.len() < 2 {
            return Err("Invalid number of lines");
        }

        let regexp = Regex::new(r"^(\w+)-to-(\w+) map:$").unwrap();

        let captures = regexp.captures(lines[0]).ok_or("Invalid category map label")?;
        
        let source_category = captures.get(1).ok_or("Invalid source category")?.as_str().to_string();
        let target_category = captures.get(2).ok_or("Invalid target category")?.as_str().to_string();

        let mappings = lines[1..].iter()
            .map(|line| RangeMapping::try_from(*line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(source_category, target_category, mappings))
    }
}

impl RangeMapping {
    fn new(source: Number, destination: Number, length: Number) -> Self {
        RangeMapping { source, destination, length }
    }

    fn apply(&self, value: Number) -> Option<Number> {
        if value < self.source || value > self.source + self.length {
            return None;
        }

        Some(self.destination + (value - self.source))
    }

    // fn differential(&self) -> Number {
    //     self.destination - self.source
    // }
}

impl TryFrom<&str> for RangeMapping {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
       let parts  = value.split_whitespace()
        .filter_map(|s| s.parse::<Number>().ok())
        .collect::<Vec<_>>();

       if parts.len() != 3 {
           return Err("Invalid number of parts");
       }

       Ok(Self::new(parts[1], parts[0], parts[2]))
    }
}

pub fn section1(input: &str) -> u32 {
    let almanac = Almanac::try_from(input).unwrap();

    let categories = almanac.categories();
    
    almanac.query.iter()
        .map(|value| categories.iter()
            .fold(*value, |value, category| category.apply(value)))
        .min()
        .map(|value| value as u32)
        .unwrap_or(0)
}

pub fn section2(input: &str) -> u32 {
    let almanac = Almanac::try_from(input).unwrap();

    let categories = almanac.categories();

    almanac.query.chunks_exact(2)
        .map(|pair| (pair[0], pair[1])) // Convert the chunk into a pair
        .flat_map(|(start, length)| start..start+length) // Transform the pair into a sequence of values
        .map(|value| categories.iter()
            .fold(value, |value, category| category.apply(value)))
        .min()
        .map(|value| value as u32)
        .unwrap_or(0)
}