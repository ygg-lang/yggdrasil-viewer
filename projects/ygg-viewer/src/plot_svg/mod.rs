use std::collections::HashMap;
use yggdrasil_rt::{TokenPair, TokenTree, YggdrasilRule};

use tree_layout::{layout_position, NodeInfo, Point, Rectangle};

#[derive(Debug)]
pub struct SvgTree<'i, R> where R:YggdrasilRule {
    cst: TokenTree<'i, R>,
}

#[derive(Debug, Default)]
pub struct SvgPlotter {
    color: String
}


impl<'i, R> NodeInfo<TokenPair<'i, R>> for SvgTree<'i, R> where R: YggdrasilRule {
    type Index = TokenPair<'i, R>;
    type Children = TokenTree<'i, R>;

    fn query(&self, node: TokenPair<'i, R>) -> Self::Index {
        node
    }

    fn children(&self, node: TokenPair<'i, R>) -> Self::Children {
        node.into_inner()
    }

    fn dimensions(&self, node: TokenPair<'i, R>) -> Rectangle {
        let chars = format!("{:?}", node.get_rule()).len() as f64;
        Rectangle::from_origin(chars, 3.0)
    }
}

impl SvgPlotter {
    pub fn draw<'i, R>(&self, tree: TokenTree<'i, R>) -> SvgTree<'i, R> where R:YggdrasilRule{
        SvgTree {
            cst: tree,
        }
    }
}

impl<'i, R> SvgTree<'i, R> where R: YggdrasilRule {
    pub fn position(&self) -> HashMap<TokenPair<'i, R>, Point> {
        let root = self.cst.clone().into_iter().next().unwrap();
        let layout = layout_position(self, &root);
        layout.into_iter().collect()
    }
}