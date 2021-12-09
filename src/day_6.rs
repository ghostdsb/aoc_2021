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

    #[derive(Clone)]
    struct School {
        lantern_fishes: Vec<Lanternfish>,
    }

    impl School {
        fn new() -> Self {
            Self {
                lantern_fishes: Vec::new(),
            }
        }

        fn add_fish(&mut self, fish: Lanternfish) {
            self.lantern_fishes.push(fish)
        }

        fn count(&self) -> u64 {
            self.lantern_fishes.len() as u64
        }
    }

    #[derive(Clone, Copy)]
    struct Lanternfish {
        age: u8,
    }

    impl Lanternfish {
        fn new(age: u8) -> Self {
            Self { age }
        }

        fn tick(&mut self) -> Option<Self> {
            if self.age > 0 {
                self.age -= 1;
                None
            } else {
                self.age = 6;
                Some(Lanternfish::new(8))
            }
        }
    }

    fn part1(content: String) -> u64 {
        let mut school = content.split(',').fold(School::new(), |mut sch, age_str| {
            let fish = Lanternfish::new(age_str.parse::<u8>().unwrap());
            sch.add_fish(fish);
            sch
        });

        for day in 0..80 {
            let mut new_fishes = vec![];

            for fish in school.lantern_fishes.iter_mut() {
                if let Some(f) = fish.tick() {
                    new_fishes.push(f)
                }
            }

            for fish in new_fishes {
                school.add_fish(fish);
            }
        }
        school.count()
    }

    fn part2(content: String) -> u64 {
        let mut fish_count_on = content
            .split(',')
            .fold([0; 9], |mut fish_count_on, age_str| {
                let age = age_str.parse::<usize>().unwrap();
                fish_count_on[age] += 1;
                fish_count_on
            });

        for _ in 0..256 {
            fish_count_on.rotate_left(1);
            fish_count_on[6] += fish_count_on[8];
        }

        fish_count_on.iter().sum()
    }
}
