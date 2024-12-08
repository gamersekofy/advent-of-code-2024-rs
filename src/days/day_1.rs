use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Part1;

impl Part1 {
    fn parse_file(file_path: String) -> (Vec<i32>, Vec<i32>) {
        let (mut left_list, mut right_list) = (Vec::new(), Vec::new());

        match File::open(file_path) {
            Ok(file) => {
                for line in BufReader::new(file).lines().map_while(Result::ok) {
                    let numbers: Vec<i32> = line
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();

                    left_list.push(numbers[0]);
                    right_list.push(numbers[1]);
                }
            }
            Err(_) => eprintln!("We couldn't find that file 😞"),
        };

        (left_list, right_list)
    }

    pub fn calculate_distance(file_path: String) -> u32 {
        let (mut left_list, mut right_list) = Part1::parse_file(file_path);

        left_list.sort();
        right_list.sort();

        let sum: i32 = left_list
            .iter()
            .zip(right_list.iter())
            .map(|(left, right)| (left - right).abs())
            .sum();

        sum as u32
    }
}
