use std::fs;

fn main() {
    let mut valid_passports = 0;
    let input = fs::read_to_string("../passports.input")
        .expect("Something went wrong reading the file"); 
    let passports: Vec<&str> = input.split("\n\n").collect();
    for passport in passports.into_iter(){
        if check_passport(passport) {
            valid_passports += 1;
        }
    }
    println!("{:#?}", valid_passports);
}

fn check_passport(passport: &str) -> bool{
    let mut fields_count = 0;
    let checks = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passport: Vec<&str> = passport.split_whitespace().collect();
    for check in checks.into_iter(){
        for _field in passport.iter(){
            if _field.contains(check){
                if check_field(_field, check){
                    fields_count += 1;
                }else{
                continue; 
                }
            }else{
                continue;
            }
         }
    }
    if fields_count == 7 {
        true
    }else{
    false
    }
}

fn check_field(_field: &str, check: &str) -> bool{
    let value: &str = _field.split(":").collect::<Vec<&str>>()[1];
    if check.contains("yr"){
        if check_year(&value, &check){
            // println!("check_year");
            true
        }else{
            false
        }
    }else{
        if check_other(&value, &check){
            true
        }else{
            false
        }
    }
}

fn check_range(value: &str, range_start: u32, range_end: u32) -> bool{
    let num = value.parse::<u32>().unwrap();
    if num >= range_start{
        if num <= range_end{
            true
        }else{
            false
        }
    }else{
        false
    }
}

fn check_other(value: &str, check: &str) -> bool{
    if check == "hgt"{
        if check_height(value){
            true
        }else{
            false
        }
    }else if check == "hcl"{
        if check_hair_color(value){
            true
        }else{
            false
        }
    }else if check == "ecl"{
        if check_eye_color(value){
            true
        }else{
            false
        }
    }else if check == "pid"{
        if check_passport_id(value){
            true
        }else{
            false
        }
    }else{
        false
    }
}

fn check_year(value: &str, check: &str) -> bool{
    if check == "byr"{
        if check_range(value, 1920, 2002){
            true
        }else{
            false
        }
    }else if check == "iyr"{
        if check_range(value, 2010, 2020){
            true
        }else{
            false
        }
    }else{
        if check_range(value, 2020, 2030){
            true
        }else{
            false
        }
    }
}

fn check_height(value: &str) -> bool{
    if value.contains("in"){
        let height: i32 = match value.replace(&['i','n'][..], "").parse::<i32>() {
            Ok(value) => {
                value
            },
            Err (e) => {
                0
            }
        };
        if height >= 59{
            if height <= 76 {
                true
            }else{
                false
            }
        }else{
            false
        }
    }else if value.contains("cm"){
        let height: i32 = match value.replace(&['c','m'][..], "").parse::<i32>() {
            Ok(value) => {
                value
            },
            Err (e) => {
                0
            }
        };
        if height >= 150{
            if height <= 193 {
                true
            }else{
                false
            }
        }else{
            false
        }
    }else {
        false
    }
}

fn check_hair_color(value: &str) -> bool{
    if value.len() == 7{
        if value.contains("#"){
            true
        }else{
            false
        }
    }else{
        false
    }
}

fn check_eye_color(value: &str) -> bool{
    let mut verified = false;
    let colors: Vec<String> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().map(|x| x.to_string()).collect();
    for color in colors.into_iter() {
        if value == color{
            verified = true;
        }else {
            continue;
        };
    };
    verified
}

fn check_passport_id(value: &str) -> bool{
    let num: i32 = match value.parse::<i32>(){
        Ok(value) => {
            value
        },
        Err (e) => {
            0
        }
    };
    if value.len() == 9 {
        true
    }else{
        false
    }
}
