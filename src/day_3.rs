#[allow(unused)]
pub mod sol {

    use std::fs;

    pub fn aoc(day: u8, part: u8) -> u64 {
        let input_path = format!("./input/day{}.txt", day);
        match (fs::read_to_string(&input_path), part) {
            (Ok(content), 1) => part1(&content),
            (Ok(content), 2) => part2(&content),
            (_, _) => {
                println!("something wrong");
                0
            }
        }
    }

    fn part1(content: &str) -> u64 {
        let report = content.split('\n').collect::<Vec<&str>>();

        let report_bit_length = report[0].len();
        let mut gamma = vec![];

        for bit in 0..report_bit_length {
            let mut bit_counts = (0, 0);
            for bits in report.iter() {
                if bits.to_string().as_bytes()[bit] == b'0' {
                    bit_counts.0 += 1;
                } else {
                    bit_counts.1 += 1;
                }
            }
            if bit_counts.0 > bit_counts.1 {
                gamma.push(0);
            } else {
                gamma.push(1);
            }
        }
        let gc = bin_to_decimal(&gamma);
        let ec = bin_invert_decimal(&gamma);
        gc * ec
    }

    enum Rating {
        Oxygen,
        CO2,
    }

    #[derive(Debug)]
    struct Report<'a> {
        data: &'a str,
        status: bool,
    }

    impl<'a> Report<'a> {
        fn new(data: &'a str) -> Self {
            Self { data, status: true }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn stringify(&self) -> String {
            String::from(self.data)
        }

        fn update_status(&mut self, status: bool) {
            self.status = status
        }

        fn is_zero_bit(&self, bit_index: usize) -> bool {
            self.stringify().as_bytes()[bit_index] == b'0'
        }
    }

    fn part2(content: &str) -> u64 {
        let ogr = get_report(content, &Rating::Oxygen);
        let csr = get_report(content, &Rating::CO2);

        bin_to_decimal(&ogr) * bin_to_decimal(&csr)
    }

    fn get_report(content: &str, rating: &Rating) -> Vec<u8> {
        let mut reports = content
            .split('\n')
            .map(|c| Report::new(c))
            .collect::<Vec<Report>>();

        let report_bit_length = reports[0].len();

        for bit in 0..report_bit_length {
            let mut bit_counts = (0, 0);

            for report in reports.iter() {
                match (report.status, report.is_zero_bit(bit)) {
                    (true, true) => bit_counts.0 += 1,
                    (true, false) => bit_counts.1 += 1,
                    _ => {}
                }
            }

            let expected_bit_count = match (rating, bit_counts.0 > bit_counts.1) {
                (Rating::Oxygen, true) => 0,
                (Rating::Oxygen, false) => 1,
                (Rating::CO2, true) => 1,
                (Rating::CO2, false) => 0,
                _ => unreachable!(),
            };

            for report in reports.iter_mut() {
                if report.stringify().as_bytes()[bit] - b'0' != expected_bit_count {
                    report.update_status(false);
                }
            }

            let mut active_count = 0;
            for report in reports.iter_mut() {
                if report.status {
                    active_count += 1;
                }
            }

            if active_count == 1 {
                break;
            }
        }

        for report in reports.iter() {
            if report.status {
                return report
                    .stringify()
                    .as_bytes()
                    .iter()
                    .fold(vec![], |mut bts, c| {
                        bts.push(*c - b'0');
                        bts
                    });
            }
        }
        return vec![];
    }

    fn bin_invert_decimal(bins: &[u8]) -> u64 {
        let l = bins.len();
        bins.iter().enumerate().fold(0, |dec, (i, dig)| {
            dec + flip_bit(*dig) as u64 * (2_i32).pow((l - i - 1) as u32) as u64
        })
    }

    fn bin_to_decimal(bins: &[u8]) -> u64 {
        let l = bins.len();
        bins.iter().enumerate().fold(0, |dec, (i, dig)| {
            dec + *dig as u64 * (2_i32).pow((l - i - 1) as u32) as u64
        })
    }

    fn flip_bit(bit: u8) -> u8 {
        match bit {
            0 => 1,
            _ => 0,
        }
    }
}
