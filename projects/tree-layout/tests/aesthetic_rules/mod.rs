use super::*;

pub fn assert_no_overlap_nodes(root: &LayoutNode) {
    let mut nodes: Vec<NonNull<LayoutNode>> = vec![];
    root.post_order_traversal(|node| {
        for other in nodes.iter() {
            let other = unsafe { other.as_ref() };
            if node.intersects(other) {
                let msg = format!("{} and {} overlap", node.str(), other.str());
                panic!("{}\n\n{}", msg, root.str());
            }
        }

        nodes.push(node.into());
    });
}

pub fn check_nodes_order(root: &LayoutNode) {
    root.pre_order_traversal(|node| {
        let mut prev = None;
        for child in node.children.iter() {
            if let Some(prev) = prev {
                assert!(prev < child.point.x);
            }

            prev = Some(child.point.x);
        }
    })
}

pub fn check_y_position_in_same_level(root: &LayoutNode) {
    root.pre_order_traversal(|node| {
        let mut prev = None;
        for child in node.children.iter() {
            if let Some(prev) = prev {
                assert_eq!(prev, child.point.y);
            }

            prev = Some(child.point.y);
        }
    })
}

pub fn assert_symmetric(root: &LayoutNode, layout: &mut LayoutConfig) {
    let mut mirrored = mirror(root);
    layout.layout(&mut mirrored);
    let mut point_origin: Vec<Coordinate> = vec![];
    let mut point_mirrored: Vec<Coordinate> = vec![];
    root.pre_order_traversal(|node| {
        point_origin.push(node.point.x);
    });
    pre_order_traversal_rev(&mirrored, |node| {
        point_mirrored.push(node.point.x);
    });

    assert_eq!(point_origin.len(), point_mirrored.len());
    for i in 0..point_origin.len() {
        if (point_origin[i] + point_mirrored[i]).abs() > 1e-6 {
            println!("{}", root.str());
            println!("{}", mirrored.str());
            panic!("{} != {}", point_origin[i], point_mirrored[i]);
        }
    }

    fn pre_order_traversal_rev<F>(node: &LayoutNode, mut f: F)
    where
        F: FnMut(&LayoutNode),
    {
        let mut stack: Vec<NonNull<LayoutNode>> = vec![node.into()];
        while let Some(mut node) = stack.pop() {
            let node = unsafe { node.as_mut() };
            f(node);
            for child in node.children.iter().rev() {
                stack.push(child.as_ref().into());
            }
        }
    }
}

fn mirror(root: &LayoutNode) -> LayoutNode {
    let mut root = root.clone();
    root.post_order_traversal_mut(|node| {
        node.point.x = 0.0;
        node.point.y = 0.0;
        node.relative_x = 0.0;
        node.relative_y = 0.0;
        let n = node.children.len();
        for i in 0..n / 2 {
            node.children.swap(i, n - i - 1);
        }
    });
    root
}
