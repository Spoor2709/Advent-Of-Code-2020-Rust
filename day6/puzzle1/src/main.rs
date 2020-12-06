use std::fs;

fn main() {
    let input = fs::read_to_string("../answers.input")
        .expect("Something went wrong reading the file"); 
    let group_answers: Vec<&str> = input.split("\n\n").into_iter().collect();
    let group_answers_counts: Vec<u32> = get_group_answers(group_answers);
    let sum: u32 = group_answers_counts.iter().sum();
    println!("{:#?}", sum)
}

fn get_group_answers(group_answers: Vec<&str>) -> Vec<u32>{
    let mut flight_answer_count: Vec<u32> = Vec::new();
    for group in group_answers.into_iter(){
        let mut unique: Vec<char>= Vec::new();
        let answers: Vec<&str> = group.split("\n").into_iter().collect();
        for answer in answers.into_iter(){
            for charecter in answer.chars(){
                if unique.contains(&charecter){
                    continue;
                }else{
                    unique.push(charecter)
                }
            };
        }
        flight_answer_count.push(unique.len() as u32);
    }
    flight_answer_count
}