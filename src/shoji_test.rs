use crate::*;

#[test]
fn it_works() {
    let mut shoji = Shoji::new();

    let child = shoji
        .new_node(
            LayoutStyle {
                ..Default::default()
            },
            vec![],
        )
        .unwrap();

    let node = shoji
        .new_node(
            LayoutStyle {
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            vec![child],
        )
        .unwrap();

    shoji.compute_layout(node, Size::undefined()).unwrap();
    dbg!(shoji.layout(node).unwrap());
}
