#[allow(unused)]
pub mod sol {

    use std::fs;

    pub fn aoc(day: u8, part: u8) -> i64 {
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

    #[derive(Debug)]
    pub struct Sub {
        hp: i64,
        depth: i64,
        aim: i64,
    }

    impl Sub {
        fn new() -> Self {
            Self {
                hp: 0,
                depth: 0,
                aim: 0,
            }
        }

        fn forward(&mut self, val: u64) {
            self.hp += val as i64;
        }

        fn forward_2(&mut self, val: u64) {
            self.hp += val as i64;
            self.depth += self.aim * (val as i64);
        }

        fn backward(&mut self, val: u64) {
            self.hp -= val as i64;
        }

        fn up(&mut self, val: u64) {
            self.depth -= val as i64;
        }

        fn down(&mut self, val: u64) {
            self.depth += val as i64;
        }

        fn up_2(&mut self, val: u64) {
            self.aim -= val as i64;
        }

        fn down_2(&mut self, val: u64) {
            self.aim += val as i64;
        }
    }

    fn part1(content: String) -> i64 {
        let mut sub = Sub::new();

        sub = content.split('\n').fold(sub, |mut s, com| {
            let command = com.split_whitespace().collect::<Vec<_>>();
            let direction = command[0];
            let distance = command[1].parse::<u64>().unwrap();
            match direction {
                "forward" => s.forward(distance),
                "up" => s.up(distance),
                "down" => s.down(distance),
                _ => (),
            };
            s
        });
        sub.hp * sub.depth
    }

    fn part2(content: String) -> i64 {
        let mut sub = Sub::new();

        sub = content.split('\n').fold(sub, |mut s, com| {
            let command = com.split_whitespace().collect::<Vec<_>>();
            let direction = command[0];
            let distance = command[1].parse::<u64>().unwrap();
            match direction {
                "forward" => s.forward_2(distance),
                "up" => s.up_2(distance),
                "down" => s.down_2(distance),
                _ => (),
            };
            s
        });
        sub.hp * sub.depth
    }
}
