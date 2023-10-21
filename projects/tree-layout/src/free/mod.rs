#[derive(Clone)]
struct Tree {
    w: i32,
    h: i32,
    y: i32,
    c: Vec<Tree>,
    cs: usize,
    x: i32,
    prelim: i32,
    mod_: i32,
    shift: i32,
    change: i32,
    tl: Option<Box<Tree>>, // Left thread
    tr: Option<Box<Tree>>, // Right thread
    el: Option<Box<Tree>>, // extreme left nodes
    er: Option<Box<Tree>>, // extreme right nodes
    msel: i32,             // sum of modifiers at the extreme nodes
    mser: i32,
}

fn set_extremes(tree: &mut Tree) {
    if tree.cs == 0 {
        tree.el = Some(Box::new(tree.clone()));
        tree.er = Some(Box::new(tree.clone()));
        tree.msel = 0;
        tree.mser = 0;
    }
    else {
        tree.el = tree.c[0].el.clone();
        tree.msel = tree.c[0].msel;
        tree.er = tree.c[tree.cs - 1].er.clone();
        tree.mser = tree.c[tree.cs - 1].mser;
    }
}

fn bottom(tree: &Tree) -> i32 {
    tree.y + tree.h
}

/// A linked list of the indexes of left siblings and their lowest vertical coordinate.
struct IYL {
    low_y: i32,
    index: usize,
    next: Option<Box<IYL>>,
}

impl IYL {
    fn new(low_y: i32, index: usize, next: Option<Box<IYL>>) -> Self {
        Self { low_y, index, next }
    }
}

fn update_iyl(min_y: i32, i: usize, ih: Option<Box<IYL>>) -> Box<IYL> {
    // Remove siblings that are hidden by the new subtree.
    let mut ih = ih;
    while let Some(ref mut node) = ih {
        if min_y >= node.low_y {
            ih = node.next.take();
        }
        else {
            break;
        }
    }
    Box::new(IYL::new(min_y, i, ih))
}

fn distribute_extra(tree: &mut Tree, i: usize, si: usize, distance: i32) {
    // Are there intermediate children?
    if si != i - 1 {
        let nr = (i - si) as i32;
        tree.c[si + 1].shift += distance / nr;
        tree.c[i].shift -= distance / nr;
        tree.c[i].change -= distance - distance / nr;
    }
}

fn move_subtree(tree: &mut Tree, i: usize, si: usize, distance: i32) {
    // Move subtree by changing mod_.
    tree.c[i].mod_ += distance;
    tree.c[i].msel += distance;
    tree.c[i].mser += distance;
    distribute_extra(tree, i, si, distance);
}

fn next_left_contour(tree: &Tree) -> Option<&Tree> {
    if tree.cs == 0 { tree.tl.as_deref() } else { tree.c.get(0) }
}

fn next_right_contour(tree: &Tree) -> Option<&Tree> {
    if tree.cs == 0 { tree.tr.as_deref() } else { tree.c.get(tree.cs - 1) }
}

fn set_left_thread(tree: &mut Tree, i: usize, cl: Option<Box<Tree>>, modsumcl: i32) {
    let mut li = tree.c[0].el.as_mut().unwrap();
    li.tl = cl;
    // Change mod_ so that the sum of modifier after following thread is correct.
    let diff = (modsumcl - cl.as_ref().unwrap().mod_) - tree.c[0].msel;
    li.mod_ += diff;
    // Change preliminary x coordinate so that the node does not move.
    li.prelim -= diff;
    // Update extreme node and its sum of modifiers.
    tree.c[0].el = tree.c[i].el.clone();
    tree.c[0].msel = tree.c[i].msel;
}

// Symmetrical to set_left_thread
fn set_right_thread(tree: &mut Tree, i: usize, sr: Option<Box<Tree>>, modsumsr: i32) {
    let mut ri = tree.c[i].er.as_mut().unwrap();
    ri.tr = sr;
    let diff = (modsumsr - sr.as_ref().unwrap().mod_) - tree.c[i].mser;
    ri.mod_ += diff;
    ri.prelim -= diff;
    tree.c[i].er = tree.c[i - 1].er.clone();
    tree.c[i].mser = tree.c[i - 1].mser;
}

fn separate(tree: &mut Tree, i: usize, mut ih: Option<Box<IYL>>) {
    // Right contour node of left siblings and its sum of modifiers.
    let sr = &tree.c[i - 1];
    let mssr = sr.mod_;
    // Left contour node of right siblings and its sum of modifiers.
    let cl = &tree.c[i];
    let mut mscl = cl.mod_;

    while let (Some(sr_node), Some(cl_node)) = (sr, cl) {
        if bottom(sr_node) > ih.as_ref().unwrap().low_y {
            ih = ih.unwrap().next;
        }

        // How far to the left of the right side of sr is the left side of cl.
        let distance = mssr + sr_node.prelim + sr_node.w - (mscl + cl_node.prelim);

        if distance > 0 {
            mscl += distance;
            move_subtree(tree, i, ih.as_ref().unwrap().index, distance);
        }

        let sr = next_right_contour(sr);
        if let Some(right) = sr {
            mssr += right.mod_;
        }

        let cl = next_left_contour(cl);
        if let Some(left) = cl {
            mscl += left.mod_;
        }
    }

    // Set threads and update extreme nodes.
    // In the first case, the current subtree must be taller than the left siblings.
    if sr.is_none() && cl.is_some() {
        set_left_thread(tree, i, cl.clone().unwrap(), mscl);
    }
    else if sr.is_some() && cl.is_none() {
        set_right_thread(tree, i, sr.clone().unwrap(), mssr);
    }
}

fn position_root(tree: &mut Tree) {
    // Position root between children, taking into account their mod.
    tree.prelim =
        (tree.c[0].prelim + tree.c[0].mod_ + tree.c[tree.cs - 1].mod_ + tree.c[tree.cs - 1].prelim + tree.c[tree.cs - 1].w) / 2
            - tree.w / 2;
}

fn first_walk(tree: &mut Tree) {
    if tree.cs == 0 {
        set_extremes(tree);
        return;
    }

    first_walk(&mut tree.c[0]);
    let mut ih = update_iyl(bottom(&tree.c[0].el.as_ref().unwrap()), 0, None);

    for i in 1..tree.cs {
        first_walk(&mut tree.c[i]);
        let minY = bottom(tree.c[i].er.as_ref().unwrap());
        separate(tree, i, ih);
        ih = update_iyl(minY, i, ih);
    }

    position_root(tree);
    set_extremes(tree);
}

fn add_child_spacing(tree: &mut Tree) {
    let mut d = 0;
    let mut modsumdelta = 0;

    for i in 0..tree.cs {
        d += tree.c[i].shift;
        modsumdelta += d + tree.c[i].change;
        tree.c[i].mod_ += modsumdelta;
    }
}

fn second_walk(tree: &mut Tree, mut modsum: i32) {
    modsum += tree.mod_;
    // Set absolute (no-relative) horizontal coordinates.
    tree.x = tree.prelim + modsum;
    add_child_spacing(tree);

    for i in 0..tree.cs {
        second_walk(&mut tree.c[i], modsum);
    }
}

fn layout(tree: &mut Tree) {
    first_walk(tree);
    second_walk(tree, 0);
}
