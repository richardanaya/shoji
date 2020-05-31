# shoji

<a href="https://docs.rs/shoji"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A vbox/hbox layout library implemented in Rust.

```rust
[dependencies]
shoji = "0.1"
```

# Example

```rust
use shoji::*;

fn main() -> Result<(), &'static str> {
    let mut shoji = Shoji::new();

    // Put together layout
    let top = shoji.new_node(LayoutStyle::default(),vec![]);

    let bottom_left = shoji.new_node(LayoutStyle::default(),vec![]);
    let bottom_right = shoji.new_node(LayoutStyle::default(),vec![]);

    let bottom = shoji.new_node(
        LayoutStyle { direction: Direction:LeftRight },
        vec![bottom_left, bottom_right],
    );

    let root = shoji.new_node(
        LayoutStyle { direction: Direction:TopDown },
        vec![top, bottom],
    );

    // Compute all layouts
    shoji.compute_layout(root, LayoutSize::new(100.0, 100.0))?;

    // Get the calculated layout
    dbg!(shoji.layout(top)?);
}
```

Calculations will give absolute values:

<p align="center">
  <img width="460" src="shoji.png">
</p>

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
