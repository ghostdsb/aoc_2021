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
        let mut positions: Vec<u64> = content
            .split(',')
            .map(|p| p.parse::<u64>().unwrap())
            .collect();
        positions.sort_unstable();
        let mid_index = positions.len() / 2;
        adjustment_sum(&positions, &positions[mid_index])
    }

    fn adjustment_sum(positions: &[u64], target_pos: &u64) -> u64 {
        positions
            .iter()
            .map(|p| (*p as i64 - *target_pos as i64).abs() as u64)
            .sum()
    }

    fn part2(content: String) -> u64 {
        let mut positions: Vec<u64> = content
            .split(',')
            .map(|p| p.parse::<u64>().unwrap())
            .collect();
        positions.sort_unstable();
        let mut min_cost = u64::MAX;
        let upper_limit = *positions.iter().max().unwrap();
        for target in (0..=upper_limit) {
            let cost = adjustment_sum_2(&positions, &target);
            if cost < min_cost {
                min_cost = cost;
            }
        }
        min_cost
    }

    fn adjustment_sum_2(positions: &[u64], target_pos: &u64) -> u64 {
        let n_sum = |num: u64| {
            if num % 2 == 0 {
                num / 2 * (num + 1)
            } else {
                num * (num + 1) / 2
            }
        };

        positions
            .iter()
            .map(|p| n_sum((*p as i64 - *target_pos as i64).abs() as u64))
            .sum()
    }
}
