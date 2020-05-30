use crate::*;

#[test]
fn it_works() {
    let mut shoji = Shoji::new();

    let top_child = shoji
        .new_node(
            LayoutStyle {
                ..Default::default()
            },
            vec![],
        )
        .unwrap();

    let bottom_child = shoji
        .new_node(
            LayoutStyle {
                ..Default::default()
            },
            vec![],
        )
        .unwrap();

    let root = shoji
        .new_node(
            LayoutStyle {
                direction: Direction::TopBottom,
                ..Default::default()
            },
            vec![top_child, bottom_child],
        )
        .unwrap();

    shoji
        .compute_layout(root, LayoutSize::new(100.0, 100.0))
        .unwrap();
    dbg!(shoji.layout(root).unwrap());
}
