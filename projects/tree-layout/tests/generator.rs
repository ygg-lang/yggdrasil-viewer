use super::*;

pub fn gen_node(rng: &mut StdRng) -> LayoutNode {
    LayoutNode {
        id: rng.gen(),
        width: rng.gen_range(5..50) as Coordinate,
        height: rng.gen_range(5..50) as Coordinate,
        ..Default::default()
    }
}

pub fn gen_tree(rng: &mut StdRng, num: usize) -> Box<LayoutNode> {
    let mut root = Box::new(gen_node(rng));
    let mut nodes: Vec<NonNull<LayoutNode>> = vec![(&mut *root).into()];
    for _ in 0..num {
        let parent_index = rng.gen_range(0..nodes.len());
        let parent = unsafe { nodes[parent_index].as_mut() };
        let node = gen_node(rng);
        parent.append_child(node);
        nodes.push(parent.children.last_mut().unwrap().as_mut().into());
    }

    root
}

pub fn prepare_tree(rng: &mut StdRng) -> (Box<LayoutNode>, Vec<NonNull<LayoutNode>>) {
    let mut root = Box::new(gen_node(rng));
    let nodes: Vec<NonNull<LayoutNode>> = vec![(&mut *root).into()];
    (root, nodes)
}

pub fn insert_new_to_tree(rng: &mut StdRng, num: usize, nodes: &mut Vec<NonNull<LayoutNode>>) {
    for _ in 0..num {
        let parent_index = rng.gen_range(0..nodes.len());
        let parent = unsafe { nodes[parent_index].as_mut() };
        let node = gen_node(rng);
        parent.append_child(node);
        nodes.push(parent.children.last().unwrap().as_ref().into());
    }
}
