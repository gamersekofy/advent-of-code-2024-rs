use days::day_1;

mod days;

fn main() {
    println!("Day 1 Part 1");

    let input = String::from("src/inputs/day1");
    let total_distance = day_1::Part1::calculate_distance(input);

    println!("{}", total_distance);
}
