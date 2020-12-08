use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("../bags_test.input")
        .expect("Something went wrong reading the file"); 
    let bags: Vec<&str> = input.lines().collect();
    let answer: u128 = get_num_shiny_gold(bags);
    // let answer = 
    println!("{:#?}", answer)
}

fn get_num_shiny_gold(bags: Vec<&str>) -> u128 {
    let mut contains: Vec<&str> = Vec::new(); 
    let mut bag_map: HashMap<&str, &str> = HashMap::new();

    for bag_rule in bags.into_iter(){
        let bag_rule_split = bag_rule.split("contain").collect::<Vec<&str>>();
        let parent: &str = bag_rule_split[0].trim_end();
        let children: &str = bag_rule_split[1].trim_start();
        bag_map.insert(
            parent,
            children,
        );
    }
    for (key, val) in &bag_map{
        for individual_val in val.split(",").into_iter(){
            if key.contains("shiny gold"){
                let new_val = if individual_val.contains("1") && individual_val.contains(".") || individual_val.contains("bags") && !individual_val.contains("."){
                    &individual_val[..individual_val.len()-4]
                } else if individual_val.contains("1"){
                    &individual_val[..individual_val.len()-3]
                }else {
                    &individual_val[..individual_val.len()-5]
                };
                contains.push(new_val);
            }
        }
    }

    dbg!(&bag_map);
    let len_of_con: usize = contains.len();
    return recursive_bag_find(bag_map, contains, len_of_con);
}

fn recursive_bag_find<'a>(bag_map: HashMap<&'a str, &'a str>, mut contains: Vec<&'a str>, con_len: usize) -> u128{
    // dbg!(&bag_map.len());
    for (key, val) in &bag_map{
        for individual_val in val.split(",").into_iter(){
            let new_val = if individual_val.contains("1") && individual_val.contains(".") || individual_val.contains("bags") && !individual_val.contains("."){
                &individual_val[..individual_val.len()-4]
            } else if individual_val.contains("1"){
                &individual_val[..individual_val.len()-3]
            }else {
                &individual_val[..individual_val.len()-5]
            };
            dbg!(new_val);
            let mut new_contains: Vec<&str> = Vec::new();
            for containing_val in &contains {
                if val.contains(containing_val) && !contains.contains(&new_val) && !new_contains.contains(&new_val) {
                    new_contains.push(new_val);
                }else{
                    continue;
                }
            }
            contains.append(&mut new_contains);
        }
    }
    let len_of_con: usize = contains.len();
    // dbg!(con_len);
    // dbg!(len_of_con);
    if con_len != contains.len(){
        recursive_bag_find(bag_map, contains, len_of_con)
    }else{
        // dbg!(contains);
        let mut collector: u128 = 1;
        for line in contains.into_iter(){
            collector = collector * line.split_whitespace().nth(0).unwrap().parse::<u128>().unwrap();
            dbg!(collector, line);
        }
        return collector;
    }
}
