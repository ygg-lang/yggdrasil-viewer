use std::slice::Iter;
use shape_core::Rectangle;
use tree_layout::layout_position;
use tree_view::{layout, NodeInfo};

#[derive(Debug, Clone)]
pub struct Tree;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub children: Vec<Node>,
}

impl<'n> NodeInfo<&'n Node> for Tree {
    type Index = usize;
    type Children = Iter<'n, Node>;
    fn query(&self, node: &'n Node) -> Self::Index {
        node.id
    }
    fn children(&self, node: &'n Node) -> Self::Children {
         node.children.iter()
    }

    fn dimensions(&self, _: &'n Node) -> Rectangle<f64> {
        Rectangle::from_origin(0.5, 0.5)
    }

    fn border(&self, _: &'n Node) -> Rectangle<f64> {
        Rectangle::from_origin(7, 3)

    }
}

pub fn labeller(_: &Tree, node: &Node) -> String {
    format!("{}", node.id)
}

fn tree() -> Node {
    Node {
        id: 0,
        children: vec![
            Node {
                id: 1,
                children: vec![
                    Node { id: 6, children: vec![] },
                    Node {
                        id: 7,
                        children: vec![Node {
                            id: 12,
                            children: vec![
                                Node { id: 18, children: vec![] },
                                Node { id: 19, children: vec![] },
                                Node { id: 20, children: vec![] },
                                Node { id: 21, children: vec![] },
                                Node { id: 22, children: vec![] },
                            ],
                        }],
                    },
                ],
            },
            Node { id: 2, children: vec![] },
            Node {
                id: 3,
                children: vec![
                    Node { id: 8, children: vec![Node { id: 13, children: vec![] }] },
                    Node {
                        id: 9,
                        children: vec![
                            Node { id: 14, children: vec![Node { id: 23, children: vec![] }] },
                            Node {
                                id: 15,
                                children: vec![
                                    Node { id: 24, children: vec![] },
                                    Node { id: 25, children: vec![] },
                                    Node {
                                        id: 26,
                                        children: vec![
                                            Node { id: 30, children: vec![] },
                                            Node {
                                                id: 31,
                                                children: vec![
                                                    Node { id: 33, children: vec![] },
                                                    Node { id: 34, children: vec![] },
                                                    Node { id: 35, children: vec![] },
                                                ],
                                            },
                                        ],
                                    },
                                    Node { id: 27, children: vec![] },
                                ],
                            },
                        ],
                    },
                    Node { id: 10, children: vec![Node { id: 16, children: vec![Node { id: 28, children: vec![] }] }] },
                ],
            },
            Node { id: 4, children: vec![] },
            Node {
                id: 5,
                children: vec![Node {
                    id: 11,
                    children: vec![Node {
                        id: 17,
                        children: vec![Node {
                            id: 29,
                            children: vec![Node {
                                id: 32,
                                children: vec![
                                    Node { id: 36, children: vec![] },
                                    Node { id: 37, children: vec![] },
                                    Node { id: 38, children: vec![] },
                                ],
                            }],
                        }],
                    }],
                }],
            },
        ],
    }
}

fn main() {
    let root = tree();
    let layout = layout_position(&Tree, &root);
    println!("{:?}", layout)
}
