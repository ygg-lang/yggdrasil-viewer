extern crate reingold_tilford;

mod utils;

struct Tree;

struct Node {
    id: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl<'n> reingold_tilford::NodeInfo<&'n Node> for Tree {
    type Key = usize;

    fn key(&self, node: &'n Node) -> Self::Key {
        node.id
    }

    fn children(&self, node: &'n Node) -> reingold_tilford::SmallVec<&'n Node> {
        let mut vec = reingold_tilford::SmallVec::new();

        if let Some(ref left) = node.left {
            vec.push(left.as_ref());
        }

        if let Some(ref right) = node.right {
            vec.push(right.as_ref());
        }

        vec
    }

    fn dimensions(&self, _: &'n Node) -> reingold_tilford::Dimensions {
        reingold_tilford::Dimensions::all(0.5)
    }

    fn border(&self, _: &'n Node) -> reingold_tilford::Dimensions {
        reingold_tilford::Dimensions { top: 1.5, right: 3.5, bottom: 1.5, left: 3.5 }
    }
}

fn tree() -> Node {
    Node {
        id: 0,
        left: Some(Box::new(Node {
            id: 1,
            left: Some(Box::new(Node { id: 3, left: None, right: Some(Box::new(Node { id: 6, left: None, right: None })) })),
            right: None,
        })),
        right: Some(Box::new(Node {
            id: 2,
            left: Some(Box::new(Node { id: 4, left: Some(Box::new(Node { id: 7, left: None, right: None })), right: None })),
            right: Some(Box::new(Node { id: 5, left: None, right: None })),
        })),
    }
}

fn main() {
    let node = tree();
    let layout = reingold_tilford::layout(&Tree, &node);
    utils::display(&Tree, &node, &layout, |_, n| format!("{}", n.id));
}
