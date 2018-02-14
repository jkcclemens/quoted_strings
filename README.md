# quoted_strings

If you need to process strings that may contain quotes and the strings are delimited by spaces,
this is the library for you!

```rust
extern crate quoted_strings;

use quoted_strings::QuotedParts;

fn main() {
  let string = r#"
    This sure is some "sample text," isn't it?
  "#;

  for part in QuotedParts::from(string) {
    println!("{}", part);
  }

  // This
  // sure
  // is
  // some
  // sample text,
  // isn't
  // it?
}
```
