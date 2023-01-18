use std::fs::read_to_string;
use crate::items::Item;
use crate::strategies::Strategy;

mod items;
mod strategies;
mod test;

fn parse_symbol_into_item(symbol: &str) -> Item {
    match symbol {
        "A" => Item::ROCK,
        "B" => Item::PAPER,
        "C" => Item::SCISSOR,
        "X" => Item::ROCK,
        "Y" => Item::PAPER,
        "Z" => Item::SCISSOR,
        _ => panic!("Undefined symbol: {}", symbol)
    }
}

fn parse_symbol_into_strategy(symbol: &str) -> Strategy {
    match symbol {
        "X" => Strategy::LOSE,
        "Y" => Strategy::DRAW,
        "Z" => Strategy::WIN,
        _ => panic!("Undefined symbol: {}", symbol)
    }
}

fn parse_line_into_items(line: &str) -> (Item, Item) {
    let items = line.split(" ").map(|symbol| {
        parse_symbol_into_item(symbol)
    }).collect::<Vec<Item>>();

    (items[0], items[1])
}

fn parse_line_into_item_and_strategy(line: &str) -> (Item, Strategy) {
    let items = line.split(" ").collect::<Vec<_>>();

    (parse_symbol_into_item(items[0]), parse_symbol_into_strategy(items[1]))
}

fn convert_item_and_strategy_into_items(item_and_strategy: (Item, Strategy)) -> (Item, Item) {
    match item_and_strategy.1 {
        Strategy::WIN => match item_and_strategy.0 {
            Item::ROCK => (item_and_strategy.0, Item::PAPER),
            Item::PAPER => (item_and_strategy.0, Item::SCISSOR),
            Item::SCISSOR => (item_and_strategy.0, Item::ROCK),
        }
        Strategy::LOSE => match item_and_strategy.0 {
            Item::ROCK => (item_and_strategy.0, Item::SCISSOR),
            Item::PAPER => (item_and_strategy.0, Item::ROCK),
            Item::SCISSOR => (item_and_strategy.0, Item::PAPER),
        }
        Strategy::DRAW => match item_and_strategy.0 {
            Item::ROCK => (item_and_strategy.0, item_and_strategy.0),
            Item::PAPER => (item_and_strategy.0, item_and_strategy.0),
            Item::SCISSOR => (item_and_strategy.0, item_and_strategy.0),
        }
    }
}

fn parse_items_into_points(items: (Item, Item)) -> usize {
    if items.0 == items.1 {
        return 3 + items.1 as usize;
    }

    if items.0 == Item::SCISSOR && items.1 == Item::ROCK
    || items.0 == Item::ROCK    && items.1 == Item::PAPER
    || items.0 == Item::PAPER   && items.1 == Item::SCISSOR {
        return 6 + items.1 as usize
    }

    items.1 as usize
}

fn get_total_score_in_points(lines: &str, function: fn(&str) -> (Item, Item)) -> usize {
    lines.split("\n").map(|line| {
        parse_items_into_points(function(line))
    }).sum()
}

fn main() {
    let data = read_to_string("final-data.txt").unwrap();
    println!("PART 1: {}", get_total_score_in_points(data.as_str(), parse_line_into_items));
    println!("PART 2: {}", get_total_score_in_points(data.as_str(), |line| {
        convert_item_and_strategy_into_items(parse_line_into_item_and_strategy(line))
    }));
}
