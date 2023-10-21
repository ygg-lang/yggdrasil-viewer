pub struct Tree {
    w: f64,
    h: f64, // ^{\normalfont 宽度和高度}^
    x: f64,
    y: f64,
    prelim: f64,
    mod_: f64,
    shift: f64,
    change: f64,
    tl: Option<Box<Tree>>, // ^{\normalfont 左线程}^
    tr: Option<Box<Tree>>, // ^{\normalfont 右线程}^
    el: Option<Box<Tree>>, // ^{\normalfont 最左边和最右边的节点}^
    er: Option<Box<Tree>>,
    msel: f64, // ^{\normalfont 最左边和最右边节点的修正总和}^
    mser: f64,
    c: Vec<Tree>,
}

impl Tree {
    pub fn new(w: f64, h: f64, y: f64, c: Vec<Tree>) -> Tree {
        Tree {
            w,
            h,
            x: 0.0,
            y,
            prelim: 0.0,
            mod_: 0.0,
            shift: 0.0,
            change: 0.0,
            tl: None,
            tr: None,
            el: None,
            er: None,
            msel: 0.0,
            mser: 0.0,
            c,
        }
    }
}

pub fn layout(t: &mut Tree) {
    first_walk(t);
    second_walk(t, 0.0);
}

fn first_walk(t: &mut Tree) {
    if t.c.is_empty() {
        set_extremes(t);
        return;
    }
    first_walk(&mut t.c[0]);
    let ih = update_iyl(bottom(&t.c[0].el), 0, None);
    for i in 1..t.c.len() {
        first_walk(&mut t.c[i]);
        let min_y = bottom(&t.c[i].er);
        seperate(t, i, ih);
        ih = update_iyl(min_y, i, ih);
    }
    position_root(t);
    set_extremes(t);
}

fn set_extremes(t: &mut Tree) {
    if t.c.is_empty() {
        t.el = Some(Box::new(t.clone()));
        t.er = Some(Box::new(t.clone()));
        t.msel = 0.0;
        t.mser = 0.0;
    }
    else {
        t.el = Some(Box::new(t.c[0].el.as_ref().unwrap().clone()));
        t.msel = t.c[0].msel;
        t.er = Some(Box::new(t.c[t.c.len() - 1].er.as_ref().unwrap().clone()));
        t.mser = t.c[t.c.len() - 1].mser;
    }
}

fn seperate(t: &mut Tree, i: usize, ih: Option<IYL>) {
    let sr = &t.c[i - 1];
    let mssr = sr.mod_;
    let cl = &t.c[i];
    let mscl = cl.mod_;
    let mut sr = sr.tl.as_ref();
    let mut cl = cl.tl.as_ref();
    while sr.is_some() && cl.is_some() {
        if bottom(sr.unwrap()) > ih.as_ref().unwrap().low_y {
            // ^{\normalfont 右侧节点的左侧到左侧节点的左侧有多远？}^
            let dist = (mssr + sr.unwrap().prelim + sr.unwrap().w) - (mscl + cl.unwrap().prelim);
            if dist > 0.0 {
                cl.unwrap().mod_ += dist;
                move_subtree(t, i, ih.as_ref().unwrap().index, dist);
            }
            let sy = bottom(sr.unwrap());
            let cy = bottom(cl.unwrap());
            if sy <= cy {
                sr = next_right_contour(sr.unwrap());
                if let Some(sr) = sr {
                    mssr += sr.mod_;
                }
            }
            if sy >= cy {
                cl = next_left_contour(cl.unwrap());
                if let Some(cl) = cl {
                    mscl += cl.mod_;
                }
            }
        }
        else {
            break;
        }
    }
    ih.as_ref().unwrap().index = i;
}

fn move_subtree(t: &mut Tree, i: usize, si: usize, dist: f64) {
    t.c[i].mod_ += dist;
    t.c[i].prelim += dist;
    t.c[i].shift += dist;
    t.c[si].change += dist / (t.c[si].w - t.c[i].w);
    t.c[si].shift += dist;
    t.c[si].mod_ += dist;
}

fn next_left_contour(mut t: &mut Tree) -> Option<&mut Tree> {
    if !t.c.is_empty() {
        return Some(&mut t.c[0]);
    }
    if let Some(tr) = &mut t.tr {
        return Some(tr);
    }
    None
}

fn next_right_contour(mut t: &mut Tree) -> Option<&mut Tree> {
    if !t.c.is_empty() {
        return Some(&mut t.c[t.c.len() - 1]);
    }
    if let Some(tl) = &mut t.tl {
        return Some(tl);
    }
    None
}

fn update_iyl(min_y: f64, i: usize, mut ih: Option<IYL>) -> Option<IYL> {
    if ih.is_none() || min_y < ih.as_ref().unwrap().low_y {
        return Some(IYL { low_y: min_y, index: i });
    }
    ih.unwrap().index = i;
    ih
}

fn bottom(t: &Tree) -> f64 {
    t.y + t.h
}

fn position_root(t: &mut Tree) {
    let mut x = 0.0;
    for i in 0..t.c.len() {
        let c = &mut t.c[i];
        c.prelim += x;
        x += c.mod_ + c.shift;
    }
}

fn second_walk(t: &mut Tree, mod_sum: f64) {
    t.x += mod_sum;
    mod_sum += t.mod_;
    for i in 0..t.c.len() {
        second_walk(&mut t.c[i], mod_sum);
    }
}

struct IYL {
    low_y: f64,
    index: usize,
}
