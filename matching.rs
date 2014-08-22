fn main() {
  let pair = (51i, true);

  print!("Tell me about {} ", pair);

  match pair {
    // if the bool is true and the int is between 20 and 26
    (20..26, true) => { println!("True and between 20-26."); }

    // if the bool is true and the above isn't true for the int
    (x, true) if x != 20|21|22|23|24|25|26 => { println!("True and not between 20-26."); }

    // if bool is true or false and the int is between 40 and 49
    (40..49, _) => { println!("True or False and between 40-49."); }

    // none of the previous conditions are true.
    (_,_) => { println!("None of the conditions are true."); }
  }
}
