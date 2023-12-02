pub mod utils;
pub mod task1;
pub mod task2;

fn main() {
  let entries = utils::clean_data();
  println!("{}", task1::task1(&entries));
  println!("{}", task2::task2(&entries));
}
