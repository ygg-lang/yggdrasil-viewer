/// A vector backed tree implementation used to essentially cache the user's tree.
use crate::NodeInfo;
use std::vec::IntoIter;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeNode<D> {
    /// Data needed by the actual algorithm.
    pub data: D,

    /// The position of this node among it's siblings.
    ///
    /// Can also be thought of as the number of left-siblings this node has.
    pub order: usize,
    /// The depth of this node.
    ///
    /// Can be thought of as the number of edges between this node and the root node.
    pub depth: usize,

    /// The index into `Tree` of this node's parent.
    pub parent: Option<usize>,
    /// The indices into `Tree` of this node's children.
    pub children: Vec<usize>,
}

impl<D> TreeNode<D> {
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeLayout<D> {
    arena: Vec<TreeNode<D>>,
}

impl<D> Default for TreeLayout<D> {
    fn default() -> Self {
        Self { arena: vec![] }
    }
}

impl<D> TreeLayout<D> {
    pub fn new<F, N, T>(user_tree: &T, root: N, data: F) -> Self
    where
        F: Fn(&T, N) -> D,
        N: Clone,
        T: NodeInfo<N>,
    {
        let mut tree = Vec::new();
        tree.push(TreeNode { data: data(user_tree, root.clone()), order: 0, depth: 0, parent: None, children: Vec::new() });

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, root));

        while let Some((parent, node)) = queue.pop_front() {
            let index = tree.len();

            for (i, child) in user_tree.children(node).into_iter().enumerate() {
                let index = index + i;
                let depth = tree[parent].depth + 1;

                tree[parent].children.push(index);

                tree.push(TreeNode {
                    data: data(user_tree, child.clone()),

                    order: i,
                    depth,

                    parent: Some(parent),
                    children: Vec::new(),
                });

                queue.push_back((index, child));
            }
        }

        TreeLayout { arena: tree }
    }

    pub fn root(&self) -> Option<usize> {
        if self.arena.is_empty() { None } else { Some(0) }
    }

    pub fn breadth_first(&self, node: usize) -> Vec<usize> {
        let mut breadth_first = vec![node];
        let mut index = 0;

        while index < breadth_first.len() {
            let node = breadth_first[index];
            breadth_first.extend_from_slice(&self[node].children);
            index += 1;
        }

        breadth_first
    }

    pub fn post_order(&self, node: usize) -> Vec<usize> {
        let mut breadth_first = vec![node];
        let mut post_order = Vec::new();

        while let Some(node) = breadth_first.pop() {
            breadth_first.extend_from_slice(&self[node].children);
            post_order.push(node);
        }

        post_order.reverse();
        post_order
    }

    pub fn left_siblings(&self, node: usize) -> Vec<usize> {
        let order = self[node].order;

        if let Some(parent) = self[node].parent { self[parent].children[0..order].into() } else { Vec::new() }
    }

    pub fn siblings_between(&self, left: usize, right: usize) -> Vec<usize> {
        let left_order = self[left].order;
        let right_order = self[right].order;

        if self[left].is_root() || self[right].is_root() {
            assert!(self[left].is_root(), "If one node is the root then both nodes must be.");
            assert!(self[right].is_root(), "If one node is the root then both nodes must be.");

            return Vec::new();
        }

        let left_parent = self[left].parent.expect("`is_none` has already been checked.");

        let right_parent = self[right].parent.expect("`is_none` has already been checked.");

        assert_eq!(left_parent, right_parent, "Nodes must actually be siblings.");

        let parent = left_parent;

        self[parent].children[left_order + 1..right_order].into()
    }

    pub fn previous_sibling(&self, node: usize) -> Option<usize> {
        let order = self[node].order;
        if order == 0 {
            return None;
        }

        let parent = self[node].parent.expect("Nodes where `order != 0` always have parents.");

        Some(self[parent].children[order - 1])
    }
}

impl<D> IntoIterator for TreeLayout<D> {
    type Item = TreeNode<D>;
    type IntoIter = IntoIter<TreeNode<D>>;

    fn into_iter(self) -> Self::IntoIter {
        self.arena.into_iter()
    }
}

impl<D> std::ops::Index<usize> for TreeLayout<D> {
    type Output = TreeNode<D>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arena[index]
    }
}

impl<D> std::ops::IndexMut<usize> for TreeLayout<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arena[index]
    }
}
