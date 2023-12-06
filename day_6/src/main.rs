fn file_to_string(path: &str) -> String {
  use std::fs::File;
  use std::io::prelude::*;
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn main() {
  use std::time::Instant;
  let input = file_to_string("inputs/input");

  {
    let start = Instant::now();
    println!("Part 1: {}", part_1(&input));
    let duration = start.elapsed();
    println!("Time for part 1: {:?}", duration);
  }
  {
    let start = Instant::now();
    println!("Part 2: {}", part_2(&input));
    let duration = start.elapsed();
    println!("Time for part 2: {:?}", duration);
  }
}

fn count_win_races(t: i64, s: i64) -> i64 {

  let mut acc = 0;
  for v in 0..t {
    if v*(t - v) > s {
      acc += 1;
    }
  }
  acc
}

fn part_1(input: &str) -> i64 {
  let mut lines = input.lines();
  let mut times = lines.next().unwrap().strip_prefix("Time:").unwrap()
  .split_whitespace().map(
    |e| e.parse::<i64>().unwrap()
  );
  let mut distances = lines.next().unwrap().strip_prefix("Distance:").unwrap()
  .split_whitespace().map(
    |e| e.parse::<i64>().unwrap()
  );
  let mut res = 1;
  while let (Some(t), Some(s)) = (times.next(), distances.next()) {
    res *= count_win_races(t,s);
  }
  res

}

fn part_2(input: &str) -> i64 {
  let mut lines = input.lines();
  let time = lines.next().unwrap().strip_prefix("Time:").unwrap()
  .split_whitespace().collect::<String>().parse::<i64>().unwrap();
  let distance = lines.next().unwrap().strip_prefix("Distance:").unwrap()
  .split_whitespace().collect::<String>().parse::<i64>().unwrap();

  count_win_races(time, distance)
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_win_races() {
    assert_eq!(count_win_races(7,9), 4);
  }

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(&file_to_string("inputs/demo")), 288);
  }

  #[test]
  fn test_part_2() {
    assert_eq!(part_2(&file_to_string("inputs/demo")), 71503);
  }

  #[test]
  fn test_incorrect_answers() {
  }
}
