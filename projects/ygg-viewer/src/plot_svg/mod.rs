use std::cmp::max;

use shape_svg::ToSVG;
use svg::{
    node::element::{Rectangle, Text, SVG},
    Document,
};
use yggdrasil_rt::{TokenPair, TokenTree, YggdrasilRule};

use tree_layout::{layout, NodeInfo, TreeBox, TreeData, TreeNode};

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
    type Key = TokenPair<'i, R>;

    fn key(&self, node: TokenPair<'i, R>) -> Self::Key {
        node
    }

    fn children(&self, node: TokenPair<'i, R>) -> impl Iterator<Item = TokenPair<'i, R>> {
        node.into_inner().filter(|s| !s.get_rule().is_ignore())
    }

    fn dimensions(&self, node: TokenPair<'i, R>) -> TreeBox {
        let chars = width_hint(node);
        TreeBox::rectangle(chars * 16.0, 32.0)
    }
    fn border(&self, _: TokenPair<'i, R>) -> TreeBox {
        TreeBox::square(20.0)
    }
}

fn width_hint<R>(node: TokenPair<R>) -> f64
where
    R: YggdrasilRule,
{
    let text = if node.has_child() { format!("{:?}", node.get_rule()) } else { node.get_string() };
    max(text.len(), 3) as f64
}

impl SvgPlotter {
    pub fn draw<R>(&self, tree: TokenTree<R>) -> SVG
    where
        R: YggdrasilRule,
    {
        SvgTree { cst: tree }.as_svg()
    }
}

impl<'i, R> SvgTree<'i, R>
where
    R: YggdrasilRule,
{
    fn as_svg(&self) -> SVG {
        let mut document = Document::new().set("viewBox", (0, 0, 3000, 3000));
        let root = self.cst.clone().into_iter().next().unwrap();
        let layout = layout(self, root);
        for node in layout {
            let area = node.data.boundary().to_svg().set("fill", "none").set("stroke", "black").set("stroke-width", 0.1);
            document = document.add(area);
            let pair = node.data.key.clone();
            let mut text = Text::new()
                .set("x", node.data.center().x - width_hint(pair.clone()) * 4.0)
                .set("y", node.data.boundary().min.y + 24.0);
            if pair.has_child() {
                text = text.add(svg::node::Text::new(format!("{:?}", pair.get_rule())));
            }
            else {
                text = text.add(svg::node::Text::new(format!("{}", pair.get_string())));
            }
            document = document.add(text);
        }

        document
    }
}
