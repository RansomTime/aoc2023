fn file_to_string(path: &str) -> String {
  use std::fs;
  fs::read_to_string(path).unwrap()
}

fn main() {
  use std::time::Instant;
  {
    let start = Instant::now();
    println!("Part 1: {}", part_1(file_to_string("inputs/input")));
    let duration = start.elapsed();
    println!("Time for part 1: {:?}", duration);
  }
  {
    let start = Instant::now();
    println!("Part 2: {}", part_2(file_to_string("inputs/input")));
    let duration = start.elapsed();
    println!("Time for part 2: {:?}", duration);
  }
}


fn part_1(input: String) -> i32 {
  unimplemented!()
}

fn part_2(input: String) -> i32 {
  unimplemented!()
}


#[cfg(test)]
mod tests {
  use super::*;
  /*
  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 8);
  }

  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 2286);
  } */

  #[test]
  fn test_incorrect_answers() {
  }
}
