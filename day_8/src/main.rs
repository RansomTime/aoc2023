use std::collections::HashMap;

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

#[derive(Clone, Copy)]
enum Direction{
  Left,
  Right,
}

#[derive(Clone)]
struct Location {
  l_node: String,
  r_node: String
}

impl Location {
  fn new_with_key(input: &str) -> (String, Location) {
    let mut parts = input.split(" = ");
    let key = String::from(parts.next().unwrap());
    let mut location_parts = parts.next().unwrap()
    .strip_prefix('(').unwrap()
    .strip_suffix(')').unwrap()
    .split(", ");

    let l_node = String::from(location_parts.next().unwrap());
    let r_node = String::from(location_parts.next().unwrap());
    (key, Location {
      l_node,
      r_node
    })
  }
}


fn part_1(input: String) -> i32 {
  let mut locations = HashMap::new();
  let mut instructions = vec![];

  for (i,line) in input.lines().enumerate() {
    if i == 1 { continue; }
    if i == 0 {
      instructions = line.chars().map(|e| {
        match e {
          'L' => Direction::Left,
          'R' => Direction::Right,
          _   => unreachable!(),
        }
      }).collect();
      continue;
    }
    let (key, value) = Location::new_with_key(line);
    locations.insert(key,value);
  }

  let mut position = String::from("AAA");
  let mut res = 0;

  for instruction in instructions.into_iter().cycle() {
    if position == *"ZZZ" { break; }

    let location = locations.get(&position).unwrap();
    position = match instruction {
      Direction::Left => location.l_node.clone(),
      Direction::Right => location.r_node.clone(),
    };
    res+= 1;
  }

  res
}


fn get_period(
  mut location: String,
  instructions: Vec<Direction>,
  map: HashMap<String, Location>
)-> usize {

  for (i, instruction) in instructions.into_iter().cycle().enumerate() {
    if location.ends_with('Z') {
      return i;
    }
    let next = map.get(&location).unwrap();
    location = match instruction {
      Direction::Left => next.l_node.clone(),
      Direction::Right => next.r_node.clone(),
    };
  }
  unreachable!()
}

fn part_2(input: String) -> usize {
  let mut locations = HashMap::new();
  let mut instructions = vec![];
  let mut current_positions = vec![];

  for (i,line) in input.lines().enumerate() {
    if i == 1 { continue; }
    if i == 0 {
      instructions = line.chars().map(|e| {
        match e {
          'L' => Direction::Left,
          'R' => Direction::Right,
          _   => unreachable!(),
        }
      }).collect();
      continue;
    }
    let (key, value) = Location::new_with_key(line);

    if key.ends_with('A') {
      current_positions.push(key.clone());
    }
    locations.insert(key,value);
  }

  let mut periods = current_positions.into_iter()
  .map(|e| get_period(e,
    instructions.clone(),
    locations.clone()
  ));

  let mut res = periods.next().unwrap();
  for period in periods {
    res = num::integer::lcm(res,period);
  }
  res
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo_1")), 2);
  }

  #[test]
  fn test_part_1_2() {
    assert_eq!(part_1(file_to_string("inputs/demo_2")), 6);
  }


  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo_3")), 6);
  }

  #[test]
  fn test_correct_answers() {
    assert_eq!(part_1(file_to_string("inputs/input")),19241);
    assert_eq!(part_2(file_to_string("inputs/input")),9606140307013);
  }
}
