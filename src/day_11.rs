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

    #[derive(Debug)]
    struct Octopus{
        energy: u8,
        flashed: bool
    }

    impl Octopus{
        fn new(energy: u8) -> Self{
            Self{energy, flashed: false}
        }

        fn energize(&mut self){
            if self.energy < 9 {
                self.energy += 1;
            }else{
                self.energy = 0;
                self.flashed = true;
            }
        }
    }

    #[derive(Debug)]
    struct OctopusGrid{
        grid : Vec<Vec<Octopus>>
    }

    impl OctopusGrid{
        fn new(content: String) -> Self{
            let octopus_grid = content.split('\n').fold(vec![], |mut lines, line_str| {
                lines.push(
                    line_str
                        .split("")
                        .filter(|val| val.parse::<u8>().is_ok())
                        .fold(vec![], |mut values, value_str| {
                            values.push(Octopus::new(value_str.parse::<u8>().unwrap()));
                            values
                        }),
                );
                lines
            });

            Self{grid: octopus_grid}
        }

        fn tick(&mut self) -> Vec<(usize, usize)>{
            let mut glow_coordinates = vec![];
            for i in 0..self.grid.len(){
                for j in 0..self.grid[0].len(){
                    self.grid[i][j].energize();
                    if self.grid[i][j].flashed {
                        glow_coordinates.push((i,j));
                    }

                }
            }
            glow_coordinates
        }

        fn glow(&mut self, glow_coordinates: Vec<(usize, usize)>) -> u64{
            // let mut glow_count = 0;
            // for i in 0..self.grid.len(){
            //     for j in 0..self.grid[0].len(){
            //         if self.grid[i][j].0 == 0 {
            //             glow_count += 1;
            //         }
            //     }
            // }
            // glow_count
            for (i,j) in glow_coordinates {

            } 
            0
        }

        fn flash_counts(&self) -> u64 {
            let mut flash_count = 0;
            for i in 0..self.grid.len(){
                for j in 0..self.grid[0].len(){
                    if self.grid[i][j].energy == 0{
                        flash_count +=1;
                    };
                }
            }
            flash_count
        }
    }

    fn part1(content: String) -> u64 {
        let mut energy_grid = OctopusGrid::new(content);
        println!("{:?}", energy_grid);
        energy_grid.tick();
        println!("{:?}", energy_grid);
        energy_grid.tick();
        println!("{:?}", energy_grid);
        0
    }

    fn part2(content: String) -> u64 {
        0
    }

    fn get_neighbours_coordinates(
        grid: &[Vec<(u64, bool)>],
        x: usize,
        y: usize,
    ) -> (
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
    ) {
        let left = if y > 0 {
            Some((x, y - 1))
        } else {
            None
        };

        
        let right = if y < grid[0].len() - 1 {
            Some((x, y + 1))
        } else {
            None
        };

        let top = if x > 0 {
            Some((x - 1, y))
        } else {
            None
        };

        let bottom = if x < grid.len() - 1 {
            Some((x + 1, y))
        } else {
            None
        };

        let top_left = if top.is_some() && left.is_some(){
            Some((x-1, y-1))
        }else{
            None
        };
        
        let top_right = if top.is_some() && right.is_some(){
            Some((x-1, y+1))
        }else{
            None
        };
        
        let bottom_right = if bottom.is_some() && right.is_some(){
            Some((x+1, y+1))
        }else{
            None
        };

        let bottom_left = if bottom.is_some() && left.is_some(){
            Some((x+1, y-1))
        }else{
            None
        };

        (top, top_right, right, bottom_right, bottom, bottom_left, left, top_left)
    }
}
