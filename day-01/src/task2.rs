
pub fn task2() -> u64 {
  let input = include_str!("input.txt");
  let entries: Vec<&str> = input.split("\n").collect();
  let mut sum = 0;
  
  for entry in entries {
    let first = get_first_num(entry).expect("Couldn't get first num");
    let last = get_last_num(entry).expect("Couldn't get last num");
    let num: u64 = format!("{}{}", first, last).parse().expect("Couldn't parse");
    sum += num;
  }

  sum
}

fn get_first_num(inp: &str) -> Option<char> {
  for (i, c) in inp.char_indices() {
    if vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&c) {
      return Some(c);
    }

    let rem = &inp[i..];
    if rem.starts_with("zero") {
      return Some('0');
    } else if rem.starts_with("one") {
      return Some('1');
    } else if rem.starts_with("two") {
      return Some('2');
    } else if rem.starts_with("three") {
      return Some('3');
    } else if rem.starts_with("four") {
      return Some('4');
    } else if rem.starts_with("five") {
      return Some('5');
    } else if rem.starts_with("six") {
      return Some('6');
    } else if rem.starts_with("seven") {
      return Some('7');
    } else if rem.starts_with("eight") {
      return Some('8');
    } else if rem.starts_with("nine") {
      return Some('9');
    }
  }

  return None
}

fn get_last_num(inp: &str) -> Option<char> {
  let inp: String = inp.chars().rev().collect();
  for (i, c) in inp.char_indices() {
    if vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&c) {
      return Some(c);
    }

    let rem = &inp[i..];
    if rem.starts_with("orez") {
      return Some('0');
    } else if rem.starts_with("eno") {
      return Some('1');
    } else if rem.starts_with("owt") {
      return Some('2');
    } else if rem.starts_with("eerht") {
      return Some('3');
    } else if rem.starts_with("ruof") {
      return Some('4');
    } else if rem.starts_with("evif") {
      return Some('5');
    } else if rem.starts_with("xis") {
      return Some('6');
    } else if rem.starts_with("neves") {
      return Some('7');
    } else if rem.starts_with("thgie") {
      return Some('8');
    } else if rem.starts_with("enin") {
      return Some('9');
    }
  }

  return None
}