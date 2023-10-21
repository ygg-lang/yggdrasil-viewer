use crate::{LayoutConfig, LayoutNode, Traverse, TreeInfo};

pub struct TreeArena<T: TreeInfo> {
    arena: Vec<T::Node>,
    root: LayoutNode,
    tree: std::marker::PhantomData<T>,
}

impl<T: TreeInfo> TreeArena<T> {
    pub fn build(tree: T, layout: &LayoutConfig) -> Self {
        let mut out = Self { arena: Vec::with_capacity(tree.count()), root: Default::default(), tree: Default::default() };
        out.root = out.insert_node(tree.root(), &tree);
        let mut config = layout.clone();
        config.layout(&mut out.root);
        out
    }
    fn insert_node(&mut self, parent: T::Node, tree: &T) -> LayoutNode {
        let mut node = LayoutNode::new(self.arena.len(), tree.width(&parent), tree.height(&parent));
        for child in tree.children(&parent) {
            node.append_child(self.insert_node(child, tree));
        }
        self.arena.push(parent);
        node
    }
}

pub struct ArenaIterator<'i, T: TreeInfo> {
    pool: &'i [T::Node],
    iter: Traverse<'i>,
}

impl<'i, T: TreeInfo> IntoIterator for &'i TreeArena<T> {
    type Item = (&'i LayoutNode, &'i T::Node);
    type IntoIter = ArenaIterator<'i, T>;

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
