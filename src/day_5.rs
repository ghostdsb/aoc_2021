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

    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn new(x: u32, y: u32) -> Self {
            Self { x, y }
        }
    }

    #[derive(Debug)]
    struct LineSegment {
        a: Point,
        b: Point,
    }

    impl LineSegment {
        fn new(p1: Point, p2: Point) -> Self {
            if p1.y == p2.y {
                if p1.x < p2.x {
                    return Self { a: p1, b: p2 };
                } else {
                    return Self { a: p2, b: p1 };
                }
            } else if p1.x == p2.x {
                if p1.y < p2.y {
                    return Self { a: p1, b: p2 };
                } else {
                    return Self { a: p2, b: p1 };
                }
            } else if ((p1.x as i64 - p2.x as i64) as i64).abs()
                == ((p1.y as i64 - p2.y as i64) as i64).abs()
            {
                if p1.x < p2.x {
                    return Self { a: p1, b: p2 };
                } else {
                    return Self { a: p2, b: p1 };
                }
            }
            Self { a: p1, b: p2 }
        }

        fn is_vertical(&self) -> bool {
            self.a.x == self.b.x
        }

        fn is_horizontal(&self) -> bool {
            self.a.y == self.b.y
        }

        fn is_diagonal(&self) -> bool {
            ((self.a.x as i64 - self.b.x as i64) as i64).abs()
                == ((self.a.y as i64 - self.b.y as i64) as i64).abs()
        }

        fn slope(&self) -> i8 {
            if self.a.y < self.b.y {
                1
            } else {
                -1
            }
        }

        fn integral_points(&self) -> Vec<Point> {
            if self.is_vertical() {
                if self.a.y < self.b.y {
                    return (self.a.y..=self.b.y)
                        .map(|y| Point::new(self.a.x, y))
                        .collect();
                } else {
                    return (self.b.y..=self.a.y)
                        .map(|y| Point::new(self.a.x, y))
                        .collect();
                }
            } else if self.is_horizontal() {
                if self.a.x < self.b.x {
                    return (self.a.x..=self.b.x)
                        .map(|x| Point::new(x, self.a.y))
                        .collect();
                } else {
                    return (self.b.x..=self.a.x)
                        .map(|x| Point::new(x, self.a.y))
                        .collect();
                }
            } else if self.is_diagonal() {
                if self.slope() > 0 {
                    return (self.a.x..=self.b.x)
                        .enumerate()
                        .map(|(i, x)| Point::new(x, self.a.y + i as u32))
                        .collect();
                } else {
                    return (self.a.x..=self.b.x)
                        .enumerate()
                        .map(|(i, x)| Point::new(x, self.a.y - i as u32))
                        .collect();
                }
            }
            vec![]
        }
    }

    #[derive(Debug)]
    struct Grid {
        size: u64,
        board: Vec<Vec<u64>>,
    }

    impl Grid {
        fn new(size: u64) -> Self {
            let board = (0..size).fold(vec![], |mut row, _| {
                row.push((0..size).fold(vec![], |mut col, _| {
                    col.push(0);
                    col
                }));
                row
            });
            Self { size, board }
        }

        fn lay_vertical(&mut self, line_segment: &LineSegment) {
            if line_segment.is_vertical() {
                for point in line_segment.integral_points() {
                    self.board[point.y as usize][point.x as usize] += 1;
                }
            }
        }

        fn lay_horizontal(&mut self, line_segment: &LineSegment) {
            if line_segment.is_horizontal() {
                for point in line_segment.integral_points() {
                    self.board[point.y as usize][point.x as usize] += 1;
                }
            }
        }

        fn lay_diagonal(&mut self, line_segment: &LineSegment) {
            if line_segment.is_diagonal() {
                for point in line_segment.integral_points() {
                    self.board[point.y as usize][point.x as usize] += 1;
                }
            }
        }

        fn danger_point_count(&self) -> u64 {
            let mut danger = 0;
            for i in 0..self.size {
                for j in 0..self.size {
                    if self.board[i as usize][j as usize] >= 2 {
                        danger += 1;
                    }
                }
            }
            danger
        }
    }

    fn part1(content: String) -> u64 {
        let mut line_segments: Vec<LineSegment> = vec![];
        let line_segments =
            content
                .split('\n')
                .fold(vec![], |mut segments: Vec<LineSegment>, ls_data| {
                    let mut points: Vec<Point> = vec![];
                    for points_data in ls_data.split(" -> ").collect::<Vec<&str>>().iter() {
                        let coordinates = points_data
                            .split(',')
                            .map(|p| p.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>();
                        let point = Point::new(coordinates[0], coordinates[1]);
                        points.push(point);
                    }
                    let line_segment = LineSegment::new(points[0], points[1]);
                    segments.push(line_segment);
                    segments
                });

        let mut grid = Grid::new(1000);

        for ls in line_segments.iter() {
            grid.lay_horizontal(ls);
            grid.lay_vertical(ls);
        }

        grid.danger_point_count()
    }

    fn part2(content: String) -> u64 {
        let mut line_segments: Vec<LineSegment> = vec![];
        let line_segments =
            content
                .split('\n')
                .fold(vec![], |mut segments: Vec<LineSegment>, ls_data| {
                    let mut points: Vec<Point> = vec![];
                    for points_data in ls_data.split(" -> ").collect::<Vec<&str>>().iter() {
                        let coordinates = points_data
                            .split(',')
                            .map(|p| p.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>();
                        let point = Point::new(coordinates[0], coordinates[1]);
                        points.push(point);
                    }
                    let line_segment = LineSegment::new(points[0], points[1]);
                    segments.push(line_segment);
                    segments
                });

        let mut grid = Grid::new(1000);

        for ls in line_segments.iter() {
            grid.lay_horizontal(ls);
            grid.lay_vertical(ls);
            grid.lay_diagonal(ls);
        }

        grid.danger_point_count()
    }
}
