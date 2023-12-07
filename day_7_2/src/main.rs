use std::{collections::HashMap, cmp::Ordering};

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
  {
    let start = Instant::now();
    println!("Part 2: {}", part_2(file_to_string("inputs/input")));
    let duration = start.elapsed();
    println!("Time for part 2: {:?}", duration);
  }
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
  FiveKind  = 6,
  FourKind  = 5,
  House     = 4,
  ThreeKind = 3,
  TwoPair   = 2,
  OnePair   = 1,
  HighCard  = 0 ,
}

#[derive(PartialEq, Eq)]
struct Hand {
  contents: String,
  bid: i32
}

impl Hand {
  fn new(contents: &str, bid: i32) -> Self {
    Hand {
      contents: String::from(contents),
      bid,
    }
  }
  fn get_type(&self) -> HandType {
    let mut count = HashMap::new();
    for e in self.contents.chars().map(get_card_value) {
      let elem = count.entry(e).or_insert(0);
      *elem += 1;
    }
    let jokers = count.remove(&0).unwrap_or(0);
    let mut counts = count.into_values().collect::<Vec<usize>>();

    counts.sort();
    counts.reverse();
    let top_joker = counts.first().unwrap_or(&0) + jokers;
    match top_joker {
      5 => HandType::FiveKind,
      4 => HandType::FourKind,
      3 => {
        if counts[1] == 2 {
          HandType::House
        } else {
          HandType::ThreeKind
        }
      },
      2 => {
        if counts[1] == 2 {
          HandType::TwoPair
        } else {
          HandType::OnePair
        }
      }
      1 => HandType::HighCard,
      _ => unreachable!()
    }
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.contents == other.contents {
      return Ordering::Equal
    }
    let self_hand = self.get_type();
    let other_hand = other.get_type();
    let mut self_chars = self.contents.chars();
    let mut other_chars = other.contents.chars();

    let mut candidate = self_hand.cmp(&other_hand);
    while candidate == Ordering::Equal {
      candidate = get_card_value(self_chars.next().unwrap())
      .cmp(&get_card_value(other_chars.next().unwrap()))
    }
    candidate
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn get_card_value(card: char) -> u8 {
  match card {
    '0'..='9' => card.to_digit(10).unwrap() as u8,
    'T' => 10,
    'J' => 0,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
    _ => unreachable!(),
  }
}




fn part_2(input: String) -> i32 {
  let mut hands = vec![];
  for line in input.lines() {
    let mut parts = line.split_whitespace();
    hands.push(Hand::new(
      parts.next().unwrap(),
      parts.next().unwrap().parse::<i32>().unwrap(),
    ));
  }
  hands.sort();
  let mut res = 0;
  for (i,hand) in hands.into_iter().enumerate() {
    res += (i +1) as i32 * hand.bid;
  }
  res
}



#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_eq_hands() {
    assert_eq!(HandType::FiveKind,HandType::FiveKind);
    assert_ne!(HandType::FiveKind,HandType::FourKind);
  }

  #[test]
  fn test_ord_hands() {
    assert!(HandType::FiveKind > HandType::FourKind);
    assert!(HandType::HighCard < HandType::FourKind);
  }

  #[test]
  fn test_five_kind() {
    assert_eq!(Hand::new("AAAAA",0).get_type(),HandType::FiveKind);
  }

  #[test]
  fn test_four_kind() {
    assert_eq!(Hand::new("AAAA1",0).get_type(),HandType::FourKind);
    assert_eq!(Hand::new("1AAAA",0).get_type(),HandType::FourKind);
  }


  #[test]
  fn test_threes() {
    assert_eq!(Hand::new("1AAA1",0).get_type(),HandType::House);
    assert_eq!(Hand::new("11AK1",0).get_type(),HandType::ThreeKind);
  }

  #[test]
  fn test_twos() {
    assert_eq!(Hand::new("1A2A1",0).get_type(),HandType::TwoPair);
    assert_eq!(Hand::new("11234",0).get_type(),HandType::OnePair);
  }

  #[test]
  fn test_one() {
    assert_eq!(Hand::new("12345",0).get_type(),HandType::HighCard);
  }

  #[test]
  fn test_hand_ordering() {
    assert!(Hand::new("AAAAA",0) > Hand::new("12345",0) );
  }

  #[test]
  fn test_hand_ordering_same() {
    assert!(Hand::new("33332",0) > Hand::new("2AAAA",0) );
  }

  #[test]
  fn test_part_two() {
    assert_eq!( part_2(file_to_string("inputs/demo")),5905);
  }

  #[test]
  fn test_incorrect_answers() {
  }
}
