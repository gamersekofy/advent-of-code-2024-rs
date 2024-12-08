use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Part1;

impl Part1 {
    fn parse_file(file_path: String) -> (Vec<i32>, Vec<i32>) {
        let (mut left_list, mut right_list) = (Vec::new(), Vec::new());

        if let Ok(file) = File::open(file_path) {
            for line in BufReader::new(file).lines() {
                if let Ok(line) = line {
                    let numbers: Vec<i32> = line
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();

                    left_list.push(numbers[0]);
                    right_list.push(numbers[1]);
                }
            }
        }

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
