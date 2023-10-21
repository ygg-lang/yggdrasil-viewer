#[derive(Clone)]
struct Tree {
    w: f64,              // 宽度
    h: f64,              // 高度
    y: f64,              // y坐标
    children: Vec<Tree>, // 子节点
    cs: usize,           // 子节点数量

    x: f64,
    prelim: f64,
    modifier: f64,
    shift: f64,
    change: f64,
    tl: Option<Box<Tree>>, // 左线程
    tr: Option<Box<Tree>>, // 右线程
    el: Option<Box<Tree>>, // 极左节点
    er: Option<Box<Tree>>, // 极右节点
    // 极端节点的modifier总和
    msel: f64,
    mser: f64,
}

// 左兄弟节点索引和最低y坐标的链表
struct IYL {
    low_y: f64,
    index: usize,
    next: Option<Box<IYL>>,
}

fn set_extremes(tree: &mut Tree) {
    if tree.cs == 0 {
        tree.el = Some(Box::new(tree.clone()));
        tree.er = Some(Box::new(tree.clone()));
        tree.msel = 0.0;
        tree.mser = 0.0;
    }
    else {
        tree.el = tree.children[0].el.clone();
        tree.msel = tree.children[0].msel;
        tree.er = tree.children[tree.cs - 1].er.clone();
        tree.mser = tree.children[tree.cs - 1].mser;
    }
}

fn bottom(tree: &Tree) -> f64 {
    tree.y + tree.h
}

fn update_iyl(min_y: f64, i: usize, mut ih: Option<Box<IYL>>) -> Option<Box<IYL>> {
    while let Some(ref mut h) = ih {
        if min_y >= h.low_y {
            ih = h.next.take();
        }
        else {
            break;
        }
    }

    Some(Box::new(IYL { low_y: min_y, index: i, next: ih }))
}

fn distribute_extra(tree: &mut Tree, i: usize, si: usize, distance: f64) {
    if si != i - 1 {
        let nr = i - si;
        tree.children[si + 1].shift += distance / nr as f64;
        tree.children[i].shift -= distance / nr as f64;
        tree.children[i].change -= distance - distance / nr as f64;
    }
}

fn move_subtree(tree: &mut Tree, i: usize, si: usize, distance: f64) {
    tree.children[i].modifier += distance;
    tree.children[i].msel += distance;
    tree.children[i].mser += distance;
    distribute_extra(tree, i, si, distance);
}

fn next_left_contour(tree: &mut Tree) -> Option<&mut Tree> {
    if tree.cs == 0 { Some(&mut *tree.tl?) } else { tree.children.get_mut(0) }
}

fn next_right_contour(tree: &mut Tree) -> Option<&mut Tree> {
    if tree.cs == 0 { Some(&mut *tree.tr?) } else { tree.children.get_mut(tree.cs - 1) }
}

fn set_left_thread(tree: &mut Tree, i: usize, cl: &Tree, modsumcl: f64) {
    let li = tree.children[0].el.as_mut().unwrap();
    li.tl = Some(Box::new(cl.clone()));

    let diff = (modsumcl - cl.modifier) - tree.children[0].msel;
    li.modifier += diff;
    li.prelim -= diff;

    tree.children[0].el = tree.children[i].el.clone();
    tree.children[0].msel = tree.children[i].msel;
}

fn set_right_thread(tree: &mut Tree, i: usize, sr: &Tree, modsumsr: f64) {
    let ri = tree.children[i].er.as_mut().unwrap();
    ri.tr = Some(Box::new(sr.clone()));

    let diff = (modsumsr - sr.modifier) - tree.children[i].mser;
    ri.modifier += diff;
    ri.prelim -= diff;

    tree.children[i].er = tree.children[i - 1].er.clone();
    tree.children[i].mser = tree.children[i - 1].mser;
}

fn separate(tree: &mut Tree, i: usize, mut ih: Option<Box<IYL>>) {
    let mut nsr = tree.children.get_mut(i - 1);
    let mut mssr = nsr.map(|v| v.modifier).unwrap_or_default();

    let mut ncl = tree.children.get_mut(i);
    let mut mscl = ncl.map(|v| v.modifier).unwrap_or_default();

    loop {
        match (nsr, ncl) {
            (Some(sr), Some(cl)) => {
                if bottom(sr) > ih.as_ref().unwrap().low_y {
                    ih = ih.unwrap().next.take();
                }

                let distance = mssr + sr.prelim + sr.w - (mscl + cl.prelim);
                if distance > 0.0 {
                    mscl += distance;
                    move_subtree(tree, i, ih.unwrap().index, distance);
                }

                let sy = bottom(sr);
                let cy = bottom(cl);
                if sy <= cy {
                    nsr = next_right_contour(sr);
                    if let Some(s) = sr {
                        mssr += s.modifier;
                    }
                }
                if sy >= cy {
                    ncl = next_left_contour(cl);
                    if let Some(c) = cl {
                        mscl += c.modifier;
                    }
                }
            }
            (None, Some(cl)) => {
                return set_left_thread(tree, i, cl.unwrap(), mscl);
            }
            (Some(sr), None) => {
                return set_right_thread(tree, i, sr.unwrap(), mssr);
            }
            _ => {
                unreachable!()
            }
        }
    }
}

fn position_root(tree: &mut Tree) {
    tree.prelim = (tree.children[0].prelim
        + tree.children[0].modifier
        + tree.children[tree.cs - 1].modifier
        + tree.children[tree.cs - 1].prelim
        + tree.children[tree.cs - 1].w)
        / 2.0
        - tree.w / 2.0;
}

fn first_walk(tree: &mut Tree) {
    if tree.cs == 0 {
        set_extremes(tree);
        return;
    }

    first_walk(&mut tree.children[0]);

    let mut ih = update_iyl(bottom(&tree.children[0].el), 0, None);

    for i in 1..tree.cs {
        first_walk(&mut tree.children[i]);

        let min_y = bottom(&tree.children[i].er);
        separate(tree, i, ih);
        ih = update_iyl(min_y, i, ih);
    }

    position_root(tree);
    set_extremes(tree);
}

fn add_child_spacing(tree: &mut Tree) {
    let mut d = 0.0;
    let mut modsumdelta = 0.0;

    for i in 0..tree.cs {
        d += tree.children[i].shift;
        modsumdelta += d + tree.children[i].change;
        tree.children[i].modifier += modsumdelta;
    }
}

fn second_walk(tree: &mut Tree, mut modsum: f64) {
    modsum += tree.modifier;
    tree.x = tree.prelim + modsum;

    add_child_spacing(tree);

    for i in 0..tree.cs {
        second_walk(&mut tree.children[i], modsum);
    }
}

fn layout(tree: &mut Tree) {
    first_walk(tree);
    second_walk(tree, 0.0);
}
