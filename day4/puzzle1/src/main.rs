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
    let checks = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    for check in checks.into_iter(){
            if passport.contains(check){
                fields_count += 1;
            }else{
            continue; 
        }
    }
    if fields_count == 7 {
        true
    }else{
    false
    }
}