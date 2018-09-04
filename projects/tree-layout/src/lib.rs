#![feature(return_position_impl_trait_in_trait)]
mod tree;

use shape_core::Point;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy, Debug)]
pub struct TreeBox {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl TreeBox {
    pub fn all(size: f64) -> TreeBox {
        TreeBox { top: size, right: size, bottom: size, left: size }
    }
}

pub trait NodeInfo<N>
where
    Self::Key: Eq + Hash,
    N: Copy,
{
    type Key;

    /// Returns a key that will be used to uniquely identify a given node.
    fn key(&self, node: N) -> Self::Key;

    /// Returns the children that a given node has.
    fn children(&self, node: N) -> impl Iterator<Item = N>;

    /// Returns the dimensions of a given node.
    ///
    /// This is the padding that you want around the centre point of the node so that you can line
    /// things up as you want to (e.g. nodes aligned by their top border vs being aligned by their
    /// centres).
    ///
    /// This value is generic over units (but all nodes must use the same unit) and the layout that
    /// this crate calculates will be given in terms of this unit. For example if you give this
    /// value in pixels then the layout will be given in terms of number of pixels from the left of
    /// the tree. Alternatively you might want to give this value in terms of the proportion of the
    /// width of your window (though note that this does not guarantee that the tree will fit in
    /// your window).
    ///
    /// # Default
    ///
    /// By default the algorithm assumes that each node is point-like (i.e. has no width or height).
    fn dimensions(&self, node: N) -> TreeBox {
        TreeBox::all(0.0)
    }

    /// Returns the desired border around a given node.
    ///
    /// See the `dimensions` method for a description of what units this has.
    ///
    /// # Default
    ///
    /// By default the algorithm assumes that each node has a border of `0.5` on every side.
    fn border(&self, node: N) -> TreeBox {
        TreeBox::all(0.5)
    }
}

#[derive(Clone, Debug)]
struct TreeData<K> {
    pub key: K,
    x: f64,
    y: f64,
    modifier: f64,
    dimensions: TreeBox,
    border: TreeBox,
}

impl<K> TreeData<K> {
    fn top_space(&self) -> f64 {
        self.dimensions.top + self.border.top
    }

    #[allow(dead_code)]
    fn top(&self) -> f64 {
        self.y - self.top_space()
    }

    fn bottom_space(&self) -> f64 {
        self.dimensions.bottom + self.border.bottom
    }

    fn bottom(&self) -> f64 {
        self.y + self.bottom_space()
    }

    fn left_space(&self) -> f64 {
        self.dimensions.left + self.border.left
    }

    fn left(&self) -> f64 {
        self.x - self.left_space()
    }

    fn right_space(&self) -> f64 {
        self.dimensions.right + self.border.right
    }

    fn right(&self) -> f64 {
        self.x + self.right_space()
    }
}

/// Returns the coordinates for the _centre_ of each node.
///
/// The origin of the coordinate system will be at the top left of the tree. The coordinates take
/// into account the width of the left-most node and shift everything so that the left-most border
/// of the left-most node is at 0 on the x-axis.
///
/// # Important
///
/// This algorithm _does_ account for the height of nodes but this is only to allow each row of
/// nodes to be aligned by their centre. If your tree has some nodes at a given depth which are
/// significantly larger than others and you want to avoid large gaps between rows then a more
/// general graph layout algorithm is required.
pub fn layout<N, T>(tree: &T, root: N) -> HashMap<T::Key, Point<f64>>
where
    N: Copy,
    T: NodeInfo<N>,
{
    let mut tree = tree::Tree::new(tree, root, |t, n| TreeData {
        key: t.key(n),

        x: 0.0,
        y: 0.0,
        modifier: 0.0,

        dimensions: t.dimensions(n),
        border: t.border(n),
    });

    if let Some(root) = tree.root() {
        initialise_y(&mut tree, root);

        initialise_x(&mut tree, root);
        ensure_positive_x(&mut tree, root);
        finalise_x(&mut tree, root);

        tree.0.into_iter().map(|tree::TreeNode { data: d, .. }| (d.key, Point { x: d.x, y: d.y })).collect()
    }
    else {
        HashMap::new()
    }
}

fn initialise_y<K>(tree: &mut tree::Tree<TreeData<K>>, root: usize) {
    let mut next_row = MediumVec::from_elem(root, 1);

    while !next_row.is_empty() {
        let row = next_row;
        next_row = MediumVec::new();

        let mut max = -std::f64::INFINITY;
        for node in &row {
            let node = *node;

            tree[node].data.y = if let Some(parent) = tree[node].parent { tree[parent].data.bottom() } else { 0.0 }
                + tree[node].data.top_space();

            if tree[node].data.y > max {
                max = tree[node].data.y;
            }

            next_row.extend_from_slice(&tree[node].children);
        }

        for node in &row {
            tree[*node].data.y = max;
        }
    }
}

fn initialise_x<K>(tree: &mut tree::Tree<TreeData<K>>, root: usize) {
    for node in tree.post_order(root) {
        if tree[node].is_leaf() {
            tree[node].data.x = if let Some(sibling) = tree.previous_sibling(node) { tree[sibling].data.right() } else { 0.0 }
                + tree[node].data.left_space();
        }
        else {
            let mid = {
                let first = tree[*tree[node].children.first().expect("Only leaf nodes have no children.")].data.x;
                let last = tree[*tree[node].children.last().expect("Only leaf nodes have no children.")].data.x;

                (first + last) / 2.0
            };

            if let Some(sibling) = tree.previous_sibling(node) {
                tree[node].data.x = tree[sibling].data.right() + tree[node].data.left_space();
                tree[node].data.mod_ = tree[node].data.x - mid;
            }
            else {
                tree[node].data.x = mid;
            }

            fix_overlaps(tree, node);
        }
    }
}

fn fix_overlaps<K>(tree: &mut tree::Tree<TreeData<K>>, right: usize) {
    fn max_depth(l: &HashMap<usize, f64>, r: &HashMap<usize, f64>) -> usize {
        if let Some(l) = l.keys().max() {
            if let Some(r) = r.keys().max() {
                return std::cmp::min(*l, *r);
            }
        }

        0
    }

    let right_node_contour = left_contour(tree, right);

    for left in tree.left_siblings(right) {
        let left_node_contour = right_contour(tree, left);
        let mut shift = 0.0;

        for depth in tree[right].depth..=max_depth(&right_node_contour, &left_node_contour) {
            let gap = right_node_contour[&depth] - left_node_contour[&depth];
            if gap + shift < 0.0 {
                shift = -gap;
            }
        }

        tree[right].data.x += shift;
        tree[right].data.modifier += shift;

        centre_nodes_between(tree, left, right);
    }
}

fn left_contour<K>(tree: &tree::Tree<TreeData<K>>, node: usize) -> HashMap<usize, f64> {
    contour(tree, node, min, |n| n.data.left())
}

fn right_contour<K>(tree: &tree::Tree<TreeData<K>>, node: usize) -> HashMap<usize, f64> {
    contour(tree, node, max, |n| n.data.right())
}

fn min<T: std::cmp::PartialOrd>(l: T, r: T) -> T {
    if l < r { l } else { r }
}

fn max<T: std::cmp::PartialOrd>(l: T, r: T) -> T {
    if l > r { l } else { r }
}

fn contour<C, E, K>(tree: &tree::Tree<TreeData<K>>, node: usize, cmp: C, edge: E) -> HashMap<usize, f64>
where
    C: Fn(f64, f64) -> f64,
    E: Fn(&tree::TreeNode<TreeData<K>>) -> f64,
{
    let mut stack = MediumVec::from_elem((0.0, node), 1);
    let mut contour = HashMap::new();

    while let Some((mod_, node)) = stack.pop() {
        let depth = tree[node].depth;
        let shifted = edge(&tree[node]) + mod_;
        let new = if let Some(current) = contour.get(&depth) { cmp(*current, shifted) } else { shifted };
        let mod_ = mod_ + tree[node].data.modifier;

        contour.insert(depth, new);
        stack.extend(tree[node].children.iter().map(|c| (mod_, *c)));
    }

    contour
}

fn centre_nodes_between<K>(tree: &mut tree::Tree<TreeData<K>>, left: usize, right: usize) {
    let num_gaps = tree[right].order - tree[left].order;

    let space_per_gap = (tree[right].data.left() - tree[left].data.right()) / (num_gaps as f64);

    for (i, sibling) in tree.siblings_between(left, right).into_iter().enumerate() {
        let i = i + 1;

        let old_x = tree[sibling].data.x;
        // HINT: We traverse the tree in post-order so we should never be moving anything to the
        //       left.
        // TODO: Have some kind of `move_node` method that checks things like this?
        let new_x = max(old_x, tree[left].data.right() + space_per_gap * (i as f64));
        let diff = new_x - old_x;

        tree[sibling].data.x = new_x;
        tree[sibling].data.modifier += diff;
    }
}

fn ensure_positive_x<K>(tree: &mut tree::Tree<TreeData<K>>, root: usize) {
    let contour = left_contour(tree, root);
    let shift = -contour
        .values()
        .fold(None, |acc, curr| {
            let acc = acc.unwrap_or(std::f64::INFINITY);
            let curr = *curr;
            Some(if curr < acc { curr } else { acc })
        })
        .unwrap_or(0.0);

    tree[root].data.x += shift;
    tree[root].data.modifier += shift;
}

fn finalise_x<K>(tree: &mut tree::Tree<TreeData<K>>, root: usize) {
    for node in tree.breadth_first(root) {
        let shift = if let Some(parent) = tree[node].parent { tree[parent].data.mod_ } else { 0.0 };

        tree[node].data.x += shift;
        tree[node].data.mod_ += shift;
    }
}
