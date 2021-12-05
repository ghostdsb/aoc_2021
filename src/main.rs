mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    println!("Hello, AOC!");

    let ans1 = day_1::sol::aoc(1, 1);
    let ans2 = day_1::sol::aoc(1, 2);
    
    let ans3 = day_2::sol::aoc(2, 1);
    let ans4 = day_2::sol::aoc(2, 2);
    
    let ans5 = day_3::sol::aoc(3, 1);
    let ans6 = day_3::sol::aoc(3, 2);

    let ans7 = day_4::sol::aoc(4, 1);
    let ans8 = day_4::sol::aoc(4, 2);

    let ans9 = day_5::sol::aoc(5, 1);
    let ans10 = day_5::sol::aoc(5, 2);

    println!("\n\nday1: part1: {}", ans1);
    println!("day1: part2: {}", ans2);
    println!("day2: part1: {}", ans3);
    println!("day2: part2: {}", ans4);
    println!("day3: part1: {}", ans5);
    println!("day3: part2: {}", ans6);
    println!("day4: part1: {}", ans7);
    println!("day4: part2: {}", ans8);
    println!("day5: part1: {}", ans9);
    println!("day5: part2: {}", ans10);
}
