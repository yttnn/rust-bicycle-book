use toy_vec::ToyVec;

fn main() {
  let mut v = ToyVec::new();
  v.push("Java Finch".to_string());
  v.push("Budgerigar".to_string());

  let mut iter = v.iter();

  //v.push("Hill Mynah".to_string());
  // error: cannot borrow `v` as mutable because it is also borrowed as immutable

  assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
  v.push("Canary".to_string());
}