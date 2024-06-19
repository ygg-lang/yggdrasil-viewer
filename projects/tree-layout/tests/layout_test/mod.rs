use super::*;

pub fn test_layout(layout: &mut LayoutConfig) {
    let mut rng = StdRng::seed_from_u64(1001);
    for _ in 0..100 {
        let mut tree = gen_tree(&mut rng, 100);
        layout.layout(&mut tree);
        // let first: Vec<Coord> = tree.iter().map(|node| node.x).collect();
        // layout.layout(&mut tree);
        // let second: Vec<Coord> = tree.iter().map(|node| node.x).collect();
        // assert_eq!(first, second);
        aesthetic_rules::assert_no_overlap_nodes(&tree);
        aesthetic_rules::check_nodes_order(&tree);
        aesthetic_rules::check_y_position_in_same_level(&tree);
        aesthetic_rules::assert_symmetric(&tree, layout);
    }
}

pub fn test_partial_layout(layout: &mut LayoutConfig) {
    let mut rng = StdRng::seed_from_u64(2001);
    for _ in 0..10 {
        let mut tree = gen_tree(&mut rng, 10);
        layout.layout(&mut tree);
        let mut nodes: Vec<NonNull<LayoutNode>> = vec![];
        tree.pre_order_traversal(|node| nodes.push(node.into()));
        for _ in 0..100 {
            let new_node = insert_random_node(&mut rng, &nodes);
            let changed_node = change_random_node(&mut rng, &nodes);
            // let pre = tree.str();
            layout.partial_layout(&mut tree, &[new_node, changed_node]);
            let result = catch_unwind(|| {
                aesthetic_rules::check_nodes_order(&tree);
                aesthetic_rules::check_y_position_in_same_level(&tree);
                aesthetic_rules::assert_no_overlap_nodes(&tree);
            });
            if result.is_err() {
                println!("\n\nTREE:\n{}", tree.str());
                println!("NEW NODE:\n{}", unsafe { new_node.as_ref() }.str());
                // println!("CHANGED NODE:\n{}", unsafe { changed_node.as_ref() }.str());
                // println!("\n\nPRE:\n{}", pre);
                panic!();
            }
        }
    }
}

pub fn align_partial_layout_with_full_layout(layout: &mut LayoutConfig) {
    let mut rng = StdRng::seed_from_u64(1001);
    for _i in 0..10 {
        let mut tree = gen_tree(&mut rng, 100);
        layout.layout(&mut tree);
        let mut nodes: Vec<NonNull<LayoutNode>> = vec![];
        tree.pre_order_traversal(|node| nodes.push(node.into()));
        for times in 0..100 {
            let new_node = insert_random_node(&mut rng, &nodes);
            let changed_node = change_random_node(&mut rng, &nodes);
            layout.partial_layout(&mut tree, &[new_node, changed_node]);
            // let partial_str = tree.str();
            let partial_x: Vec<Coordinate> = tree.iter().map(|node| node.center.x).collect();
            layout.layout(&mut tree);
            let full_x: Vec<Coordinate> = tree.iter().map(|node| node.center.x).collect();
            for i in 0..partial_x.len() {
                if (full_x[i] - partial_x[i]).abs() > 1e-6 {
                    println!("NEW_NODE: {}", unsafe { new_node.as_ref().str() });
                    println!("{} != {}", full_x[i], partial_x[i]);
                    panic!(
                        "partial layout result does not equal full layout result. Times: {}.\nfull: {:?}\npartial: {:?}\n\nFULL\n{}\n\nPARTIAL\n{}",
                        times,
                        &full_x,
                        &partial_x,
                        tree.str(),
                        "" // partial_str
                    );
                }
            }
        }
    }
}

fn change_random_node(rng: &mut StdRng, nodes: &[NonNull<LayoutNode>]) -> NonNull<LayoutNode> {
    let node_index = rng.gen_range(0..nodes.len());
    let node = unsafe { &mut *nodes[node_index].as_ptr() };
    node.width = rng.gen_range(1. ..100.);
    node.height = rng.gen_range(1. ..100.);
    nodes[node_index]
}

fn insert_random_node(rng: &mut StdRng, nodes: &[NonNull<LayoutNode>]) -> NonNull<LayoutNode> {
    let node_index = rng.gen_range(0..nodes.len());
    let node = unsafe { &mut *nodes[node_index].as_ptr() };
    let new_node = gen_node(rng);
    node.append_child(new_node)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tidy_layout() {
        let mut layout = LayoutConfig::new(10., 10.);
        test_layout(&mut layout);
    }

    #[test]
    fn test_tidy_layout2() {
        let mut tidy = LayoutConfig::new(1., 1.);
        let mut root = LayoutNode::new(0, 1., 1.);
        let first_child =
            LayoutNode::new_with_child(1, 1., 1., LayoutNode::new_with_child(10, 2., 1., LayoutNode::new(100, 1., 1.)));
        root.append_child(first_child);

        let second =
            LayoutNode::new_with_child(2, 1., 1., LayoutNode::new_with_child(11, 1., 1., LayoutNode::new(101, 1., 1.)));
        root.append_child(second);

        root.append_child(LayoutNode::new(3, 1., 2.));
        tidy.layout(&mut root);
        // println!("{}", root.str());
        aesthetic_rules::assert_symmetric(&root, &mut tidy);
    }

    #[test]
    fn test_tidy_layout3() {
        let mut tidy = LayoutConfig::new(1.0, 1.);
        let mut root = LayoutNode::new(0, 8., 7.);
        root.append_child(LayoutNode::new_with_children(
            1,
            3.,
            9.,
            vec![LayoutNode::new(10, 3., 8.), LayoutNode::new(10, 5., 5.), LayoutNode::new(10, 6., 8.)],
        ));
        root.append_child(LayoutNode::new(3, 1., 1.));

        tidy.layout(&mut root);
        // println!("{}", root.str());
        aesthetic_rules::assert_no_overlap_nodes(&root);
        aesthetic_rules::assert_symmetric(&root, &mut tidy);
    }

    #[test]
    fn test_tidy_partial_layout() {
        let mut layout = LayoutConfig::new(10.0, 10.0);
        test_partial_layout(&mut layout);
        align_partial_layout_with_full_layout(&mut layout);
    }

    #[test]
    fn test_layered_tidy_layout() {
        let mut layout = LayoutConfig::new(10.0, 10.0).with_layered(true);
        test_layout(&mut layout);
        test_partial_layout(&mut layout);
    }
}
