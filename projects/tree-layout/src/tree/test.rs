use super::*;

struct UserTree;

struct UserNode {
    id: usize,
    children: Vec<UserNode>,
}

impl<'n> NodeInfo<&'n UserNode> for UserTree {
    type Key = usize;

    fn key(&self, node: &'n UserNode) -> Self::Key {
        node.id
    }

    fn children(&self, node: &'n UserNode) -> Vec<&'n UserNode> {
        node.children.iter().collect()
    }
}

fn user_tree() -> UserNode {
    UserNode {
        id: 5,
        children: vec![
            UserNode {
                id: 1,
                children: vec![
                    UserNode { id: 0, children: vec![] },
                    UserNode {
                        id: 3,
                        children: vec![UserNode { id: 2, children: vec![] }, UserNode { id: 4, children: vec![] }],
                    },
                ],
            },
            UserNode { id: 6, children: vec![UserNode { id: 8, children: vec![UserNode { id: 7, children: vec![] }] }] },
        ],
    }
}

fn tree() -> Tree<usize> {
    Tree::new(&UserTree, &user_tree(), |_, n| n.id)
}

#[test]
fn new() {
    let tree = tree();

    assert_eq!(tree.0.len(), 9);
}

#[test]
fn root() {
    let tree = tree();

    assert!(tree.root().is_some());
    assert_eq!(tree[tree.root().unwrap()].data, 5);
}

#[test]
fn breadth_first() {
    let tree = tree();
    let order = tree.breadth_first(tree.root().unwrap()).iter().map(|&n| tree[n].data).collect::<Vec<_>>();

    assert_eq!(order, vec![5, 1, 6, 0, 3, 8, 2, 4, 7]);
}

#[test]
fn post_order() {
    let tree = tree();
    let order = tree.post_order(tree.root().unwrap()).iter().map(|&n| tree[n].data).collect::<Vec<_>>();

    assert_eq!(order, vec![0, 2, 4, 3, 1, 7, 8, 6, 5]);
}

#[test]
fn is_root_true() {
    let tree = tree();

    assert!(tree[tree.root().unwrap()].is_root());
}

#[test]
fn is_root_false() {
    let tree = tree();
    let child = tree[tree.root().unwrap()].children[0];

    assert!(!tree[child].is_root());
}

#[test]
fn is_leaf_true() {
    let tree = tree();
    let leaf = tree[tree[tree[tree.root().unwrap()].children[1]].children[0]].children[0];

    assert!(tree[leaf].is_leaf());
}

#[test]
fn is_leaf_false() {
    let tree = tree();

    assert!(!tree[tree.root().unwrap()].is_leaf());
}
