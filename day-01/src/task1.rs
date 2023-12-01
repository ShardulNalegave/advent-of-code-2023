
pub fn task1() -> u64 {
  let input = include_str!("input.txt");
  let entries: Vec<&str> = input.split("\n").collect();
  let mut sum = 0;
  
  for entry in entries {
    let first = get_first_num(entry).expect("Couldn't get first num");
    let last = get_first_num(&entry.chars().rev().collect::<String>()).expect("Couldn't get last num");
    let num: u64 = format!("{}{}", first, last).parse().expect("Couldn't parse");
    sum += num;
  }

  sum
}

fn get_first_num(inp: &str) -> Option<char> {
  for c in inp.chars() {
    if vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&c) {
      return Some(c)
    }
  }

  return None
}