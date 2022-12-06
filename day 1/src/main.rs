use std::error::Error;
use std::fs;

fn calculate_max_calories(calories: &str) -> u32 {
    let mut max_calories: u32 = 0;
    let mut current: u32 = 0;
    calories.lines().for_each(|num| {
        if !num.is_empty() {
            current = current + num.parse::<u32>().expect("Expected a number")
        } else {
            if current > max_calories {
                max_calories = current
            }
            current = 0
        }
    });
    max_calories
}

fn calculate_three_max_calories(calories: &str) -> u32 {
    let mut three_max_calories: Vec<u32> = vec![0; 3];
    let mut current: u32 = 0;
    calories.lines().for_each(|num| {
        if !num.is_empty() {
            current = current + num.parse::<u32>().expect("Expected a number")
        } else {
            if current > three_max_calories[0] {
                three_max_calories[0] = current;
                three_max_calories.sort();
            }
            current = 0
        }
    });
    three_max_calories.iter().sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let calories = fs::read_to_string("input.txt")?;
    println!("{}", calculate_max_calories(calories.as_str()));
    println!("{}", calculate_three_max_calories(calories.as_str()));
    Ok(())
}
