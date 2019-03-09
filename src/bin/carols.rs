

fn write_first_line(day: usize) {
  let days = ["first", "second", "third", "fourth", 
  "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];

  println!("On the {} day of Christmas my true love sent to me", days[day]);
}

fn write_list(day: usize) {
  let items = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
    ];
  
  for item in (0..day).rev() {
    println!("{}", items[item]);
  }
}

fn main() {
  println!("");
  for number in 0..12 {
    write_first_line(number);
    write_list(number + 1);
    println!("");
  }
}