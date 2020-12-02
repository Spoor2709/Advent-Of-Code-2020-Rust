use std::fs;

fn main(){
  let input = fs::read_to_string("../numbersFile.input")
        .expect("Something went wrong reading the file");
  let num_array: Vec<i32> = input.split("\n").into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
  println!("{:#?}",num_array);
  for num in num_array.iter() {
    for num_2 in num_array.iter() {
      for num_3 in num_array.iter() {
        if *num + *num_2 + *num_3 == 2020 {
          println!("the answer is: {:#?}",num * num_2 *num_3)
        }
      }
    }
  }
}