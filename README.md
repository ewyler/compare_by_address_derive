Defines `PartialEq`, `Eq`, and `Hash` to compare the given struct/union/enum by
address to other instances.

# Example usage

```
#[macro_use]
extern crate compare_by_address_derive;

#[derive(CompareByAddress)]
struct SomeStruct {
  // ...
}
```
