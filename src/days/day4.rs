#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

fn split_pair<'a>(text: &'a str, separator: &str) -> Result<(&'a str, &'a str), &'static str> {
    let split = text.split(separator).collect::<Vec<&str>>();

    if split.len() != 2 {
        return Err("Cannot split string, the input must be a pair");
    }

    Ok((split[0].trim(), split[1]))
}

fn words(text: &str) -> Vec<&str> {
    text.split_whitespace().filter(|word| !word.is_empty()).collect()
}

fn get_id(text: &str) -> Result<u32, &'static str> {
    let words = words(text);

    if words.len() != 2 {
        return Err("Cannot get label, the input must be a pair");
    }

    if words[0] != "Card" {
        return Err("Cannot get label, the input must start with 'Card'");
    }

    words[1].parse::<u32>().map_err(|_| "Cannot parse card id")
}

fn parse_card(text: &str) -> Result<Card, &'static str> {
    let (label, values) = split_pair(text, ":")?;

    let id = get_id(label)?;        

    let (winning_numbers_str, owned_numbers_str) = split_pair(values, "|")?;

    let winning_numbers = words(winning_numbers_str).into_iter()
        .map(|number| number.parse::<u32>().map_err(|_| "Cannot parse winning number"))
        .collect::<Result<Vec<u32>, &'static str>>()?;

    let owned_numbers = words(owned_numbers_str).into_iter()
        .map(|number| number.parse::<u32>().map_err(|_| "Cannot parse owned number"))
        .collect::<Result<Vec<u32>, &'static str>>()?;

    Ok(Card {
        id,
        winning_numbers,
        owned_numbers,
    })
}

fn parser(input: &str) -> Result<Vec<Card>, &'static str> {
    input.lines()
        .map(|line| parse_card(line))
        .collect()
}

pub fn section1(input: &str) -> u32 {
    let cards = parser(input).unwrap();

    cards.iter()
        .map(|card| card.winning_numbers.iter().filter(|number| card.owned_numbers.contains(number)).count())
        .map(|count| match count {
            0 => 0,
            _ => 2u32.pow(count as u32 - 1),
        })
        .sum::<u32>()
}

fn add_copies(cards: &mut Vec<(usize, usize, usize)>, id: usize, count: usize) {
    for idx in 0..cards.len() {
        let card = cards[idx];
        if card.0 == id {
            cards[idx] = (card.0, card.1, card.2 + count);
        }
    }
}

pub fn section2(input: &str) -> u32 {
    let cards = parser(input).unwrap();
    
    let card_results = cards.iter()
        .map(|card| (card.id, card.winning_numbers.iter().filter(|number| card.owned_numbers.contains(number)).count()))
        .collect::<Vec<(u32, usize)>>();

    let mut winnings = card_results.iter()
        .map(|(id, matching_cards)| (*id as usize, *matching_cards as usize, 1))
        .collect::<Vec<(usize, usize, usize)>>();

    for i in 0..winnings.len() {
        let (id, matching_cards, count) = winnings[i];

        for idx in id+1..id+1+matching_cards {
            add_copies(&mut winnings, idx, count);
        }
    }

    winnings.iter().map(|(_, _, count)| count).sum::<usize>() as u32
}