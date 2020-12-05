use std::fs;
fn main() {
    let input = fs::read_to_string("../boarding_pass.input")
        .expect("Something went wrong reading the file"); 
    let boarding_passes: Vec<&str> = input.split("\n").into_iter().collect();
    let seat_ids = get_seat_ids(boarding_passes);
    println!("{:#?}", seat_ids.iter().max())
}

fn get_seat_ids(boarding_passes: Vec<&str>) -> Vec<u32>{
    let mut seat_ids: Vec<u32> = Vec::new();
    for pass in boarding_passes.into_iter(){
        let row_string = &pass[..7];
        let col_string = &pass[7..];
        let pass_row = get_row(&row_string);
        let pass_col = get_col(&col_string);
        let seat_id = pass_row * 8 + pass_col;
        seat_ids.push(seat_id)
    }
    seat_ids
}

fn get_row(row_binary: &str) -> u32{
    return reccursive_find(row_binary, 0, 127, 'F')
}
fn get_col(col_binary: &str) -> u32{
    return reccursive_find(col_binary, 0, 7, 'R')
}

fn reccursive_find(binary: &str, section_start: u32, section_end: u32, lower_char: char) -> u32 {
    if section_end - section_start == 1{
        if binary.contains(lower_char) {
            return section_start;
        }else{
            return section_end;
        }
    }
    let section_mid = (section_end + section_start) / 2;
    let mut new_section_start = 0;
    let mut new_section_end = 0;
    let new_binary = &binary[1..];
    if binary.chars().nth(0).unwrap() == lower_char {
        new_section_start = section_start;
        new_section_end = section_mid;
    } else {
        new_section_start = section_mid;
        new_section_end = section_end;
    }
    return reccursive_find(&new_binary, new_section_start, new_section_end, lower_char)
}