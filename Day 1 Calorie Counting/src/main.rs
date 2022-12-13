const FILE_NAME: &str = "input.txt";

fn main() {
    let mut total_calories: Vec<u32> = Vec::new();

    // Load file in
    let input = std::fs::read_to_string(FILE_NAME).expect("Failed to read input.txt file");

    // Parse into
    let mut sub_total: u32 = 0;
    for line in input.lines() {
        if line.len() > 0 {
            sub_total += line.parse::<u32>().expect("Failed to parse line");
        } else {
            total_calories.push(sub_total);
            sub_total = 0;
        }
    }

    // Get largest value
    total_calories.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let highest_calories = total_calories[0];
    let second_highest_calories = total_calories[1];
    let third_highest_calories = total_calories[2];
    let top_three_total = highest_calories + second_highest_calories + third_highest_calories;

    println!("The top 3 most calories are: ");
    println!("  {highest_calories}");
    println!("  {second_highest_calories}");
    println!("  {third_highest_calories}");

    println!("\nTotal from the top 3 is: {top_three_total}");
}
