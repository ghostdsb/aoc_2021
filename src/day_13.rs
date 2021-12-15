/**

  ██  ██   ██    ██ ████ ████ █  █ █  █
   █ █  █ █  █    █ █    █    █ █  █  █
   █ █    █  █    █ ███  ███  ██   █  █
   █ █ ██ ████    █ █    █    █ █  █  █
█  █ █  █ █  █ █  █ █    █    █ █  █  █
 ██   ███ █  █  ██  ████ █    █  █  ██ 

 */


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

    #[derive(Debug)]
    struct Paper{
        points: Vec<(i64, i64)>,
        folds: Vec<Line>
    }

    impl Paper{
        fn new(points_data: &str, folds_data: &str) -> Self{

            let points = points_data.lines().fold(vec![], |mut coords, data| {
                let coord: Vec<i64> = data.split(',').map(|c| c.parse::<i64>().unwrap()).collect();
                coords.push((coord[0], coord[1]));
                coords
            });

            let folds = folds_data.lines().fold(vec![], |mut lines, line_str| {
                let line = Line::new(line_str);
                lines.push(line);
                lines
            });

            Self{
                points, folds
            }
        }

        fn fold(&mut self, index: usize) -> Self {
            let fold = self.folds.iter().nth(index).unwrap();
            let value = fold.value;
            let points = self.points.iter_mut().map(|mut p| {
                p = match fold.axis{
                    Axis::Y => {
                        if value < p.1 {
                            p.1 = 2 * value - p.1;
                        }
                        p
                    },
                    Axis::X => {
                        if value < p.0 {
                            p.0 = 2 * value - p.0;
                        }
                        p
                    },
                };
                *p
            }).collect();

            Self{points, folds: self.folds.clone()}
        }

        fn count_points(&self) -> usize {
            let mut points = HashSet::new();
            for point in self.points.iter(){
                points.insert(point);
            }
            points.len()
        }

        fn draw(&self) {
            let mut max_x = 0;
            let mut max_y = 0;
            let mut point_set = HashSet::new();
            for point in self.points.iter(){
                if point.0 > max_x {
                    max_x = point.0;
                }
                if point.1 > max_y {
                    max_y = point.1
                }
                point_set.insert(point);
            }

            for y in 0..=max_y{
                for x in 0..=max_x{
                    match point_set.contains(&(x,y)) {
                        true => print!("█"),
                        false => print!(" "),
                    }
                }
                println!()
            }

        }
    }

    #[derive(Debug, Clone)]
    enum Axis{X,Y}

    #[derive(Debug, Clone)]
    struct Line{
        axis: Axis,
        value: i64 
    }

    impl Line{
        fn new(line_str: &str) -> Self{
            let line_data: &str = &line_str.split_whitespace().collect::<Vec<&str>>()[2];
            let line = line_data.split('=').collect::<Vec<&str>>();
            let axis = if line[0] == "x" { Axis::X} else {Axis::Y}; 
            let value = line[1].parse::<i64>().unwrap();
            Self{
                axis, value
            }
        }
    }

    fn part1(content: String) -> u64 {
        let manual = content.split("\n\n").collect::<Vec<&str>>();

        let point_coords = manual[0];
        let fold_lines = manual[1];

        let mut paper = Paper::new(point_coords, fold_lines);
        paper.fold(0);
        let point_count = paper.count_points();
        point_count as u64
    }

    fn part2(content: String) -> u64 {
        let manual = content.split("\n\n").collect::<Vec<&str>>();

        let point_coords = manual[0];
        let fold_lines = manual[1];

        let mut paper = Paper::new(point_coords, fold_lines);
        for i in 0..paper.folds.len(){
            paper.fold(i);
        }
        paper.draw();
        0
    }
}
