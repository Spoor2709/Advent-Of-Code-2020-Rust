use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("../bags.input")
        .expect("Something went wrong reading the file"); 
    let bags: Vec<&str> = input.lines().collect();
    let answer: usize = get_num_shiny_gold(bags);
    // let answer = 
    println!("{:#?}", answer)
}

fn get_num_shiny_gold(bags: Vec<&str>) -> usize {
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
        if val.contains("shiny gold"){
            contains.push(&key[..key.len()-4]);
        }
    }

    dbg!(&bag_map);
    let len_of_con: usize = contains.len();
    return recursive_bag_find(bag_map, contains, len_of_con);
}

fn recursive_bag_find<'a>(bag_map: HashMap<&'a str, &'a str>, mut contains: Vec<&'a str>, con_len: usize) -> usize{
    // dbg!(&bag_map.len());
    for (key, val) in &bag_map{
        let new_key = &key[..key.len()-4];
        let mut new_contains: Vec<&str> = Vec::new();
        for containing_key in &contains {
            if val.contains(containing_key) && !contains.contains(&new_key) && !new_contains.contains(&new_key) {
                new_contains.push(new_key);
            }else{
                continue;
            }
        }
        contains.append(&mut new_contains);
    }
    let len_of_con: usize = contains.len();
    dbg!(con_len);
    dbg!(len_of_con);
    if con_len != contains.len(){
        recursive_bag_find(bag_map, contains, len_of_con)
    }else{
        return len_of_con;
    }
}
