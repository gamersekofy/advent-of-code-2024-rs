use days::day_1;

mod days;

fn main() {
    println!("Day 1 Part 1:");

    let input = "src/inputs/day1";
    let total_distance = day_1::Part1::calculate_distance(&input);
    let similarity_score = day_1::Part2::calculate_similarity_score(&input);

    println!("Total distance: {}\nSimilarity score: {}", total_distance, similarity_score);
}
