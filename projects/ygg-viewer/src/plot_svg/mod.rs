use std::cmp::max;

use shape_svg::ToSVG;
use svg::{
    node::element::{Text, SVG},
    Document,
};
use yggdrasil_rt::{TokenPair, TokenTree, YggdrasilRule};

use tree_layout::{layout, NodeInfo, Point, TreeBox};

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
        TreeBox::rectangle(chars * 8.0, 16.0)
    }
    fn border(&self, _: TokenPair<'i, R>) -> TreeBox {
        TreeBox::square(8.0)
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
        let mut document = Document::new();
        let root = self.cst.clone().into_iter().next().unwrap();
        let layout = layout(self, root);
        let mut max = Point::default();
        for node in layout {
            let area = node.data.boundary();
            if area.max.x > max.x {
                max.x = area.max.x;
            }
            if area.max.y > max.y {
                max.y = area.max.y;
            }
            let pair = node.data.key.clone();
            let mut text = Text::new().set("x", area.min.x + area.width() / 2.0).set("y", area.min.y + area.height() / 2.0);
            if pair.has_child() {
                text = text.add(svg::node::Text::new(format!("{:?}", pair.get_rule()))).set("class", "node");
                document = document.add(area.to_svg().set("rx", 5).set("ry", 5).set("class", "node"));
            }
            else {
                text = text.add(svg::node::Text::new(format!("{}", pair.get_string()))).set("class", "leaf");
                document = document.add(area.to_svg().set("rx", 5).set("ry", 5).set("class", "leaf"));
            }
            document = document.add(text);
        }
        document.add(svg::node::element::Style::new(include_str!("style.css"))).set("viewBox", (0, 0, max.x, max.y))
    }
}
