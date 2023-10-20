use alloc::{
    collections::VecDeque,
    vec,
    vec::{IntoIter, Vec},
};
use core::ops::{Index, IndexMut};

use hashbrown::HashMap;
use shape_core::{Point, Rectangle};

use crate::NodeInfo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeLayout<D> {
    arena: Vec<TreeNode<D>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeNode<D> {
    pub data: D,
    pub order: usize,
    pub depth: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct TreeData<K> {
    key: K,
    position: Point<f64>,
    module: f64,
    dimensions: Rectangle<f64>,
    border: Rectangle<f64>,
}

#[allow(dead_code)]
impl<K> TreeData<K> {
    fn top_space(&self) -> f64 {
        -self.dimensions.min.y - self.border.min.y
    }
    fn top(&self) -> f64 {
        self.position.y - self.top_space()
    }
    fn bottom_space(&self) -> f64 {
        self.dimensions.max.y + self.border.max.y
    }
    fn bottom(&self) -> f64 {
        self.position.y + self.bottom_space()
    }
    fn left_space(&self) -> f64 {
        -self.dimensions.min.x - self.border.min.x
    }
    fn left(&self) -> f64 {
        self.position.x - self.left_space()
    }
    fn right_space(&self) -> f64 {
        self.dimensions.max.x + self.border.max.x
    }
    fn right(&self) -> f64 {
        self.position.x + self.right_space()
    }
}

impl<D> TreeNode<D> {
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

pub fn layout<N, T>(tree: &T, root: &N) -> TreeLayout<TreeData<<T as NodeInfo<N>>::Index>>
    where
        T: NodeInfo<N>,
        N: Clone,
{
    let mut tree = TreeLayout::new(tree, root.clone(), |t, n| TreeData {
        key: t.query(n.clone()),
        position: Point::new(0.0, 0.0),
        module: 0.0,
        dimensions: t.dimensions(n.clone()),
        border: t.border(n),
    });
    if let Some(root) = tree.root() {
        initialise_y(&mut tree, root);
        initialise_x(&mut tree, root);
        ensure_positive_x(&mut tree, root);
        finalise_x(&mut tree, root);
        tree
    } else {
        Default::default()
    }
}

pub fn layout_position<N, T>(tree: &T, root: &N) -> HashMap<T::Index, Point<f64>>
    where
        T: NodeInfo<N>,
        N: Clone
{
    let layout = layout(tree, root);
    layout.into_iter().map(|TreeNode { data: d, .. }| (d.key, Point { x: d.position.x, y: d.position.y })).collect()
}


fn initialise_y<K>(tree: &mut TreeLayout<TreeData<K>>, root: usize) {
    let mut next_row = vec![root];
    while !next_row.is_empty() {
        let row = next_row;
        next_row = Vec::new();
        let mut max = f64::NEG_INFINITY;
        for node in &row {
            let node = *node;
            tree[node].data.position.y = if let Some(parent) = tree[node].parent { tree[parent].data.bottom() } else { 0.0 }
                + tree[node].data.top_space();
            if tree[node].data.position.y > max {
                max = tree[node].data.position.y;
            }
            next_row.extend_from_slice(&tree[node].children);
        }
        for node in &row {
            tree[*node].data.position.y = max;
        }
    }
}

fn initialise_x<K>(tree: &mut TreeLayout<TreeData<K>>, root: usize) {
    for node in tree.post_order(root) {
        if tree[node].is_leaf() {
            tree[node].data.position.x =
                if let Some(sibling) = tree.previous_sibling(node) { tree[sibling].data.right() } else { 0.0 }
                    + tree[node].data.left_space();
        } else {
            let mid = {
                let first = tree[*tree[node].children.first().expect("Only leaf nodes have no children.")].data.position.x;
                let last = tree[*tree[node].children.last().expect("Only leaf nodes have no children.")].data.position.x;

                (first + last) / 2.0
            };
            if let Some(sibling) = tree.previous_sibling(node) {
                tree[node].data.position.x = tree[sibling].data.right() + tree[node].data.left_space();
                tree[node].data.module = tree[node].data.position.x - mid;
            } else {
                tree[node].data.position.x = mid;
            }
            fix_overlaps(tree, node);
        }
    }
}

fn fix_overlaps<K>(tree: &mut TreeLayout<TreeData<K>>, right: usize) {
    fn max_depth(l: &HashMap<usize, f64>, r: &HashMap<usize, f64>) -> usize {
        if let Some(l) = l.keys().max() {
            if let Some(r) = r.keys().max() {
                return core::cmp::min(*l, *r);
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
        tree[right].data.position.x += shift;
        tree[right].data.module += shift;

        centre_nodes_between(tree, left, right);
    }
}

fn left_contour<K>(tree: &TreeLayout<TreeData<K>>, node: usize) -> HashMap<usize, f64> {
    contour(tree, node, min, |n| n.data.left())
}

fn right_contour<K>(tree: &TreeLayout<TreeData<K>>, node: usize) -> HashMap<usize, f64> {
    contour(tree, node, max, |n| n.data.right())
}

fn min<T: PartialOrd>(l: T, r: T) -> T {
    if l < r { l } else { r }
}

fn max<T: PartialOrd>(l: T, r: T) -> T {
    if l > r { l } else { r }
}

fn contour<C, E, K>(tree: &TreeLayout<TreeData<K>>, node: usize, cmp: C, edge: E) -> HashMap<usize, f64>
    where
        C: Fn(f64, f64) -> f64,
        E: Fn(&TreeNode<TreeData<K>>) -> f64,
{
    let mut stack = vec![(0.0, node)];
    let mut contour = HashMap::new();
    while let Some((m, node)) = stack.pop() {
        let depth = tree[node].depth;
        let shifted = edge(&tree[node]) + m;
        let new = if let Some(current) = contour.get(&depth) { cmp(*current, shifted) } else { shifted };
        let module = m + tree[node].data.module;
        contour.insert(depth, new);
        stack.extend(tree[node].children.iter().map(|c| (module, *c)));
    }
    contour
}

fn centre_nodes_between<K>(tree: &mut TreeLayout<TreeData<K>>, left: usize, right: usize) {
    let num_gaps = tree[right].order - tree[left].order;
    let space_per_gap = (tree[right].data.left() - tree[left].data.right()) / (num_gaps as f64);
    for (i, sibling) in tree.siblings_between(left, right).into_iter().enumerate() {
        let i = i + 1;
        let old_x = tree[sibling].data.position.x;
        // HINT: We traverse the tree in post-order so we should never be moving anything to the
        //       left.
        // TODO: Have some kind of `move_node` method that checks things like this?
        let new_x = max(old_x, tree[left].data.right() + space_per_gap * (i as f64));
        let diff = new_x - old_x;
        tree[sibling].data.position.x = new_x;
        tree[sibling].data.module += diff;
    }
}

fn ensure_positive_x<K>(tree: &mut TreeLayout<TreeData<K>>, root: usize) {
    let contour = left_contour(tree, root);
    let shift = -contour
        .values()
        .fold(None, |acc, curr| {
            let acc = acc.unwrap_or(f64::INFINITY);
            let curr = *curr;
            Some(if curr < acc { curr } else { acc })
        })
        .unwrap_or(0.0);
    tree[root].data.position.x += shift;
    tree[root].data.module += shift;
}

fn finalise_x<K>(tree: &mut TreeLayout<TreeData<K>>, root: usize) {
    for node in tree.breadth_first(root) {
        let shift = if let Some(parent) = tree[node].parent { tree[parent].data.module } else { 0.0 };
        tree[node].data.position.x += shift;
        tree[node].data.module += shift;
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
        let mut queue = VecDeque::new();
        queue.push_back((0, root));
        while let Some((parent, node)) = queue.pop_front() {
            let index = tree.len();
            for (i, child) in user_tree.children(node.clone()).into_iter().enumerate() {
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
                queue.push_back((index, child.clone()));
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

impl<D> Default for TreeLayout<D> {
    fn default() -> Self {
        Self {
            arena: vec![],
        }
    }
}

impl<D> IntoIterator for TreeLayout<D> {
    type Item = TreeNode<D>;
    type IntoIter = IntoIter<TreeNode<D>>;

    fn into_iter(self) -> Self::IntoIter {
        self.arena.into_iter()
    }
}

impl<D> Index<usize> for TreeLayout<D> {
    type Output = TreeNode<D>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arena[index]
    }
}

impl<D> IndexMut<usize> for TreeLayout<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arena[index]
    }
}
