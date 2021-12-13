#[allow(unused)]
pub mod sol {

    use std::fs;
    use std::collections::HashSet;

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

    struct Cave{
        is_big: bool,
        connections_with: HashSet<String>
    }

    impl Cave{
        fn new(name: &str) -> Self{
            Self{
                is_big: false,
                connections_with: HashSet::new()
            }
        }
    }

    fn part1(content: String) -> u64 {
        0
    }

    fn part2(content: String) -> u64 {
        0
    }
}
