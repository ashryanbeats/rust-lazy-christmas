use std::string::String;
use std::io;

fn main() {

    let ordinals = ["first", "second", "third"];
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three french hens"];

    let day_idx = get_input(ordinals.len());

    println!("On the {} day of Christmas my true love gave to me {}.", ordinals[day_idx], gifts[day_idx]);
}

fn get_input(max_idx: usize) -> usize {
  loop {
    println!("Enter the day of Christmas you want a gift for.");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    match input.trim().parse::<usize>() {
      Ok(num) => {
        if num > 0 && num < max_idx + 1 {
          break num - 1
        } else {
          continue
        }
      },
      Err(_) => continue,
    };
  }
}