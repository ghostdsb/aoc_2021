mod day_1;
mod day_2;
mod day_3;

fn main() {
    println!("Hello, AOC!");

    let ans1 = day_1::sol::aoc(1, 1);
    let ans2 = day_1::sol::aoc(1, 2);
    
    let ans3 = day_2::sol::aoc(2, 1);
    let ans4 = day_2::sol::aoc(2, 2);
    
    let ans5 = day_3::sol::aoc(3, 1);
    let ans6 = day_3::sol::aoc(3, 2);

    println!("\n\nday1: part1: {}", ans1);
    println!("day1: part2: {}", ans2);
    println!("day2: part1: {}", ans3);
    println!("day2: part2: {}", ans4);
    println!("day3: part1: {}", ans5);
    println!("day3: part2: {}", ans6);
}
