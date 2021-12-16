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

    fn part1(content: String) -> u64 {
       let polymer_data = content.split("\n\n").collect::<Vec<&str>>();
       let mut polymer_map: HashMap<String, (u8, u64)> = HashMap::new();
       let initial_template = polymer_data[0].chars().collect::<Vec<char>>();

       for chain in initial_template.windows(2) {
        let poly = chain.iter().collect();
        let counter = polymer_map.entry(poly).or_insert((0,0));
        counter.1 += 1;
       }

       let chains = polymer_data[1].lines().fold(vec![], |mut chain, link_data| {
        let link_instructions = link_data.split(" -> ").collect::<Vec<&str>>();
        chain.push(link_instructions);
        chain
       } );
    //    println!("{:?}", polymer_map);
       println!("{:?}", chains);

    //    for i in 0..10{
    //        for chain in chains.iter(){
    //             let strand = chain[0];
    //             let pol_data = polymer_map.get_mut(strand);
    //             if let Some((gen,count)) = pol_data{
    //                 if *gen == i {
    //                     if *count == 1{
    //                         polymer_map.remove(strand);
    //                     }else{
    //                         polymer_map.insert(strand.to_string(), (*gen, *count-1));
    //                         let new_entries = get_new_polymers(strand, chain[1]);
    //                         polymer_map.insert(strand.to_string(), (*gen, *count-1));
    //                     }
    //                 }
    //             };
    //             println!("{:?}", polymer_map);
    //        }
    //    }

       polymer_map.capacity() as u64
    }

    fn get_new_polymers(check_polymer: &str, element: &str) -> [String;2]{
        let chars = check_polymer.chars().collect::<Vec<_>>();
        let mut str_a = String::from(chars[0]);
        let mut str_b = String::from(element);
        str_a.push_str(element);
        str_b.push_str(&String::from(chars[1])[..]);
        [str_a, str_b]
    }

    fn part2(content: String) -> u64 {
        0
    }
}
