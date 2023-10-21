给定如下 rust 结构

```rust
#[derive(Clone)]
pub struct TreeArena<T> {
    nodes: Vec<(TreeNode, T)>,
}

#[derive(Clone)]
pub struct TreeNode {
    w: f32,
    // 宽度
    h: f32,
    // 高度
    y: f32,
    // y坐标
    x: f32,
    prelim: f32,
    modifier: f32,
    shift: f32,
    change: f32,
    parent: Option<usize>,
    children: Vec<usize>,
    // 子节点
    cs: usize,
    // 子节点数量
    tl: Option<usize>,
    // 左邻
    tr: Option<usize>,
    // 右邻
    el: Option<usize>,
    // 极左节点
    er: Option<usize>,
    // 极右节点
    msel: f32,
    mser: f32,
}

// 左兄弟节点索引和最低 y 坐标的链表
struct IYL {
    lowest_y: f64,
    index: usize,
    next: Option<usize>,
}
```

将如下 typescript 翻译成 rust, 注释翻译成中文

```typescript
function setExtremes(tree) {
    if (tree.cs === 0) {
        tree.el = tree
        tree.er = tree
        tree.msel = tree.mser = 0
    } else {
        tree.el = tree.c[0].el
        tree.msel = tree.c[0].msel
        tree.er = tree.c[tree.cs - 1].er
        tree.mser = tree.c[tree.cs - 1].mser
    }
}

function bottom(tree) {
    return tree.y + tree.h
}

function updateIYL(minY, i, ih) {
    // Remove siblings that are hidden by the new subtree.
    while (ih !== null && minY >= ih.lowY) {
        // Prepend the new subtree
        ih = ih.next
    }
    return new IYL(minY, i, ih)
}

function distributeExtra(tree, i, si, distance) {
    // Are there intermediate children?
    if (si !== i - 1) {
        const nr = i - si
        tree.c[si + 1].shift += distance / nr
        tree.c[i].shift -= distance / nr
        tree.c[i].change -= distance - distance / nr
    }
}

function moveSubtree(tree, i, si, distance) {
    // Move subtree by changing mod.
    tree.c[i].modifier += distance
    tree.c[i].msel += distance
    tree.c[i].mser += distance
    distributeExtra(tree, i, si, distance)
}

function nextLeftContour(tree) {
    return tree.cs === 0 ? tree.tl : tree.c[0]
}

function nextRightContour(tree) {
    return tree.cs === 0 ? tree.tr : tree.c[tree.cs - 1]
}

function setLeftThread(tree, i, cl, modsumcl) {
    const li = tree.c[0].el
    li.tl = cl
    // Change mod so that the sum of modifier after following thread is correct.
    const diff = (modsumcl - cl.modifier) - tree.c[0].msel
    li.modifier += diff
    // Change preliminary x coordinate so that the node does not move.
    li.prelim -= diff
    // Update extreme node and its sum of modifiers.
    tree.c[0].el = tree.c[i].el
    tree.c[0].msel = tree.c[i].msel
}

// Symmetrical to setLeftThread
function setRightThread(tree, i, sr, modsumsr) {
    const ri = tree.c[i].er
    ri.tr = sr
    const diff = (modsumsr - sr.modifier) - tree.c[i].mser
    ri.modifier += diff
    ri.prelim -= diff
    tree.c[i].er = tree.c[i - 1].er
    tree.c[i].mser = tree.c[i - 1].mser
}
```

注意全部作为 TreeArena 的方法, 修改时需要传入 `&mut self`


```typescript
function seperate(tree, i, ih) {
    // Right contour node of left siblings and its sum of modifiers.
    let sr = tree.c[i - 1]
    let mssr = sr.modifier
    // Left contour node of right siblings and its sum of modifiers.
    let cl = tree.c[i]
    let mscl = cl.modifier
    while (sr !== null && cl !== null) {
        if (bottom(sr) > ih.lowY) {
            ih = ih.next
        }
        // How far to the left of the right side of sr is the left side of cl.
        const distance = mssr + sr.prelim + sr.w - (mscl + cl.prelim)
        if (distance > 0) {
            mscl += distance
            moveSubtree(tree, i, ih.index, distance)
        }

        const sy = bottom(sr)
        const cy = bottom(cl)
        if (sy <= cy) {
            sr = nextRightContour(sr)
            if (sr !== null) {
                mssr += sr.modifier
            }
        }
        if (sy >= cy) {
            cl = nextLeftContour(cl)
            if (cl !== null) {
                mscl += cl.modifier
            }
        }
    }

    // Set threads and update extreme nodes.
    // In the first case, the current subtree must be taller than the left siblings.
    if (sr === null && cl !== null) {
        setLeftThread(tree, i, cl, mscl)
    } else if (sr !== null && cl === null) {
        setRightThread(tree, i, sr, mssr)
    }
}

function positionRoot(tree) {
    // Position root between children, taking into account their mod.
    tree.prelim =
        (tree.c[0].prelim +
            tree.c[0].modifier +
            tree.c[tree.cs - 1].modifier +
            tree.c[tree.cs - 1].prelim +
            tree.c[tree.cs - 1].w) /
        2 -
        tree.w / 2
}

function firstWalk(tree) {
    if (tree.cs === 0) {
        setExtremes(tree)
        return
    }

    firstWalk(tree.c[0])
    let ih = updateIYL(bottom(tree.c[0].el), 0, null)
    for (let i = 1; i < tree.cs; i++) {
        firstWalk(tree.c[i])
        const minY = bottom(tree.c[i].er)
        seperate(tree, i, ih)
        ih = updateIYL(minY, i, ih)
    }
    positionRoot(tree)
    setExtremes(tree)
}

function addChildSpacing(tree) {
    let d = 0
    let modsumdelta = 0
    for (let i = 0; i < tree.cs; i++) {
        d += tree.c[i].shift
        modsumdelta += d + tree.c[i].change
        tree.c[i].modifier += modsumdelta
    }
}

function secondWalk(tree, modsum) {
    modsum += tree.modifier
    // Set absolute (no-relative) horizontal coordinates.
    tree.x = tree.prelim + modsum
    addChildSpacing(tree)
    for (let i = 0; i < tree.cs; i++) {
        secondWalk(tree.c[i], modsum)
    }
}

function layout(tree) {
    firstWalk(tree)
    secondWalk(tree, 0)
}
```