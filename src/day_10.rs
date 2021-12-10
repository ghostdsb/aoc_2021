#[allow(unused)]
pub mod sol {

    use std::fs;
    use std::collections::HashMap;

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

    fn get_error_score(character: char) -> u64 {
        match character {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!()
        }
    }

    fn get_completion_score(character: char) -> u64 {
        match character {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!()
        }
    }

    fn is_opening(character: char) -> bool {
        character == '(' ||
        character == '[' ||
        character == '{' ||
        character == '<' 
    }

    fn get_pair(character: char) -> char {
        match character {
            ')' => '(',
            ']' => '[',
            '}' => '}',
            '>' => '<',
            '(' => ')', 
            '[' => ']', 
            '{' => '}', 
            '<' => '>', 
            _ => unreachable!()
        }
    }

    fn part1(content: String) -> u64 {

        let logs = content.split('\n').fold(vec![], |mut l, log| {l.push(log); l});
        
        let mut score = 0;
        for log in logs.iter() {
            let mut char_stack = vec![];
            for chr in log.chars(){
                if is_opening(chr) {
                    char_stack.push(chr);
                }else{
                    match char_stack.last() {
                        Some(opening) => {
                            if get_pair(*opening) == chr {
                                char_stack.pop();
                            }else{
                                score += get_error_score(chr);
                                break;
                            }
                        },
                        None => {
                            score += get_error_score(chr);
                            break;
                        },
                    }
                }
            }
        }
        score
    }

    fn part2(content: String) -> u64 {
        let logs = content.split('\n').fold(vec![], |mut l, log| {l.push(log); l});
        
        let mut completion_scores = vec![];

        for (i,log) in logs.iter().enumerate() {
            let mut score = 0;
            let mut char_stack = vec![];
            let mut is_valid = true;
            for chr in log.chars(){
                if is_opening(chr) {
                    char_stack.push(chr);
                }else{
                    match char_stack.last() {
                        Some(opening) => {
                            if get_pair(*opening) == chr {
                                char_stack.pop();
                            }else{
                                is_valid = false;
                                break;
                            }
                        },
                        None => {
                            is_valid = false;
                            break;
                        },
                    }
                }
            }
            if is_valid{
                for stack in char_stack.iter().rev(){
                    score = score * 5 + get_completion_score(get_pair(*stack)); 
                }
                completion_scores.push(score);
            }
        }
        let len = completion_scores.len();
        completion_scores.sort_unstable();
        completion_scores[len/2]
    }
}
