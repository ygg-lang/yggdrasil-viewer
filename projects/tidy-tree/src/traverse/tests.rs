use super::*;

#[test]
fn test_node_iter() {
    let mut root = TidyNode::new_with_child(0, 1., 1., TidyNode::new(1, 2., 2.));
    assert_eq!(root.iter().count(), 2);
    root.append_child(TidyNode::new(2, 3., 3.));
    assert_eq!(root.iter().count(), 3);
    root.append_child(TidyNode::new(3, 3., 3.));
    assert_eq!(root.iter().count(), 4);
    root.children[2].append_child(TidyNode::new(4, 3., 3.));
    assert_eq!(root.iter().count(), 5);

    for (i, node) in root.iter().enumerate() {
        assert_eq!(i, node.id);
    }
}
