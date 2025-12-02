use std::{fs, ops::RangeInclusive, str::FromStr};

fn calculate_invalid_ids(ranges: Vec<(u64, u64)>) -> u64 {
    let mut result = 0;
    for (lower_bound, upper_bound) in ranges {
        for id in lower_bound..upper_bound {
            let str = id.to_string();
            let max_window_size = str.len() / 2;
            let mut is_repeating_sequence = true;
            for window_size in 1..max_window_size {
                let mut slice: Vec<char> = str.chars().collect();
                // let mut last_slice: Box<&[char]> = Box::new([]);
                for window in slice.windows(window_size) {
                    // if Box::new(window).ne(&last_slice) {}
                }
            }

            if is_repeating_sequence {
                result += id
            }
        }
    }

    result
}

pub fn solution() {
    let mut id_ranges: Vec<(u64, u64)> = vec![];
    for range_str in fs::read_to_string("./data/puzzle_2.txt")
        .unwrap()
        .split(',')
    {
        println!("{}", range_str);
        let no_trailing_return = range_str.replace('\n', "");
        let split: Vec<&str> = no_trailing_return.split('-').collect();
        let lower_bound_str = split.get(0).expect("expected a lower bound");
        let upper_bound_str = split.get(1).expect("expected an upper bound");

        let lower_bound = u64::from_str(lower_bound_str)
            .map_err(|e| format!("invalud inout '{}': {}", lower_bound_str, e));

        let upper_bound = u64::from_str(upper_bound_str)
            .map_err(|e| format!("invalid input '{}': {}", upper_bound_str, e));

        id_ranges.push((lower_bound.unwrap(), upper_bound.unwrap()));
    }

    let answer = calculate_invalid_ids(id_ranges);

    println!("part 1 answer: {}", answer);
}
