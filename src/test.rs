#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::items::Item;
    use crate::{convert_item_and_strategy_into_items, get_total_score_in_points, parse_items_into_points, parse_line_into_item_and_strategy, parse_line_into_items, parse_symbol_into_item};

    #[test]
    fn test_parse_symbol_into_item() {
        assert_eq!(parse_symbol_into_item("X"), Item::ROCK);
        assert_eq!(parse_symbol_into_item("Y"), Item::PAPER);
        assert_eq!(parse_symbol_into_item("Z"), Item::SCISSOR);
        assert_eq!(parse_symbol_into_item("A"), Item::ROCK);
        assert_eq!(parse_symbol_into_item("B"), Item::PAPER);
        assert_eq!(parse_symbol_into_item("C"), Item::SCISSOR);

        let result = std::panic::catch_unwind(|| parse_symbol_into_item("J"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_into_items() {
        assert_eq!(parse_line_into_items("A Z"), (Item::ROCK, Item::SCISSOR))
    }

    #[test]
    fn test_parse_items_into_points() {
        assert_eq!(parse_items_into_points((Item::SCISSOR, Item::ROCK)), 6 + 1);
        assert_eq!(parse_items_into_points((Item::ROCK, Item::ROCK)), 3 + 1);
        assert_eq!(parse_items_into_points((Item::SCISSOR, Item::SCISSOR)), 3 + 3);
        assert_eq!(parse_items_into_points((Item::PAPER, Item::ROCK)), 0 + 1);
        assert_eq!(parse_items_into_points((Item::ROCK, Item::PAPER)), 6 + 2);
    }

    #[test]
    fn test_get_total_score_in_points() {
        let data = read_to_string("test-data.txt").unwrap();
        assert_eq!(get_total_score_in_points(data.as_str(), parse_line_into_items), 15);

        assert_eq!(get_total_score_in_points(data.as_str(), |line| {
            convert_item_and_strategy_into_items(parse_line_into_item_and_strategy(line))
        }), 12)
    }
}