fn file_to_string(path: &str) -> String {
  use std::fs::File;
  use std::io::prelude::*;
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

#[derive(PartialEq, Eq,Debug,Copy,Clone)]
struct Map {
  source: i64,
  delta: i64,
  length: i64,
}

impl Map {
  fn from_mapping_str(input: &str) -> Self {
    let mut parts = input.split(' ');
    let dest = parts.next().unwrap().parse::<i64>().unwrap();
    let source = parts.next().unwrap().parse::<i64>().unwrap();
    let length = parts.next().unwrap().parse::<i64>().unwrap();
    let delta = dest - source; // source + delta = dest
    Map {
      source,
      delta,
      length,
    }
  }

  fn navigate(&self, from: i64) -> Option<i64> {
    if from >= self.source && from < self.source + self.length {
      Some(from + self.delta)
    } else {
      None
    }
  }
}


struct Maps {
  contents: Vec<Vec<Map>>,
}

impl Maps {
  fn new() -> Self {
    Maps {
      contents: vec![]
    }
  }

  fn push(&mut self, maps: Vec<Map>)  {
    self.contents.push(maps);
  }
  fn navigate(&self, mut loc:i64) -> i64 {

    for mappings in &self.contents {
      for map in mappings {
        if let Some(new_loc) = map.navigate(loc) {
          loc = new_loc;
          break;
        }
      }
    }
    loc
  }
}



fn main() {
  use std::time::Instant;
  {
    let start = Instant::now();
    println!("Part 1: {}", part_1(file_to_string("inputs/input")));
    let duration = start.elapsed();
    println!("Time elapsed in part_1() is: {:?}", duration);
  }
  {
    let start = Instant::now();
    println!("Part 2: {}", part_2(file_to_string("inputs/input")));
    let duration = start.elapsed();
    println!("Time elapsed in part_2() is: {:?}", duration);
  }
}

fn part_1(input: String) -> i64 {
  let mut lines = input.lines();
  let seeds:Vec<i64> = lines.next().unwrap()
  .strip_prefix("seeds: ").unwrap().split(' ')
  .map(|e| e.parse::<i64>().unwrap()).collect();

  let mut maps = Maps::new();
  let mut curr_maps: Vec<Map> = vec![];
  lines.next();
  lines.next();
  for line in lines {
    if line.is_empty() {
      maps.push(curr_maps);
      curr_maps = vec![];
      continue;
    }
    if line.contains("map:") {
      continue;
    }
    curr_maps.push(Map::from_mapping_str(line));
  }

  maps.push(curr_maps);
  seeds.into_iter().map(|e| maps.navigate(e)).min().unwrap()
}

fn part_2(input: String) -> i64 {
  let mut lines = input.lines();
  let seed_ranges:Vec<i64> = lines.next().unwrap()
  .strip_prefix("seeds: ").unwrap().split(' ')
  .map(|e| e.parse::<i64>().unwrap()).collect();


  let mut maps = Maps::new();
  let mut curr_maps: Vec<Map> = vec![];
  lines.next();
  lines.next();
  for line in lines {
    if line.is_empty() {
      maps.push(curr_maps);
      curr_maps = vec![];
      continue;
    }
    if line.contains("map:") {
      continue;
    }
    curr_maps.push(Map::from_mapping_str(line));
  }

  maps.push(curr_maps);
  let mut res = i64::MAX;
  for chunk in seed_ranges.chunks(2) {
    let start = chunk[0];
    let len = chunk[1];
    for i in start..=start+len {
      let next = maps.navigate(i);
      if next < res {
        res = next;
      }
    }
  }
  res
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn make_map() {
    assert_eq!(Map::from_mapping_str("50 98 2"),Map{
      source: 98,
      delta: -48,
      length: 2,
    });
    assert_eq!(Map::from_mapping_str("52 50 48"),Map{
      source: 50,
      delta: 2,
      length: 48,
    });

  }

  #[test]
  fn navigate_map() {
    let map = Map::from_mapping_str("50 98 2");
    assert_eq!(map.navigate(98),Some(50));
    assert_eq!(map.navigate(99),Some(51));
    assert_eq!(map.navigate(100),None);
    assert_eq!(map.navigate(97),None);
  }

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 35);
  }


  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 46);
  }

  #[test]
  fn test_incorrect_answers() {
    assert_ne!(part_1(file_to_string("inputs/input")), 660782964);
  }
}
