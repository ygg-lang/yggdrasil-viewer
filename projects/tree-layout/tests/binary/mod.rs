use tree_layout::{TreeArena, TreeInfo};

use super::*;

#[derive(Clone, Debug)]
pub struct BinaryTree {
    root: BinaryNode,
}

#[derive(Clone, Debug)]
pub struct BinaryNode {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl BinaryTree {
    pub fn random(seed: u64, p: f64) -> Self {
        Self { root: BinaryNode::random(seed, p) }
    }
}

impl BinaryNode {
    pub fn random(seed: u64, p: f64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        Self { left: Self::new_with_state(&mut rng, p), right: Self::new_with_state(&mut rng, p) }
    }
    fn new_with_state(rng: &mut impl Rng, p: f64) -> Option<Box<Self>> {
        if rng.gen_bool(p) {
            Some(Box::new(Self { left: Self::new_with_state(rng, p), right: Self::new_with_state(rng, p) }))
        }
        else {
            None
        }
    }
}

impl TreeInfo for BinaryTree {
    type Node = BinaryNode;

    fn root(&self) -> &Self::Node {
        &self.root
    }

    fn children<'a>(&self, node: &'a Self::Node) -> impl Iterator<Item = &'a Self::Node> {
        node.left.iter().chain(node.right.iter()).map(move |x| &**x)
    }
}

#[test]
fn test() {
    let tree = BinaryTree::random(10086, 0.4);
    let arena = TreeArena::build(&BinaryTree::random(224, 0.4), &LayoutConfig::new(20.0, 20.0));
    println!("{:?}", tree);
    for i in arena.into_iter() {
        println!("{:?}", i);
    }
}
