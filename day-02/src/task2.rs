
// ===== Imports =====
use crate::utils::Entry;
// ===================

pub fn task2(entries: &Vec<Entry>) -> u64 {
  let mut total_power = 0;

  for entry in entries {
    let mut min = (0, 0, 0);
    for set in &entry.sets {
      if set.0 > min.0 {
        min.0 = set.0;
      }

      if set.1 > min.1 {
        min.1 = set.1;
      }

      if set.2 > min.2 {
        min.2 = set.2;
      }
    }
    total_power += min.0 * min.1 * min.2;
  }

  total_power
}