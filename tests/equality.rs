use std::collections::HashSet;

#[macro_use]
extern crate compare_by_address_derive;

#[derive(CompareByAddress)]
struct Empty {}

#[test]
pub fn two_references_are_equal() {
  let mut set = HashSet::new();

  let one = &Empty {};
  let still_one = &one;
  let really_still_one = &still_one;

  set.insert(one);
  set.insert(still_one);
  set.insert(really_still_one);

  assert_eq!(1, set.len());
}

#[test]
pub fn two_identical_instances_are_not_equal() {
  let mut set = HashSet::new();

  // Don't do the following.
  // I'm not sure why yet (perhaps the optimizer?), but 
  // only a single Empty {} will be created and thus the addresses
  // will in fact be identical.
  // let one = &Empty {};
  // let two = &Empty {};

  let a = Empty {};
  let b = Empty {};
  let one = &a;
  let two = &b;

  set.insert(one);
  set.insert(two);

  assert_eq!(2, set.len());
}
