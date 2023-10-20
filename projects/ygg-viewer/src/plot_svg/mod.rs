use yggdrasil_rt::{TokenPair, TokenTree, YggdrasilRule};

use tree_layout::{NodeInfo, Rectangle};

#[derive(Debug)]
pub struct SvgPlotter<'i, R> where R: YggdrasilRule {
    tree: TokenTree<'i, R>,
}


impl<'i, R> NodeInfo<TokenPair<'i, R>> for SvgPlotter<'i, R> where R: YggdrasilRule {
    type Index = TokenPair<'i, R>;
    type Children = TokenTree<'i, R>;

    fn query(&self, node: TokenPair<'i, R>) -> Self::Index {
        node
    }

    fn children(&self, node: TokenPair<'i, R>) -> Self::Children {
        node.into_inner()
    }

    fn dimensions(&self, node: TokenPair<'i, R>) -> Rectangle<f64> {
        let chars = format!("{:?}", node.get_rule()).len() as f64;
        Rectangle::from_origin(chars, 3.0)
    }
}