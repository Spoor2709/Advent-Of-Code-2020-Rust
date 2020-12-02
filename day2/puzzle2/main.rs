use std::fs;

fn main(){
  let input = fs::read_to_string("../password.input")
        .expect("Something went wrong reading the file");
  let passwords: Vec<&str> = input.lines().collect();
  let mut count: u32 = 0;
  for password in passwords.into_iter() {
    let password_split: Vec<&str> =  password.split_whitespace().into_iter().collect();
    let pass_range: Vec<u32> = password_split[0].split("-").into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let pass_letter: char = password_split[1].trim_matches(':').parse::<char>().unwrap();
    let password_string: &str = password_split[2];
    if letter_check(pass_range[0], pass_range[1], pass_letter, password_string) {
      count = count + 1;
    };
  };
  println!("{:#?}", count);
}

fn letter_check(range_start: u32, range_end: u32  , letter: char, password: &str) -> bool{
  let mut contains_letter: bool = false;
  let letter_value1: u32 = range_start -1 ;
  let letter_value2: u32 = range_end -1 ;
  let mut count: u32 =  0;
  let password: Vec<char> = password.chars().collect();
  if password[letter_value1 as usize] == letter {
    if password[letter_value2 as usize] == letter {
      contains_letter = false;
    }else {
      contains_letter = true;
    }
  }else {
    if password[letter_value2 as usize] == letter {
      contains_letter = true;
    } else {
      false;
    }
  }
  contains_letter
}