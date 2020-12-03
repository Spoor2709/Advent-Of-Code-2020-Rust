use std::fs;

fn main(){
  let input = fs::read_to_string("../puzzle.input")
        .expect("Something went wrong reading the file");
  let terrain: Vec<&str> = input.lines().collect();
  let terrain: Vec<String> = terrain.into_iter().map(|x| x.repeat(1000)).collect();
  let tree_count: u64 = check_paths(&terrain);
  println!("{:#?}", tree_count);
}

fn check_paths(terrain: &Vec<String>) -> u64{
  let slope_1 = check_path(&terrain, 1);
  let slope_2 = check_path(&terrain, 3);
  let slope_3 = check_path(&terrain, 5);
  let slope_4 = check_path(&terrain, 7);
  let slope_5 = check_steep(&terrain, 1);
  println!("{},{},{},{},{}", slope_1, slope_2, slope_3, slope_4, slope_5);
  let answer: u64 = slope_1 as u64 * slope_2 as u64 * slope_3 as u64 * slope_4 as u64 * slope_5 as u64;
  answer
}

fn check_path(terrain: &Vec<String>, slope_angle: u32) -> u32{
  let mut tree_count: u32 = 0;
  let mut x_count: u32 = 0;
  for line in terrain {
    if line.chars().nth(x_count as usize).unwrap() == '#'{
      tree_count += 1;
    }
    x_count += &slope_angle;
  }
  tree_count
}

fn check_steep(terrain: &Vec<String>, slope_angle: u32) -> u32{
  let mut tree_count: u32 = 0;
  let mut x_count: u32 = 0;
  let mut loop_count: u32 = 1;
  for line in terrain {
    if loop_count % 2 == 0{
      loop_count += 1;
      continue
    }else{
      if line.chars().nth(x_count as usize).unwrap() == '#'{
        tree_count += 1;
      }
      loop_count += 1;
      x_count += &slope_angle;
    };
  }
  tree_count
}