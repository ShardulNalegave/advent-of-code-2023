
// ===== Imports =====
use crate::utils::Entry;
// ===================

const RED_MAX: u64 = 12;
const GREEN_MAX: u64 = 13;
const BLUE_MAX: u64 = 14;

pub fn task1(entries: &Vec<Entry>) -> u64 {
  let mut sum = 0;

  for entry in entries {
    let mut possible = true;
    for set in &entry.sets {
      if set.0 > RED_MAX || set.1 > GREEN_MAX || set.2 > BLUE_MAX {
        possible = false;
      }
    }

    if possible {
      sum += entry.id;
    }
  }

  sum
}