use super::*;
use tree_layout::NodeInfo;

pub struct BinaryTree {
    root: BinaryNode,
}

#[derive(Clone)]
pub struct BinaryNode {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
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

impl NodeInfo<BinaryNode> for BinaryTree {
    type Key = ();

    fn key(&self, _: &BinaryNode) -> Self::Key {
        ()
    }

    fn children(&self, node: &BinaryNode) -> impl Iterator<Item = BinaryNode> {
        node.left.iter().chain(node.right.iter()).map(|x| (**x).clone())
    }
}
