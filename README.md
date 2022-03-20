Defines `PartialEq`, `Eq`, and `Hash` to compare the given struct/union/enum by
address to other instances. This allows you to easily work with these items in
hashing contexts (HashSets, HashMaps, etc.) whereby equality is defined by
instance irrespective of content.

# Example usage

```
#[macro_use]
extern crate compare_by_address_derive;

#[derive(CompareByAddress)]
struct SomeStruct {
  // ...
}
```
