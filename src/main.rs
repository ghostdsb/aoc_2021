mod day_1;
mod day_2;

fn main() {
    println!("Hello, AOC!");

    let ans1 = day_1::sol::aoc(1, 1);
    let ans2 = day_1::sol::aoc(1, 2);
    let ans3 = day_2::sol::aoc(2, 1);
    let ans4 = day_2::sol::aoc(2, 2);
    println!("day1: part1: {}", ans1);
    println!("day1: part2: {}", ans2);
    println!("day2: part1: {}", ans3);
    println!("day2: part2: {}", ans4);
}
