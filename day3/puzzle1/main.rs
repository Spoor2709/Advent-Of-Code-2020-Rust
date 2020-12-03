use std::fs;

fn main(){
  let input = fs::read_to_string("../puzzle.input")
        .expect("Something went wrong reading the file");
  let terrain: Vec<&str> = input.lines().collect();
  let terrain: Vec<String> = terrain.into_iter().map(|x| x.repeat(500)).collect();
  let tree_count: u32 = check_path(terrain);
  println!("{:#?}", tree_count);
}

fn check_path(terrain: Vec<String>) -> u32{
  let mut tree_count: u32 = 0;
  let mut x_count: u32 = 0;
  for line in terrain {
    if x_count != 0 {
      if line.chars().nth(x_count as usize).unwrap() == '#'{
        tree_count += 1;
      }
      x_count += 3;
    } else {
      x_count += 3;
    };
  }
  tree_count
}