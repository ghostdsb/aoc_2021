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
    struct Board {
        lines: Vec<Vec<(u8, bool)>>,
        last_num_called: u8,
    }

    impl Board {
        fn new(board_data: &str) -> Self {
            let lines = board_data.split('\n').fold(vec![], |mut lines, line_str| {
                lines.push(
                    line_str
                        .split_whitespace()
                        .fold(vec![], |mut values, value_str| {
                            values.push((value_str.parse::<u8>().unwrap(), false));
                            values
                        }),
                );
                lines
            });

            Self {
                lines,
                last_num_called: 0,
            }
        }

        fn strike_number(&mut self, number: u8) {
            self.last_num_called = number;
            let col_count = self.lines[0].len();
            let row_count = self.lines.len();

            for i in 0..col_count {
                for j in 0..row_count {
                    if self.lines[i][j].0 == number {
                        self.lines[i][j].1 = true;
                        break;
                    }
                }
            }
        }

        fn is_gameover(&mut self) -> bool {
            let col_count = self.lines[0].len();
            let row_count = self.lines.len();

            for i in 0..row_count {
                let mut rows_done = 0;
                for j in 0..col_count {
                    if self.lines[i][j].1 {
                        rows_done += 1;
                    }
                }
                if rows_done == row_count {
                    return true;
                }
            }
            for i in 0..col_count {
                let mut cols_done = 0;
                for j in 0..row_count {
                    if self.lines[j][i].1 {
                        cols_done += 1;
                    }
                }
                if cols_done == col_count {
                    return true;
                }
            }
            false
        }

        fn score(&self) -> u64 {
            let col_count = self.lines[0].len();
            let row_count = self.lines.len();
            let mut unmarked_sum = 0;
            for i in 0..col_count {
                for j in 0..row_count {
                    if !self.lines[i][j].1 {
                        unmarked_sum += self.lines[i][j].0 as u64;
                    }
                }
            }
            unmarked_sum * self.last_num_called as u64
        }
    }

    struct Game {
        boards: Vec<Board>,
        inputs: Vec<u8>,
    }

    impl Game {
        fn new(content: &[&str]) -> Self {
            let inputs = content[0]
                .split(',')
                .map(|d| d.parse::<u8>().unwrap())
                .collect();
            let mut boards: Vec<Board> = vec![];
            for board_data in content.iter().skip(1) {
                let board = Board::new(board_data);
                boards.push(board);
            }
            Self { inputs, boards }
        }
    }

    fn part1(content: String) -> u64 {
        let game_log = content.split("\n\n").collect::<Vec<&str>>();

        let mut game = Game::new(&game_log);

        for input in game.inputs {
            for board in game.boards.iter_mut() {
                if !board.is_gameover() {
                    board.strike_number(input);
                    if board.is_gameover() {
                        return board.score();
                    }
                }
            }
        }
        unreachable!();
    }
    fn part2(content: String) -> u64 {
        let game_log = content.split("\n\n").collect::<Vec<&str>>();

        let mut game = Game::new(&game_log);

        let total_boards = game.boards.len();

        let mut total_wins = 0;
        for input in game.inputs {
            for board in game.boards.iter_mut() {
                if !board.is_gameover() {
                    board.strike_number(input);
                    if board.is_gameover() {
                        total_wins += 1;
                    }
                    if total_wins == total_boards {
                        return board.score();
                    }
                }
            }
        }
        unreachable!();
    }
}
