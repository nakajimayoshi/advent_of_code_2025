use std::{
    fs,
    io::{self, Error},
    num::Wrapping,
    str::FromStr,
};

struct Rotation {
    is_right: bool,
    steps: u32,
}

impl FromStr for Rotation {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_at(1);

        let direction = split.0;

        let steps = u32::from_str(split.1).expect("expected integer after direction string");

        match direction.to_uppercase().as_ref() {
            "L" => Ok(Self {
                is_right: false,
                steps,
            }),
            "R" => Ok(Self {
                is_right: true,
                steps,
            }),
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "invalid rotation string found in file. Must be prefixed with L or R {} followed by a digit",
                    s
                ),
            )),
        }
    }
}

struct Dial {
    size: u32,
    number: u32,
    zeroed_count: u32,
    zeroed_clicks_count: u32,
}

impl Dial {
    fn new(size: u32, initial: u32) -> Self {
        if initial == 0 {
            Self {
                size,
                number: initial,
                zeroed_count: 1,
                zeroed_clicks_count: 0,
            }
        } else {
            Self {
                size,
                number: initial,
                zeroed_count: 0,
                zeroed_clicks_count: 0,
            }
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        let start = self.number;
        let modulus = self.size + 1;
        let rotation_steps = rotation.steps.rem_euclid(modulus);
        let quotient = rotation.steps.div_euclid(modulus);
        self.zeroed_clicks_count += quotient;
        if rotation.is_right {
            let result_value = self.number + rotation_steps;
            let result_lands_on_zero = result_value.rem_euclid(100) == 0 || result_value == 0;
            if result_value > self.size || result_lands_on_zero {
                self.zeroed_clicks_count += 1;
            }
            self.number = result_value.rem_euclid(modulus);
        } else {
            let subtracted: i32 = (self.number as i32 - rotation_steps as i32) as i32;
            let result_lands_on_zero = subtracted.rem_euclid(100) == 0 || subtracted == 0;
            if (subtracted < 0 || result_lands_on_zero) && start != 0 {
                self.zeroed_clicks_count += 1;
            }
            self.number = subtracted.rem_euclid(modulus as i32) as u32;
        }

        if self.number == 0 {
            self.zeroed_count += 1;
        }
    }
}

mod test {
    use std::{collections::HashMap, str::FromStr, u32, vec};

    use crate::day1::{Dial, Rotation};

    #[test]
    fn dial_should_wrap() {
        let mut dial = Dial::new(99, u32::MIN);

        let results = vec![
            ("left three", 97, Rotation::from_str("L3").unwrap()),
            ("right two", 99, Rotation::from_str("R2").unwrap()),
            ("right two wrap", 1, Rotation::from_str("R2").unwrap()),
            ("right fifty-four", 55, Rotation::from_str("R54").unwrap()),
            ("left fifty-five", 0, Rotation::from_str("L55").unwrap()),
            (
                "right 198 wrap twice",
                98,
                Rotation::from_str("R198").unwrap(),
            ),
        ];

        for (test_name, result, rotation) in results {
            dial.rotate(&rotation);
            print!("{}", test_name);
            assert_eq!(result, dial.number);
            print!("pass ✅\n");
        }
    }

    #[test]
    fn dial_should_count_0_clicks() {
        const DIAL_SIZE: u32 = 99;
        const INITIAL_NUMBER: u32 = 50;
        let mut dial = Dial::new(DIAL_SIZE, INITIAL_NUMBER);
        let results = vec![
            ("L68", 82, 1),
            ("L30", 52, 1),
            ("R48", 0, 2),
            ("L5", 95, 2),
            ("R60", 55, 3),
            ("L55", 0, 4),
            ("L1", 99, 4),
            ("L99", 0, 5),
        ];

        assert_eq!(dial.zeroed_clicks_count, 0);

        for (test_case, expected_result_number, expected_click_count_total) in results {
            let rotation = Rotation::from_str(test_case).unwrap();
            dial.rotate(&rotation);
            print!("test case {}: ", test_case);
            assert_eq!(dial.number, expected_result_number);
            assert_eq!(dial.zeroed_clicks_count, expected_click_count_total);
            print!("pass ✅\n");
        }
    }
}

pub fn solution() {
    const DIAL_SIZE: u32 = 99;
    const DIAL_START: u32 = 50;
    let mut dial = Dial::new(DIAL_SIZE, DIAL_START);

    let mut rotations: Vec<Rotation> = vec![];

    for line in fs::read_to_string("./data/puzzle_1.txt").unwrap().lines() {
        let rotation = Rotation::from_str(line).expect("failed to parse rotation from string");
        rotations.push(rotation);
    }

    for rotation in rotations {
        dial.rotate(&rotation);
    }

    println!("part 1: {}", dial.zeroed_count);
    println!("part 2: {}", dial.zeroed_clicks_count);
}
