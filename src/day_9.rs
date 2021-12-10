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
        let height_map = content.split('\n').fold(vec![], |mut lines, line_str| {
            lines.push(
                line_str
                    .split("")
                    .filter(|val| val.parse::<u8>().is_ok())
                    .fold(vec![], |mut values, value_str| {
                        values.push((value_str.parse::<u64>().unwrap(), false));
                        values
                    }),
            );
            lines
        });
        let mut danger = 0;

        for i in 0..height_map.len() {
            for j in 0..height_map[0].len() {
                match get_neighbours_values(&height_map, i, j) {
                    (t, r, b, l) => {
                        if height_map[i][j].0 < t
                            && height_map[i][j].0 < r
                            && height_map[i][j].0 < b
                            && height_map[i][j].0 < l
                        {
                            danger += height_map[i][j].0 + 1;
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }
        danger
    }

    fn part2(content: String) -> u64 {
        let mut height_map = content.split('\n').fold(vec![], |mut lines, line_str| {
            lines.push(
                line_str
                    .split("")
                    .filter(|val| val.parse::<u8>().is_ok())
                    .fold(vec![], |mut values, value_str| {
                        values.push((value_str.parse::<u64>().unwrap(), false));
                        values
                    }),
            );
            lines
        });
        let mut dips = vec![];

        for i in 0..height_map.len() {
            for j in 0..height_map[0].len() {
                match get_neighbours_values(&height_map, i, j) {
                    (t, r, b, l) => {
                        if height_map[i][j].0 < t
                            && height_map[i][j].0 < r
                            && height_map[i][j].0 < b
                            && height_map[i][j].0 < l
                        {
                            dips.push((i, j))
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }
        let mut basins = vec![];
        for dip in dips.iter() {
            let s = flood_fill(&mut height_map, dip.0, dip.1, 0);
            basins.push(s);
        }
        basins.sort();
        let len = basins.len();
        basins[len - 1] * basins[len - 2] * basins[len - 3]
    }

    fn flood_fill(terrain: &mut Vec<Vec<(u64, bool)>>, x: usize, y: usize, count: u64) -> u64 {
        if terrain[x][y].1 {
            return count;
        }
        let mut area = 1;
        terrain[x][y].1 = true;
        let (t, r, b, l) = get_neighbours_coordinates(terrain, x, y);
        if let Some((tx, ty)) = t {
            let a = flood_fill(terrain, tx, ty, count);
            area += a;
        };
        if let Some((rx, ry)) = r {
            let a = flood_fill(terrain, rx, ry, count);
            area += a;
        };
        if let Some((bx, by)) = b {
            let a = flood_fill(terrain, bx, by, count);
            area += a;
        };
        if let Some((lx, ly)) = l {
            let a = flood_fill(terrain, lx, ly, count);
            area += a;
        };
        area
    }

    fn get_neighbours_values(
        terrain: &[Vec<(u64, bool)>],
        x: usize,
        y: usize,
    ) -> (u64, u64, u64, u64) {
        let left = if y > 0 { terrain[x][y - 1].0 } else { 9 };

        let right = if y < terrain[0].len() - 1 {
            terrain[x][y + 1].0
        } else {
            9
        };

        let top = if x > 0 { terrain[x - 1][y].0 } else { 9 };

        let bottom = if x < terrain.len() - 1 {
            terrain[x + 1][y].0
        } else {
            9
        };

        (top, right, bottom, left)
    }

    fn get_neighbours_coordinates(
        terrain: &[Vec<(u64, bool)>],
        x: usize,
        y: usize,
    ) -> (
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
    ) {
        let left = if y > 0 && terrain[x][y - 1].0 != 9 {
            Some((x, y - 1))
        } else {
            None
        };

        let right = if y < terrain[0].len() - 1 && terrain[x][y + 1].0 != 9 {
            Some((x, y + 1))
        } else {
            None
        };

        let top = if x > 0 && terrain[x - 1][y].0 != 9 {
            Some((x - 1, y))
        } else {
            None
        };

        let bottom = if x < terrain.len() - 1 && terrain[x + 1][y].0 != 9 {
            Some((x + 1, y))
        } else {
            None
        };

        (top, right, bottom, left)
    }
}
