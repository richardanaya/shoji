#![no_std]
extern crate alloc;
use alloc::vec::Vec;

use generational_arena::{Arena, Index};

pub type NodeIndex = Index;

pub struct LayoutStyle {
    pub direction: Direction,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        LayoutStyle {
            direction: Direction::LeftRight,
        }
    }
}

pub enum Direction {
    TopBottom,
    LeftRight,
}

#[derive(Debug, PartialEq)]
pub struct Layout {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub struct Node {
    pub layout: Option<Layout>,
    pub style: LayoutStyle,
    pub children: Vec<NodeIndex>,
}

pub struct Shoji {
    nodes: Arena<Node>,
}

pub struct LayoutSize {
    width: Option<f64>,
    height: Option<f64>,
}

impl LayoutSize {
    pub fn new(w: f64, h: f64) -> LayoutSize {
        LayoutSize {
            width: Some(w),
            height: Some(h),
        }
    }
}

impl Shoji {
    pub fn new() -> Self {
        Shoji {
            nodes: Arena::new(),
        }
    }

    pub fn new_node(&mut self, style: LayoutStyle, children: Vec<NodeIndex>) -> NodeIndex {
        self.nodes.insert(Node {
            layout: None,
            style,
            children,
        })
    }

    pub fn get_node(&mut self, node_index: NodeIndex) -> &mut Node {
        &mut self.nodes[node_index]
    }

    pub fn get_layout(&self, i: NodeIndex) -> Result<&Layout, &'static str> {
        match &self.nodes[i].layout {
            Some(l) => Ok(l),
            None => Err("layout has not been calculated yet"),
        }
    }

    pub fn compute_layout(
        &mut self,
        node_index: NodeIndex,
        s: LayoutSize,
    ) -> Result<(), &'static str> {
        self.compute_layout_helper(0.0, 0.0, node_index, s)
    }

    fn compute_layout_helper(
        &mut self,
        x: f64,
        y: f64,
        node_index: NodeIndex,
        s: LayoutSize,
    ) -> Result<(), &'static str> {
        let node = self.get_node(node_index);
        node.layout = Some(Layout {
            x: x,
            y: y,
            w: s.width.ok_or("cannot create width from undefined value")?,
            h: s.height
                .ok_or("cannot create height from undefined value")?,
        });
        let children = node.children.clone();
        let num_children = children.len();
        if num_children == 0 {
            // do nothing
            Ok(())
        } else if num_children == 1 {
            self.compute_layout_helper(x, y, children[0], s)?;
            Ok(())
        } else {
            match node.style.direction {
                Direction::LeftRight => {
                    let width = s.width;
                    let height = s.height;
                    match width {
                        Some(w) => {
                            let child_width = w / children.len() as f64;
                            for (i, c) in children.iter().enumerate() {
                                self.compute_layout_helper(
                                    x + i as f64 * child_width,
                                    y,
                                    *c,
                                    LayoutSize {
                                        width: Some(child_width),
                                        height,
                                    },
                                )?;
                            }
                            Ok(())
                        }
                        None => Err("cannot compute layout of LeftRight without defined width"),
                    }
                }
                Direction::TopBottom => {
                    let width = s.width;
                    let height = s.height;
                    match height {
                        Some(h) => {
                            let child_height = h / children.len() as f64;
                            for (i, c) in children.iter().enumerate() {
                                self.compute_layout_helper(
                                    x,
                                    y + i as f64 * child_height,
                                    *c,
                                    LayoutSize {
                                        width,
                                        height: Some(child_height),
                                    },
                                )?;
                            }
                            Ok(())
                        }
                        None => Err("cannot compute layout of TopBottom without defined width"),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod shoji_test;
