use crate::*;

#[test]
fn it_works() -> Result<(), &'static str>{
    let mut shoji = Shoji::new();

    let top_child = shoji
        .new_node(
            LayoutStyle {
                ..Default::default()
            },
            vec![],
        )?;

    let bottom_child = shoji
        .new_node(
            LayoutStyle {
                ..Default::default()
            },
            vec![],
        )?;

    let root = shoji
        .new_node(
            LayoutStyle {
                direction: Direction::TopBottom,
                ..Default::default()
            },
            vec![top_child, bottom_child],
        )?;

    shoji
        .compute_layout(root, LayoutSize::new(100.0, 100.0))?;
    assert_eq!(shoji.layout(root)?,&Layout {
        x:0.0,
        y:0.0,
        w:100.0,
        h: 100.0,
    });
    dbg!(shoji.layout(root)?);
    Ok(())
}
