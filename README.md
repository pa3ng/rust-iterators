# `fizz-buzz` iterator demo

Unlike Go, Rust's support for iterators, ranges, etc. is pretty good.

Take a look at [`fizz-buzz`](src/main.rs) written using iterators (example comes from _Programming Rust_)

One minor warning: Rust's iterators are lazy by default and must be consumed. Here's an example:

```rust
use std::thread::sleep;
use std::time::Duration;

fn main() {
    (0..100).map(|i| sleep(Duration::from_secs(i)));
}
```

The following will compile with the following warning:

```
warning: unused `std::iter::Map` that must be used
 --> lazy.rs::5:5
  |
5 |     (0..100).map(|i| sleep(Duration::from_secs(i)));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
  = note: iterators are lazy and do nothing unless consumed
```

It will also execute immediately. To "consume" an iterator, loop over it, `collect()` it, or execute another operation which consumes it. Here we use `last()`:

```rust
use std::thread::sleep;
use std::time::Duration;

fn main() {
    (0..100).map(|i| sleep(Duration::from_secs(i))).last();
}
```

This will compile without warnings, and will take a long time to execute.