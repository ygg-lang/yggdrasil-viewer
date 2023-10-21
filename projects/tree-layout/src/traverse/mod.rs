#[cfg(test)]
mod tests;

use crate::LayoutNode;

pub struct Traverse<'a> {
    nodes: Vec<&'a LayoutNode>,
}

impl<'a> Iterator for Traverse<'a> {
    type Item = &'a LayoutNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop()
    }
}

fn recursive_iter<'a>(node: &'a LayoutNode, nodes: &mut Vec<&'a LayoutNode>) {
    nodes.push(node);
    for child in node.children.iter() {
        recursive_iter(child, nodes);
    }
}

impl LayoutNode {
    #[inline]
    pub fn iter(&self) -> Traverse {
        let mut nodes = Vec::new();
        recursive_iter(self, &mut nodes);
        nodes.reverse();
        Traverse { nodes }
    }
}
