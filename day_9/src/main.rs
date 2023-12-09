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

struct Sequence {
  original: Vec<i32>,
}

impl Sequence {
  fn new(input: &str) -> Self {
    Sequence {
      original: input.split_whitespace().map(|e| e.parse::<i32>().unwrap()).collect(),
    }
  }
  
  fn get_derivitive(&self) -> Option<Sequence> {
    let mut res = vec![];
    if self.is_final() {
      None
    } else {
      let mut i = 0;
      while i + 1 < self.original.len() {
        res.push(self.original[i+1] - self.original[i]);
        i += 1;
      }
      Some(Sequence { original: res })
    }
    
  }
  
  fn is_final(&self) -> bool {
    self.original.iter().all(|e| *e == 0)
  }
  
  fn get_next_value(&self) -> i32 {
    if self.is_final() {
      0
    } else {
      self.original.last().unwrap() + self.get_derivitive().unwrap().get_next_value()
    }
  }
  
  fn get_prev_value(&self) -> i32 {
    if self.is_final() {
      0      
    } else {
      self.original.first().unwrap() - self.get_derivitive().unwrap().get_prev_value()
    }
  }
}



fn part_1(input: String) -> i32 {
  let mut res = 0;
  for line in input.lines() {
    if line.is_empty() {
      continue;
    }
    res += Sequence::new(line).get_next_value();
  }
  res
}

fn part_2(input: String) -> i32 {
  let mut res = 0;
  for line in input.lines() {
    if line.is_empty() {
      continue;
    }
    res += Sequence::new(line).get_prev_value();
  }
  res}
  
  
  #[cfg(test)]
  mod tests {
    use super::*;
    #[test]
    fn test_sequence_parse() {
      assert_eq!(Sequence::new("0 3 6 9 12 15").original, vec![0,3,6,9,12,15]);
    }
    
    #[test]
    fn test_deriv() {
      let seq = Sequence::new("0 3 6 9 12 15");
      let deriv = seq.get_derivitive();
      assert_eq!(deriv.unwrap().original, vec![3,3,3,3,3]);
    }
    
    #[test]
    fn test_next() {
      let seq = Sequence::new("0 3 6 9 12 15");
      assert_eq!(seq.get_next_value(), 18);
    }
    
    #[test]
    fn test_part_1() {
      assert_eq!(part_1(file_to_string("inputs/demo")), 114);
    }
    
    #[test]
    fn test_prev() {
      let seq = Sequence::new("0 3 6 9 12 15");
      assert_eq!(seq.get_prev_value(), -3);
    }
    
    #[test]
    fn test_part_2() {
      assert_eq!(part_2(file_to_string("inputs/demo")), 2);
    }
    
    #[test]
    fn test_incorrect_answers() {
    }
  }
  