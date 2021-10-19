# Rust By Example

This repo will house the examples found in The Rust Programming Language book & personal implementations of the practice problems found in documentation

```rust
use std::fmt;

struct Owner {
    name: String,
    exp: u8,
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{0} has < {1} months of experience", self.name, self.exp)
    }
}

fn main() {
   let name = String::from("");
   let exp = 1;
   let owner = Owner {name, exp};

   println!("{}", owner);
}

```
