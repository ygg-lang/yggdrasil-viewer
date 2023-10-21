#[cfg(test)]
mod tests;

use crate::TidyNode;

pub struct Traverse<'a> {
    nodes: Vec<&'a TidyNode>,
}

impl<'a> Iterator for Traverse<'a> {
    type Item = &'a TidyNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop()
    }
}

fn recursive_iter<'a>(node: &'a TidyNode, nodes: &mut Vec<&'a TidyNode>) {
    nodes.push(node);
    for child in node.children.iter() {
        recursive_iter(child, nodes);
    }
}

impl TidyNode {
    #[inline]
    pub fn iter(&self) -> Traverse {
        let mut nodes = Vec::new();
        recursive_iter(self, &mut nodes);
        nodes.reverse();
        Traverse { nodes }
    }
}
