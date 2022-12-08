use std::error::Error;
use std::fs;

fn calculate_points(moves: &str, points: [[u32; 3]; 3]) -> u32 {
    moves.lines().fold(0, |acc, x| {
        acc + points[(x.bytes().next().unwrap() - 65) as usize]
            [(x.bytes().last().unwrap() - 88) as usize]
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let moves = fs::read_to_string("input.txt")?;
    let incorrect_points: [[u32; 3]; 3] = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];
    let correct_points: [[u32; 3]; 3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
    println!(
        "You get {} Points for incorrect strategy",
        calculate_points(moves.as_str(), incorrect_points)
    );
    println!(
        "You get {} Points for correct strategy",
        calculate_points(moves.as_str(), correct_points)
    );
    Ok(())
}
