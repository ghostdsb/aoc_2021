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
        let mut grid = content.split('\n').fold(vec![], |mut lines, line_str| {
            lines.push(
                line_str
                    .split("")
                    .filter(|val| val.parse::<u8>().is_ok())
                    .fold(vec![], |mut values, value_str| {
                        values.push(value_str.parse::<u64>().unwrap());
                        values
                    }),
            );
            lines
        });
        let first_square = grid[0][0];
        grid[0][0] = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let min_path_value = get_min_path_neighbour_value(&grid, i, j);
                grid[i][j] += min_path_value;
            }
        }
        // for i in 0..grid.len() {
        //     for j in 0..grid[0].len() {
        //         print!("{:2} ",grid[i][j]);
        //     }
        //     println!()
        // }
        grid[grid.len()-1][grid[0].len()-1] - first_square
    }

    fn part2(content: String) -> u64 {
        0
    }

    fn get_min_path_neighbour_value(
        grid: &[Vec<u64>],
        x: usize,
        y: usize,
    ) -> u64 {
        let left = if y > 0 { grid[x][y - 1] } else { u64::MAX };
        let top = if x > 0 { grid[x - 1][y] } else { u64::MAX };
        if left == top && left == u64::MAX{
            return 0
        }else if left == top {
            return left
        }else if left < top {
            return left
        }else{
            return top
        }
    }

    fn get_cave_value(
        grid: &[Vec<u64>], 
        x: usize,
        y: usize ) -> u64 {
            0
        }
}
