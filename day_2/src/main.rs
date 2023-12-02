fn file_to_string(path: &str) -> String {
  use std::fs::File;
  use std::io::prelude::*;
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cubes {
  red: i32,
  green: i32,
  blue: i32,
  game_id: i32,
}

enum Colour {
  Red,
  Green,
  Blue
}

impl Colour {
  fn from_str(input: &str) -> Colour {
    match input {
      "red" => Colour::Red,
      "green" => Colour::Green,
      "blue" => Colour::Blue,
      _ => unimplemented!()
    }
  }
}

impl Cubes {
  fn new(game_id:i32) -> Self {
    Cubes {
      red: 0,
      green: 0,
      blue: 0,
      game_id,
    }
  }

  fn get_power(&self) -> i32 {
    self.red * self.green * self.blue
  }

  fn update(&mut self, colour: Colour, num: i32) {
    match colour {
      Colour::Red => self.red = if self.red < num {num} else {self.red},
      Colour::Green => self.green = if self.green < num {num} else {self.green},
      Colour::Blue => self.blue = if self.blue < num {num} else {self.blue},
    }
  }
}

fn main() {
  println!("Part 1: {}", part_1(file_to_string("inputs/input")));
  println!("Part 2: {}", part_2(file_to_string("inputs/input")));
}

fn part_1(input: String) -> i32 {
  let mut res = 0;
  for line in input.split('\n') {
    if line.is_empty() {
      continue;
    }
    let parse = parse_line(line);
    if parse.red <= 12 && parse.green <= 13 && parse.blue <= 14 {
      res += parse.game_id;
    }
  }
  res
}

fn part_2(input: String) -> i32 {
  let mut res = 0;
  for line in input.split('\n') {
    if line.is_empty() {
      continue;
    }
    res += parse_line(line).get_power();
  }
  res
}

fn parse_line(line: &str) -> Cubes {
  let mut parts = line.strip_prefix("Game ").unwrap().split(": ");
  let value = parts.next().unwrap().parse::<i32>().unwrap();
  let mut max_cubes: Cubes = Cubes::new(value);

  for round in parts.next().unwrap().split("; ") {
    for cubes in round.split(", ") {
      let mut parts = cubes.split(' ');
      let num = parts.next().unwrap().parse::<i32>().unwrap();
      max_cubes.update(Colour::from_str(parts.next().unwrap()),num);
    }
  }
  max_cubes
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_parse_correct() {
    assert_eq!(parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Cubes {
      red: 4,
      green: 2,
      blue: 6,
      game_id: 1
    });

  }

  #[test ]
  fn test_power() {
    assert_eq!(parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").get_power(), 48);
  }

  #[test]
  fn test_parse_incorrect() {
    assert_eq!(parse_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").red,20);
  }

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 8);
  }

  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 2286);
  }

  #[test]
  fn test_incorrect_answers() {
  }
}
