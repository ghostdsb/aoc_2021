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
        let raw_signal_logs = content.split('\n').collect::<Vec<&str>>();
        let signal_log = raw_signal_logs.iter().fold(Vec::new(), |mut sl, log| {
            let log_entry = log.split('|').collect::<Vec<&str>>();
            let input_signals = log_entry[0].split_whitespace().collect::<Vec<&str>>();
            let output_signals = log_entry[1].split_whitespace().collect::<Vec<&str>>();
            sl.push((input_signals, output_signals));
            sl
        });

        let mut unique_count = 0;
        for log in signal_log{
            unique_count +=  log.1.iter().filter(|l| (l.len()==2 || l.len() == 3 || l.len()==4 || l.len() == 7)).count();
        }
        unique_count as u64
    }

    fn part2(content: String) -> u64 {
        0
    }
}
