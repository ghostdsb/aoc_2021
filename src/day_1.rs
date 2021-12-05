#[allow(unused)]
pub mod sol {

    use std::fs;

    pub fn aoc(day: u8, part: u8) -> u64 {
        let input_path = format!("./input/day{}.txt", day);
        match (fs::read_to_string(&input_path), part) {
            (Ok(content), 1) => part1(content),
            (Ok(content), 2) => part2(content),
            (_, _) => {
                println!("something wrong");
                0
            }
        }
    }

    fn part1(content: String) -> u64 {
        let mut ans = 0;
        let depths = content
            .split('\n')
            .map(|d| d.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for i in 1..depths.len() {
            if depths[i] > depths[i - 1] {
                ans += 1;
            }
        }
        ans
    }

    fn part2(content: String) -> u64 {
        let mut ans = 0;
        let depths = content
            .split('\n')
            .map(|d| d.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for i in 1..depths.len() - 2 {
            if depths[i + 2] > depths[i - 1] {
                ans += 1;
            }
        }
        ans
    }
}
