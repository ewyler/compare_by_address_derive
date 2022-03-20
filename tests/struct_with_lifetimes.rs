// This file will fail to compile if lifetimes aren't supported properly
// in the derive macro

#[macro_use]
extern crate compare_by_address_derive;

struct Poop {}

#[derive(CompareByAddress)]
#[allow(dead_code)]
struct Dog<'a, 'b> {
    poop: &'a Poop,
    poop2: &'b Poop,
}
