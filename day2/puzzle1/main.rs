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

fn letter_check(range_start: u32, range_end: u32  , letter: char, password: &str) -> bool {
  let mut count: u32 =  0;
  for password_letter in password.chars() {
    // println!("{:#?}{:#?}", password_letter, letter);
    if password_letter == letter {
      count = count + 1;
    };
  };
  if count >= range_start {
    if count <= range_end {
      true
    }else {
      false
    }
  } else {
  false
  }
}