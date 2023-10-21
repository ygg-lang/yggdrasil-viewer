use crate::{LayoutConfig, LayoutNode, Traverse, TreeInfo};

pub struct TreeArena<'i, T: TreeInfo> {
    arena: Vec<&'i T::Node>,
    root: LayoutNode,
    tree: std::marker::PhantomData<T>,
}

impl<'i, T: TreeInfo> TreeArena<'i, T> {
    pub fn build(tree: &T, layout: &LayoutConfig) -> Self {
        let mut out = Self { arena: Vec::with_capacity(tree.count()), root: Default::default(), tree: Default::default() };
        out.root = out.insert_node(tree.root().as_ref(), tree);
        let mut config = layout.clone();
        config.layout(&mut out.root);
        out
    }
    fn insert_node(&mut self, parent: &T::Node, tree: &T) -> LayoutNode {
        let mut node = LayoutNode::new(self.arena.len(), tree.width(parent), tree.height(parent));
        for child in tree.children(parent) {
            node.append_child(self.insert_node(child.as_ref(), tree));
        }
        node
    }
}

pub struct ArenaIterator<'i, T: TreeInfo> {
    pool: &'i [&'i T::Node],
    iter: Traverse<'i>,
}

impl<'a, 'i, T: TreeInfo> IntoIterator for &'a TreeArena<'i, T> {
    type Item = (&'a LayoutNode, &'a T::Node);
    type IntoIter = ArenaIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIterator { pool: self.arena.as_slice(), iter: self.root.iter() }
    }
}

impl<'i, T: TreeInfo> Iterator for ArenaIterator<'i, T> {
    type Item = (&'i LayoutNode, &'i T::Node);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let data = self.pool.get(item.id)?;
        Some((item, data))
    }
}
