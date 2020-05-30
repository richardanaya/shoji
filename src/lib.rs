use generational_arena::{Arena, Index};

pub struct LayoutStyle {
    justify_content: JustifyContent,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        LayoutStyle {
            justify_content: JustifyContent::Center,
        }
    }
}

pub enum JustifyContent {
    Center,
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

pub struct Size {}

impl Size {
    fn undefined() -> Size {
        Size {}
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

    pub fn layout(&self, i: Index) -> Result<&Layout, &'static str> {
        match &self.nodes[i].layout {
            Some(l) => Ok(l),
            None => Err("layout has not been calculated yet"),
        }
    }

    pub fn compute_layout(&mut self, i: Index, s: Size) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod shoji_test;
