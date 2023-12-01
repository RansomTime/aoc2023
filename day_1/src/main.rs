use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn main() -> std::io::Result<()> {
    println!("part 1: {}", part_1(file_to_string("inputs/input")));
    println!("part 2: {}", part_2(file_to_string("inputs/input")));
    Ok(())
}

fn part_1(data: String) -> i32 {
    let digits = Regex::new(r"[0-9]").unwrap();
    let mut result = 0;

    for line in data.split('\n') {
        if line.is_empty() { //eof
            continue;
        }
        let mut caps = digits.find_iter(line);
        let next = caps.next().unwrap().as_str();
        let last = match caps.last() {
            Some(m) => m.as_str(),
            None => next,
        };
        result += format!("{}{}", next,last).parse::<i32>().unwrap();
    }
    result
}


fn part_2(data: String) -> i32 {
    let mut result = 0;

    for mut line in data.split('\n') {
        if line.is_empty() { //eof
            continue;
        }

        let mut first = None;
        while first.is_none() {
            if line.starts_with("one") | line.starts_with('1') {
                first = Some(1);
            } else if line.starts_with("two") | line.starts_with('2') {
                first = Some(2);
            } else if line.starts_with("three") | line.starts_with('3') {
                first = Some(3);
            } else if line.starts_with("four") | line.starts_with('4') {
                first = Some(4);
            } else if line.starts_with("five") | line.starts_with('5') {
                first = Some(5);
            } else if line.starts_with("six") | line.starts_with('6') {
                first = Some(6);
            } else if line.starts_with("seven") | line.starts_with('7') {
                first = Some(7);
            } else if line.starts_with("eight") | line.starts_with('8') {
                first = Some(8);
            } else if line.starts_with("nine") | line.starts_with('9') {
                first = Some(9);
            } else {
                line = &line[1..];
            }
        }

        let mut last = None;

        // reverse the string, and keep a binding
        let rev = line.chars().rev().collect::<String>();
        line = rev.as_str();

        while last.is_none() {
            if line.starts_with("eno") | line.starts_with('1') {
                last = Some(1);
            } else if line.starts_with("owt") | line.starts_with('2') {
                last = Some(2);
            } else if line.starts_with("eerht") | line.starts_with('3') {
                last = Some(3);
            } else if line.starts_with("ruof") | line.starts_with('4') {
                last = Some(4);
            } else if line.starts_with("evif") | line.starts_with('5') {
                last = Some(5);
            } else if line.starts_with("xis") | line.starts_with('6') {
                last = Some(6);
            } else if line.starts_with("neves") | line.starts_with('7') {
                last = Some(7);
            } else if line.starts_with("thgie") | line.starts_with('8') {
                last = Some(8);
            } else if line.starts_with("enin") | line.starts_with('9') {
                last = Some(9);
            } else {
                line = &line[1..];
            }
        }
        result += format!("{}{}", first.unwrap(), last.unwrap()).parse::<i32>().unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, file_to_string};

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(file_to_string("inputs/demo_1")), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(file_to_string("inputs/demo_2")), 281);
    }

    #[test]
    fn test_part_2_overlap() {
        assert_eq!(part_2("oneight".to_string()), 18);
        assert_eq!(part_2("threeight".to_string()), 38);
    }

    #[test]
    fn test_incorrect_answers() {
        assert_ne!(part_2(file_to_string("inputs/input")), 54533);
    }
}
