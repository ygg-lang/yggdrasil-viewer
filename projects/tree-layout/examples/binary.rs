use shape_core::Rectangle;
use tree_layout::{layout_position, NodeInfo};

struct Tree;

struct Node {
    id: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl<'n> NodeInfo<&'n Node> for Tree {
    type Index = usize;
    type Children = Vec<&'n Node>;

    fn query(&self, node: &'n Node) -> Self::Index {
        node.id
    }

    fn children(&self, node: &'n Node) -> Vec<&'n Node> {
        let mut vec = Vec::new();

        if let Some(ref left) = node.left {
            vec.push(left.as_ref());
        }

        if let Some(ref right) = node.right {
            vec.push(right.as_ref());
        }

        vec
    }

    fn dimensions(&self, _: &'n Node) -> Rectangle<f64> {
        Rectangle::from_origin(0.5,0.5)
    }

    fn border(&self, _: &'n Node) -> Rectangle<f64> {
        Rectangle::from_origin(7.0,3.0)
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
    let layout = layout_position(&Tree, &node);
    println!("{:#?}", layout)
}
