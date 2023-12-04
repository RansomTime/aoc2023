fn file_to_string(path: &str) -> String {
  use std::fs::File;
  use std::io::prelude::*;
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}
fn main() {
  println!("Part 1: {}", part_1(file_to_string("inputs/input")));
  println!("Part 2: {}", part_2(file_to_string("inputs/input")));
}

struct Card {
  values: Vec<i32>,
  winning: Vec<i32>,
}

impl Card {
  fn from(input: &str) -> Self {

    let mut nums = input.split(": ").last().unwrap().split(" | ");
    let winning = Self::vectorise(nums.next().unwrap());
    let values = Self::vectorise(nums.next().unwrap());
    Card {
      values,
      winning,
    }
  }

  fn vectorise(card: &str) -> Vec<i32> {
    //println!("'{}'",card);
    card.replace("  "," ").trim().split(' ').map(|e| e.parse::<i32>().unwrap()).collect()
  }

  fn calculate_num_wins(&self) -> usize {
    let mut res = self.values.clone();
    res.retain(|e| self.winning.contains(e));
    res.len()
  }

  fn calculate_part_1_points(&self) -> i32 {
    let wins = self.calculate_num_wins();
    if wins == 0 {
      0
    } else {
      let mut res = 1;
      for _ in 1..wins {
        res *= 2;
      }
      res
    }
  }

}

struct Cards {
  card: Card,
  owned: usize
}

impl Cards {
  fn from(input: &str) -> Self {
    Cards {
      card: Card::from(input),
      owned: 1,
    }
  }
}

fn part_1(input: String) -> i32 {
  let mut res = 0;
  for line in input.lines() {
    let card = Card::from(line);
    res += card.calculate_part_1_points();
  }
  res
}

fn part_2(input: String) -> usize {
  let mut cvec = vec![];
  for line in input.lines() {
    cvec.push(Cards::from(line));
  }

  for i in 0..cvec.len() {
    let (wins,owned) = {
      // borrow checker buster
      let current_cards = cvec.get(i).unwrap();
      (current_cards.card.calculate_num_wins(), current_cards.owned)
    };
    for j in i+1..=i+wins {
      if let Some(cards) = cvec.get_mut(j) {
        cards.owned += owned;
      }
    }
  }
  cvec.into_iter().map(|e| e.owned).sum()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_card_to_vec() {
    assert_eq!(Card::vectorise("1 2 3"), vec![1,2,3]);
  }

  #[test]
  fn test_card_to_vec_more_than_one_space() {
    assert_eq!(Card::vectorise("4  3  3"), vec![4,3,3]);
  }

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 13);
  }


  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 30);
  }

  #[test]
  fn test_incorrect_answers() {
  }
}
