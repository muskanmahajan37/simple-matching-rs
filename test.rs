#[test]
fn prints_pair() {
  let pair = (51i, true);

  print!("Tell me about {}", pair);
}

#[test]
fn if_true_and_20_to_26() {
  let pair = (51i, true);

  match pair {
    (20..26, true) => { println!("True and between 20-26"); }
    (_,_) => { println!("True or False and not between 20-26"); }
  }
}

#[test]
fn if_true_and_not_20_to_26() {
  let pair = (51i, true);

  match pair {
    (x, true) if x != 20|21|22|23|24|25|26 => { println!("True and not between 20-26"); }
    (_,_) => { println!("True or False and not between 20-26"); }
  }
}
