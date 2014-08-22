#[test]
fn prints_pair() {
  let pair = (51i, true);

  print!("Tell me about {}", pair);
}

#[test]
fn if_bool_true_int_20_to_26() {
  let pair = (51i, true);

  match pair {
    (20..26, true) => { println!("True and between 20-26"); }
    (_,_) => { println!("True or False and not between 20-26"); }
  }
}
