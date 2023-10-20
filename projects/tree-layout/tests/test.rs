extern crate petgraph;
extern crate reingold_tilford;

use petgraph::graph;

struct Graph(graph::Graph<usize, ()>);

impl reingold_tilford::NodeInfo<graph::NodeIndex> for Graph {
    type Key = graph::NodeIndex;

    fn key(&self, node: graph::NodeIndex) -> Self::Key {
        node
    }

    fn children(&self, node: graph::NodeIndex) -> reingold_tilford::SmallVec<graph::NodeIndex> {
        self.0.neighbors(node).collect()
    }

    fn dimensions(&self, _: graph::NodeIndex) -> reingold_tilford::Dimensions {
        reingold_tilford::Dimensions::all(0.5)
    }

    fn border(&self, _: graph::NodeIndex) -> reingold_tilford::Dimensions {
        reingold_tilford::Dimensions::all(1.5)
    }
}

impl Graph {
    fn new() -> (Graph, graph::NodeIndex) {
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
    let layout = reingold_tilford::layout(&graph, root);

    let expected = [
        (graph::NodeIndex::new(0), reingold_tilford::Coordinate { x: 10.0, y: 2.0 }),
        (graph::NodeIndex::new(1), reingold_tilford::Coordinate { x: 14.0, y: 6.0 }),
        (graph::NodeIndex::new(2), reingold_tilford::Coordinate { x: 6.0, y: 6.0 }),
        (graph::NodeIndex::new(3), reingold_tilford::Coordinate { x: 14.0, y: 10.0 }),
        (graph::NodeIndex::new(4), reingold_tilford::Coordinate { x: 10.0, y: 10.0 }),
        (graph::NodeIndex::new(5), reingold_tilford::Coordinate { x: 6.0, y: 10.0 }),
        (graph::NodeIndex::new(6), reingold_tilford::Coordinate { x: 2.0, y: 10.0 }),
    ]
    .iter()
    .cloned()
    .collect::<std::collections::HashMap<graph::NodeIndex<u32>, _>>();

    assert_eq!(layout, expected);
}

#[derive(Debug, Clone)]
pub struct Tree;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub children: Vec<Node>,
}

impl<'n> reingold_tilford::NodeInfo<&'n Node> for Tree {
    type Key = usize;

    fn key(&self, node: &'n Node) -> Self::Key {
        node.id
    }

    fn children(&self, node: &'n Node) -> reingold_tilford::SmallVec<&'n Node> {
        node.children.iter().collect()
    }

    fn dimensions(&self, _: &'n Node) -> reingold_tilford::Dimensions {
        reingold_tilford::Dimensions::all(0.5)
    }

    fn border(&self, _: &'n Node) -> reingold_tilford::Dimensions {
        reingold_tilford::Dimensions { top: 1.5, right: 3.5, bottom: 1.5, left: 3.5 }
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
    let layout = reingold_tilford::layout(&Tree, &root);

    let expected = [
        (0, reingold_tilford::Coordinate { x: 18.0, y: 2.0 }),
        (1, reingold_tilford::Coordinate { x: 18.0, y: 6.0 }),
        (2, reingold_tilford::Coordinate { x: 8.0, y: 10.0 }),
        (3, reingold_tilford::Coordinate { x: 28.0, y: 10.0 }),
        (4, reingold_tilford::Coordinate { x: 4.0, y: 14.0 }),
        (5, reingold_tilford::Coordinate { x: 12.0, y: 14.0 }),
        (6, reingold_tilford::Coordinate { x: 20.0, y: 14.0 }),
        (7, reingold_tilford::Coordinate { x: 28.0, y: 14.0 }),
        (8, reingold_tilford::Coordinate { x: 36.0, y: 14.0 }),
        (9, reingold_tilford::Coordinate { x: 4.0, y: 18.0 }),
        (10, reingold_tilford::Coordinate { x: 16.0, y: 18.0 }),
        (11, reingold_tilford::Coordinate { x: 24.0, y: 18.0 }),
        (12, reingold_tilford::Coordinate { x: 16.0, y: 22.0 }),
        (13, reingold_tilford::Coordinate { x: 24.0, y: 22.0 }),
        (14, reingold_tilford::Coordinate { x: 32.0, y: 22.0 }),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(layout, expected);
}
