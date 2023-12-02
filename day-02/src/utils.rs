
#[derive(Clone, Debug)]
pub struct Entry {
  pub id: u64,
  pub sets: Vec<(u64, u64, u64)>,
}

pub fn clean_data() -> Vec<Entry> {
  let data = include_str!("input.txt");
  let raw_entries = data.lines();
  let mut entries = vec![];

  for entry in raw_entries {
    let (game_id, game_data) = {
      let splt: Vec<&str> = entry.split(":").collect();

      (
        splt[0].split(" ").collect::<Vec<&str>>()[1].parse::<u64>().unwrap(),
        splt[1],
      )
    };

    let sets = game_data.split(";").map(|x| x.trim());
    let mut clean_sets = vec![];

    for set in sets {
      let mut clean_inputs = (0, 0, 0);
      for inputs in set.split(",") {
        let inputs: Vec<&str> = inputs.trim().split(" ").collect();
        match inputs[1] {
          "red" => {
            let num = inputs[0].parse::<u64>().unwrap();
            clean_inputs.0 += num;
          },
          "green" => {
            let num = inputs[0].parse::<u64>().unwrap();
            clean_inputs.1 += num;
          },
          "blue" => {
            let num = inputs[0].parse::<u64>().unwrap();
            clean_inputs.2 += num;
          },
          _ => {},
        }
      }

      clean_sets.push(clean_inputs);
    }

    entries.push(Entry { id: game_id, sets: clean_sets });
  }

  entries
}