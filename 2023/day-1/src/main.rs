use std::{collections::HashMap, fs};

fn get_input() -> String {
    fs::read_to_string("input.txt").expect("Should have been able to read the file")
}

fn first_part(input: String) {
    let digits: Vec<char> = input
        .chars()
        .filter(|c| c.is_numeric() || *c == '\n')
        .collect::<Vec<char>>();

    let digits: u32 = digits
        .split(|c| *c == '\n')
        .map(|digits| digits.iter().map(|d| d.to_digit(10).unwrap()).collect())
        .map(|digits: Vec<u32>| digits.first().unwrap() * 10 + digits.last().unwrap())
        .reduce(|acc, curr| acc + curr)
        .unwrap();

    println!("{:?}", digits);
}

fn second_part(input: String) {
    let mapping: HashMap<&str, u32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let digits: u32 = input
        .chars()
        .collect::<Vec<char>>()
        .split(|c| *c == '\n')
        .map(|chars| {
            let mut vec: Vec<u32> = Vec::new();

            let mut chars = String::from_iter(chars);

            while !chars.is_empty() {
                if let Some(num) = mapping.keys().find(|number| chars.starts_with(*number)) {
                    vec.push(*mapping.get(num).unwrap());
                }

                if chars.is_empty() {
                    continue
                }

                let char = chars.remove(0);

                if char.is_digit(10) {
                    vec.push(char.to_digit(10).unwrap());
                }
            }

            vec
        })
        .map(|digits: Vec<u32>| digits.first().unwrap() * 10 + digits.last().unwrap())
        .reduce(|acc, curr| acc + curr)
        .unwrap();

    println!("{:?}", digits);
}

fn main() {
    let input = get_input();
    //first_part(input)
    second_part(input)
}
