use std::collections::BTreeMap;

use petgraph::{graph, graph::NodeIndex};
use shape_core::Point;

use tree_layout::{layout, NodeInfo, TreeBox};

struct Graph(graph::Graph<usize, ()>);

impl NodeInfo<NodeIndex> for Graph {
    type Key = NodeIndex;

    fn key(&self, node: NodeIndex) -> Self::Key {
        node
    }

    fn children(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        self.0.neighbors(node)
    }

    fn dimensions(&self, _: NodeIndex) -> TreeBox {
        TreeBox::square(0.5)
    }

    fn border(&self, _: NodeIndex) -> TreeBox {
        TreeBox::square(1.5)
    }
}

impl Graph {
    fn new() -> (Graph, NodeIndex) {
        let mut graph = graph::Graph::new();
        let root = graph.add_node(0);

        let c1 = graph.add_node(1);
        let c2 = graph.add_node(2);
        let c1c1 = graph.add_node(3);
        let c2c1 = graph.add_node(4);
        let c2c2 = graph.add_node(5);
        let c2c3 = graph.add_node(6);

        graph.extend_with_edges(&[(root, c1), (root, c2), (c1, c1c1), (c2, c2c1), (c2, c2c2), (c2, c2c3)]);

        (Graph(graph), root)
    }
}

#[test]
fn petgraph() {
    let (graph, root) = Graph::new();
    let layout = layout(&graph, root).into_iter().map(|a| (a.data.key, a.data.center())).collect::<BTreeMap<_, _>>();

    let expected = [
        (NodeIndex::new(0), Point { x: 10.0, y: 2.0 }),
        (NodeIndex::new(1), Point { x: 14.0, y: 6.0 }),
        (NodeIndex::new(2), Point { x: 6.0, y: 6.0 }),
        (NodeIndex::new(3), Point { x: 14.0, y: 10.0 }),
        (NodeIndex::new(4), Point { x: 10.0, y: 10.0 }),
        (NodeIndex::new(5), Point { x: 6.0, y: 10.0 }),
        (NodeIndex::new(6), Point { x: 2.0, y: 10.0 }),
    ]
    .iter()
    .cloned()
    .collect::<BTreeMap<NodeIndex<u32>, _>>();

    assert_eq!(layout, expected);
}

#[derive(Debug, Clone)]
pub struct Tree;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub children: Vec<Node>,
}

impl<'n> NodeInfo<&'n Node> for Tree {
    type Key = usize;

    fn key(&self, node: &'n Node) -> Self::Key {
        node.id
    }

    fn children(&self, node: &'n Node) -> impl Iterator<Item = &'n Node> {
        node.children.iter()
    }

    fn dimensions(&self, _: &'n Node) -> TreeBox {
        TreeBox::square(0.5)
    }

    fn border(&self, _: &'n Node) -> TreeBox {
        TreeBox { top: 1.5, right: 3.5, bottom: 1.5, left: 3.5 }
    }
}

impl Node {
    fn new() -> Node {
        Node {
            id: 0,
            children: vec![Node {
                id: 1,
                children: vec![
                    Node {
                        id: 2,
                        children: vec![
                            Node { id: 4, children: vec![Node { id: 9, children: vec![] }] },
                            Node { id: 5, children: vec![] },
                        ],
                    },
                    Node {
                        id: 3,
                        children: vec![
                            Node {
                                id: 6,
                                children: vec![
                                    Node { id: 10, children: vec![] },
                                    Node {
                                        id: 11,
                                        children: vec![
                                            Node { id: 12, children: vec![] },
                                            Node { id: 13, children: vec![] },
                                            Node { id: 14, children: vec![] },
                                        ],
                                    },
                                ],
                            },
                            Node { id: 7, children: vec![] },
                            Node { id: 8, children: vec![] },
                        ],
                    },
                ],
            }],
        }
    }
}

#[test]
fn ptc() {
    let root = Node::new();
    let layout = layout(&Tree, &root).into_iter().map(|a| (a.data.key, a.data.center())).collect::<BTreeMap<_, _>>();

    let expected = [
        (0, Point { x: 18.0, y: 2.0 }),
        (1, Point { x: 18.0, y: 6.0 }),
        (2, Point { x: 8.0, y: 10.0 }),
        (3, Point { x: 28.0, y: 10.0 }),
        (4, Point { x: 4.0, y: 14.0 }),
        (5, Point { x: 12.0, y: 14.0 }),
        (6, Point { x: 20.0, y: 14.0 }),
        (7, Point { x: 28.0, y: 14.0 }),
        (8, Point { x: 36.0, y: 14.0 }),
        (9, Point { x: 4.0, y: 18.0 }),
        (10, Point { x: 16.0, y: 18.0 }),
        (11, Point { x: 24.0, y: 18.0 }),
        (12, Point { x: 16.0, y: 22.0 }),
        (13, Point { x: 24.0, y: 22.0 }),
        (14, Point { x: 32.0, y: 22.0 }),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(layout, expected);
}
