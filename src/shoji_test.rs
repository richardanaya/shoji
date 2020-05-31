use crate::*;
use alloc::vec::Vec;

#[test]
fn it_works() -> Result<(), &'static str> {
    let mut shoji = Shoji::new();

    let top_child = shoji.new_node(
        LayoutStyle {
            ..Default::default()
        },
        Vec::new(),
    );

    let bottom_left_child = shoji.new_node(
        LayoutStyle {
            ..Default::default()
        },
        Vec::new(),
    );

    let bottom_right_child = shoji.new_node(
        LayoutStyle {
            ..Default::default()
        },
        Vec::new(),
    );

    let bottom_child = shoji.new_node(
        LayoutStyle {
            ..Default::default()
        },
        {
            let mut v = Vec::new();
            v.push(bottom_left_child);
            v.push(bottom_right_child);
            v
        },
    );

    let root = shoji.new_node(
        LayoutStyle {
            direction: Direction::TopBottom,
            ..Default::default()
        },
        {
            let mut v = Vec::new();
            v.push(top_child);
            v.push(bottom_child);
            v
        },
    );

    shoji.compute_layout(root, LayoutSize::new(100.0, 100.0))?;
    assert_eq!(
        shoji.get_layout(root)?,
        &Layout {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 100.0,
        }
    );

    assert_eq!(
        shoji.get_layout(top_child)?,
        &Layout {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 50.0,
        }
    );

    assert_eq!(
        shoji.get_layout(bottom_child)?,
        &Layout {
            x: 0.0,
            y: 50.0,
            w: 100.0,
            h: 50.0,
        }
    );

    assert_eq!(
        shoji.get_layout(bottom_left_child)?,
        &Layout {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        }
    );

    assert_eq!(
        shoji.get_layout(bottom_right_child)?,
        &Layout {
            x: 50.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        }
    );

    Ok(())
}
