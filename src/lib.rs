use generational_arena::{Arena, Index};

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

#[derive(Debug)]
pub struct Layout {}

pub struct Node {
    layout: Option<Layout>,
    style: LayoutStyle,
    children: Vec<Index>,
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

    pub fn new_node(
        &mut self,
        style: LayoutStyle,
        children: Vec<Index>,
    ) -> Result<Index, &'static str> {
        Ok(self.nodes.insert(Node {
            layout: None,
            style,
            children,
        }))
    }

    pub fn get_node(&mut self,node_index: Index) -> Result<&mut Node,&'static str> {
        Ok(&mut self.nodes[node_index])
    }

    pub fn layout(&self, i: Index) -> Result<&Layout, &'static str> {
        match &self.nodes[i].layout {
            Some(l) => Ok(l),
            None => Err("layout has not been calculated yet"),
        }
    }

    pub fn compute_layout(&mut self, node_index: Index, s: LayoutSize) -> Result<(), &'static str> {
        let node = self.get_node(node_index)?;
        let children = node.children.clone();
        match node.style.direction {
            Direction::LeftRight => {
                let width = s.width;
                let height = s.height;
                match width {
                    Some(w) => {
                        let child_width = w/children.len() as f64;
                        for c in children.iter() {
                            self.compute_layout(*c, LayoutSize{
                                width:Some(child_width),
                                height
                            })?;
                        }
                        Ok(())
                    },
                    None => {
                        Err("cannot compute layout of LeftRight without defined width")
                    }
                }
            },
            Direction::TopBottom => {
                let width = s.width;
                let height = s.height;
                match height {
                    Some(h) => {
                        let child_height = h/children.len() as f64;
                        for c in children.iter() {
                            self.compute_layout(*c, LayoutSize{
                                width,
                                height:Some(child_height)
                            })?;
                        }
                        Ok(())
                    },
                    None => {
                        Err("cannot compute layout of TopBottom without defined width")
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod shoji_test;
