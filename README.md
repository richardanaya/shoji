# shoji

<a href="https://docs.rs/shoji"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A vbox/hbox layout library implemented in Rust.

```rust
[dependencies]
shoji = "0.0"
```

# Example

```rust
use shoji::*;

fn main() -> Result<(), &'static str> {
    let mut shoji = Shoji::new();
    
    let child = shoji.new_node(
        LayoutStyle { ..Default::default() },
        vec![],
    )?;

    let node = shoji.new_node(
        LayoutStyle {
            direction: Direction::TopBottom,
            ..Default::default()
        },
        vec![child],
    )?;

    shoji.compute_layout(node, Size::undefined())?;
    dbg!(shoji.layout(node)?);
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `shoji` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
