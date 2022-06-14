[![crates.io](https://img.shields.io/crates/v/unmem.svg)](https://crates.io/crates/unmem)
[![License](https://img.shields.io/crates/l/unmem.svg)](https://choosealicense.com/licenses/mpl-2.0/)
[![Documentation](https://img.shields.io/docsrs/unmem/latest)](https://docs.rs/unmem)

# unmem

Some memory working functions. Not marked as unsafe BUT STILL ARE.

Examples:
```rust
extern crate unmem;
use unmem::change;

fn main() {
    let a: u8 = 6;
    change(&a, 255);
    println!("{a}"); // >> 255
}
```

```rust
extern crate unmem;
use unmem::get_mut;

fn main() {
    let a: u8 = 15;
    let mut mut_a = get_mut(&a);
    *mut_a = 8;
    println!("{a}"); // >> 8
}
```

# Don't deploy this shit to production you madman!
