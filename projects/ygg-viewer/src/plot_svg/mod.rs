use std::collections::HashMap;
use yggdrasil_rt::{TokenPair, TokenTree, YggdrasilRule};

use tree_layout::{layout, layout_position, NodeInfo, Point, Rectangle, TreeData, TreeNode};

#[derive(Debug)]
pub struct SvgTree<'i, R>
where
    R: YggdrasilRule,
{
    cst: TokenTree<'i, R>,
}

#[derive(Debug, Default)]
pub struct SvgPlotter {
    color: String,
}

impl<'i, R> NodeInfo<TokenPair<'i, R>> for SvgTree<'i, R>
where
    R: YggdrasilRule,
{
    type Index = TokenPair<'i, R>;
    type Children = Vec<TokenPair<'i, R>>;

    fn query(&self, node: TokenPair<'i, R>) -> Self::Index {
        node
    }

    fn children(&self, node: TokenPair<'i, R>) -> Self::Children {
        let v: Vec<_> = node.into_inner().filter(|s| !s.get_rule().is_ignore()).collect();
        v
    }

    fn dimensions(&self, node: TokenPair<'i, R>) -> Rectangle {
        let chars = format!("{:?}", node.get_rule()).len() as f64;
        Rectangle::from_origin(chars * 16.0, 40.0)
    }
    fn border(&self, _: TokenPair<'i, R>) -> Rectangle {
        Rectangle::from_origin(10.0, 20.0)
    }
}

impl SvgPlotter {
    pub fn draw<'i, R>(&self, tree: TokenTree<'i, R>) -> SvgTree<'i, R>
    where
        R: YggdrasilRule,
    {
        SvgTree { cst: tree }
    }
}

impl<'i, R> SvgTree<'i, R>
where
    R: YggdrasilRule,
{
    pub fn position(&self) -> Vec<TreeNode<TreeData<TokenPair<'i, R>>>> {
        let root = self.cst.clone().into_iter().next().unwrap();
        let layout = layout(self, root);
        layout.into_iter().collect()
    }
}
