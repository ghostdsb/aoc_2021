#[allow(unused)]
pub mod sol {

    use std::collections::{HashMap, HashSet};
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
        for log in signal_log {
            unique_count += log
                .1
                .iter()
                .filter(|l| (l.len() == 2 || l.len() == 3 || l.len() == 4 || l.len() == 7))
                .count();
        }
        unique_count as u64
    }

    fn part2(content: String) -> u64 {
        let lines_count_to_lines: HashMap<usize, Vec<&str>> = HashMap::new();
        let mut number_to_pattern: HashMap<usize, &str> = HashMap::new();

        let raw_signal_logs = content.split('\n').collect::<Vec<&str>>();
        let signal_log = raw_signal_logs.iter().fold(Vec::new(), |mut sl, log| {
            let log_entry = log.split('|').collect::<Vec<&str>>();
            let input_signals = log_entry[0].split_whitespace().collect::<Vec<&str>>();
            let output_signals = log_entry[1].split_whitespace().collect::<Vec<&str>>();
            sl.push((input_signals, output_signals));
            sl
        });

        let mut unique_count = 0;
        let mut decode_sum = 0;
        for log in signal_log {
            let pattern_map = log.0.iter().fold(HashMap::new(), |mut hm, pattern| {
                let key = pattern.len();
                let pattern_vec = hm.entry(key).or_insert_with(Vec::new);
                pattern_vec.push(pattern);
                hm
            });

            let one = *pattern_map.get(&2_usize).unwrap()[0];
            let four = *pattern_map.get(&4_usize).unwrap()[0];
            let seven = *pattern_map.get(&3_usize).unwrap()[0];
            let eight = *pattern_map.get(&7_usize).unwrap()[0];

            // _3 + _1 = _3
            let three: &str = &pattern_map
                .get(&5_usize)
                .unwrap()
                .iter()
                .filter(|pat| equal_str(&add_str(one, pat), ***pat))
                .map(|pat| **pat)
                .collect::<String>()[..];

            // _4 + _2 = _8
            let two: &str = &pattern_map
                .get(&5_usize)
                .unwrap()
                .iter()
                .filter(|pat| equal_str(&add_str(four, pat), eight))
                .map(|pat| **pat)
                .collect::<String>()[..];

            // _6 + _1 = _8
            let six: &str = &pattern_map
                .get(&6_usize)
                .unwrap()
                .iter()
                .filter(|pat| equal_str(&add_str(one, pat), eight))
                .map(|pat| **pat)
                .collect::<String>()[..];

            // len == 5 & not 2,3
            let five: &str = &pattern_map
                .get(&5_usize)
                .unwrap()
                .iter()
                .filter(|pat| !equal_str(pat, two) && !equal_str(pat, three))
                .map(|pat| **pat)
                .collect::<String>()[..];

            // _5 + _0 = _8
            let zero: &str = &pattern_map
                .get(&6_usize)
                .unwrap()
                .iter()
                .filter(|pat| equal_str(&add_str(five, pat), eight))
                .map(|pat| **pat)
                .collect::<String>()[..];

            // _3 + _9 = _9
            let nine: &str = &pattern_map
                .get(&6_usize)
                .unwrap()
                .iter()
                .filter(|pat| equal_str(&add_str(three, pat), pat))
                .map(|pat| **pat)
                .collect::<String>()[..];

            let mut char_lookup = [
                (0, zero),
                (1, one),
                (2, two),
                (3, three),
                (4, four),
                (5, five),
                (6, six),
                (7, seven),
                (8, eight),
                (9, nine),
            ];

            let mut decode_vec = vec![];

            for output in log.1.iter() {
                for lkup in char_lookup.iter() {
                    if equal_str(output, lkup.1) {
                        // println!("---{}", lkup.0);
                        decode_vec.push(lkup.0);
                    }
                }
            }
            let num = num_from_vec(decode_vec);
            // println!("decoded: {}", num);
            decode_sum += num;

            // println!("{:?}", one);
            // println!("{:?}", two);
            // println!("{:?}", three);
            // println!("{:?}", four);
            // println!("{:?}", five);
            // println!("{:?}", six);
            // println!("{:?}", seven);
            // println!("{:?}", eight);
            // println!("{:?}", nine);
            // println!("{:?}", zero);
            // println!("");
        }
        // println!("decoded: {}", decode_sum);
        decode_sum
    }

    fn num_from_vec(num_vec: Vec<u64>) -> u64 {
        num_vec
            .iter()
            .rev()
            .enumerate()
            .fold(0, |mut num, (i, dig)| {
                num + (dig * 10_u64.pow(i as u32)) as u64
            })
    }

    fn equal_str(pat_1: &str, pat_2: &str) -> bool {
        arrange_str(pat_1) == arrange_str(pat_2)
    }

    fn add_str<'a>(pat_1: &'a str, pat_2: &'a str) -> String {
        let mut charset_a = HashSet::new();
        let mut charset_b = HashSet::new();
        for chr in pat_1.chars() {
            charset_a.insert(chr);
        }
        for chr in pat_2.chars() {
            charset_b.insert(chr);
        }
        let mut s = charset_a.union(&charset_b).collect::<Vec<_>>();
        s.sort_unstable();
        s.iter().fold(String::new(), |mut ad, &chr| {
            ad.push(*chr);
            ad
        })
    }

    fn arrange_str(pattern: &str) -> String {
        let mut pstr = pattern.bytes().collect::<Vec<u8>>();
        pstr.sort_unstable();
        pstr.iter().map(|b| *b as char).collect::<String>()
    }
}
