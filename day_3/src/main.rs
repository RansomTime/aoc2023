fn file_to_string(path: &str) -> String {
  use std::fs::File;
  use std::io::prelude::*;
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  if contents.ends_with('\n') {
    contents.pop();
    if contents.ends_with('\r') {
      contents.pop();
    }
  }
  contents
}

fn main() {
  println!("Part 1: {}", part_1(file_to_string("inputs/input")));
  println!("Part 2: {}", part_2(file_to_string("inputs/input")));
}

#[derive (Debug, Copy, Clone, PartialEq)]
struct Num {
  start: usize,
  end: usize,
  value: i32,
}

impl Num {
  fn get_neighbours(&self, row_size: usize) -> Vec<usize> {
    // returns vector of neighbours around &self
    // this won't go out of index, but might overextend
    // because we're going to filter in the next step we don't care

    let mut res: Vec<usize> = vec![];
    let left = match self.start.checked_sub(1) {
      Some(l) => {
        res.push(l);
        l
      },
      None => self.start,
    };
    let right = self.end + 1;
    res.push(right);

    // if we can't go up from left then we're on the first line
    if let Some(in_range) = left.checked_sub(row_size) {
      res.append(&mut (in_range..=right-row_size).collect::<Vec<usize>>());
    };

    res.append(&mut(left+row_size..=right+row_size).collect::<Vec<usize>>());
    res

  }
}

#[derive(PartialEq)]
enum Symbol {
  Dot,
  Number,
  Newline,
  Symbol,
  Gear,
}

impl Symbol {
  fn from_char(input: char) -> Symbol {
    match input {
      '.' => Symbol::Dot,
      '0'..='9' => Symbol::Number,
      '\n' => Symbol::Newline,
      '*' => Symbol::Gear, // part 2
      _ => Symbol::Symbol,
    }
  }
}


fn part_1(input: String) -> i32 {

  let mut start_num: Option<usize> = None;
  let mut symbols: Vec<usize> = vec![];
  let mut nums: Vec <Num> = vec![];
  let row_size = input.split('\n').next().unwrap().chars().count() + 1;

  for (pos, symbol) in input.chars().enumerate() {
    let symbol_type = Symbol::from_char(symbol);
    if symbol_type == Symbol::Symbol || symbol_type == Symbol::Gear {
      symbols.push(pos);
    }

    if let Some(start) = start_num {
      // we're half-way through parsing a number
    if symbol_type != Symbol::Number {
      nums.push(
        Num {
          start,
          end: pos-1,
          value: input[start..pos].parse::<i32>().unwrap(),
        }
      );
      start_num = None;
    }
  } else if symbol_type == Symbol::Number {
    start_num = Some(pos);
  }
}

let mut res = 0;

for num in nums {
  let mut neighbours = num.get_neighbours(row_size);
  neighbours.retain(|e| symbols.contains(e));
  if !neighbours.is_empty() { // next to symbol
    res += num.value;
  }
}


res
}

struct Gear {
  position: usize,
  parts: Vec<i32>,
}

impl Gear {
  fn get_ratio(&self) -> i32 {
    if self.parts.len() == 2 {
      self.parts.first().unwrap() * self.parts.last().unwrap()
    } else {
      0
    }
  }
}

fn part_2(input: String) -> i32 {

  let mut symbols: Vec<usize> = vec![];
  let mut gears: Vec<Gear> = vec![];
  let mut gear_pos = vec![];
  let mut nums: Vec <Num> = vec![];
  let row_size = input.split('\n').next().unwrap().chars().count() + 1;
  let mut start_num: Option<usize> = None;

  for (pos, symbol) in input.chars().enumerate() {
    let symbol_type = Symbol::from_char(symbol);
    if symbol_type == Symbol::Symbol {
      symbols.push(pos);
    }
    if symbol_type == Symbol::Gear {
      symbols.push(pos);
      gears.push(Gear {position: pos, parts: vec![]});
      gear_pos.push(pos);
    }
    if let Some(start) = start_num {
      if symbol_type != Symbol::Number {
        nums.push(
          Num {
            start,
            end: pos-1,
            value: input[start..pos].parse::<i32>().unwrap(),
          }
        );
        start_num = None;
      }
    } else if symbol_type == Symbol::Number {
      start_num = Some(pos);
    }
  }

  for num in nums {
    let mut neighbour_gears = num.get_neighbours(row_size);
    neighbour_gears.retain(|e| gear_pos.contains(e));

    for neighbour_gear in neighbour_gears {
      for gear in &mut gears {
        if gear.position == neighbour_gear {
          gear.parts.push(num.value);
        }
      }
    }
  }

  gears.into_iter().map(|e| e.get_ratio()).sum()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 4361);
  }

  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 467835);
  }

  #[test]
  fn test_incorrect_answers() {
  }
}
