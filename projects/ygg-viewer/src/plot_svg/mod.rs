use std::{borrow::Cow, cmp::max};

use shape_svg::ToSVG;
use svg::{
    node::element::{Text, SVG},
    Document,
};
use tree_layout::{Coordinate, LayoutConfig, Line, Point, Rectangle, TreeArena, TreeInfo};
use yggdrasil_rt::{TokenPair, TokenTree, YggdrasilRule};

/// Plot a svg structure
#[derive(Debug, Default)]
pub struct SvgPlotter {
    color: String,
}

#[derive(Clone, Debug)]
struct SvgTree<'i, R>
where
    R: YggdrasilRule,
{
    cst: TokenTree<'i, R>,
}

impl<'i, R> TreeInfo for SvgTree<'i, R>
where
    R: YggdrasilRule,
{
    type Node = TokenPair<'i, R>;

    fn root(&self) -> Self::Node {
        self.cst.clone().into_iter().next().unwrap()
    }

    fn children(&self, node: &Self::Node) -> impl Iterator<Item = Self::Node> {
        let mut out = vec![];
        for pair in node.clone().into_inner() {
            if pair.get_rule().is_ignore() {
                continue;
            };
            out.push(pair)
        }
        out.into_iter()
    }

    fn width(&self, node: &Self::Node) -> Coordinate {
        width_hint(node) * 16.0
    }
    fn height(&self, _: &Self::Node) -> Coordinate {
        20.0
    }
}

fn width_hint<R>(node: &TokenPair<R>) -> f64
where
    R: YggdrasilRule,
{
    let text = if node.has_child(false) { format!("{:?}", node.get_rule()) } else { node.get_string() };
    max(text.len(), 3) as f64
}

impl<'i, R> SvgTree<'i, R>
where
    R: YggdrasilRule,
{
    fn as_svg(&self) -> SVG {
        let mut document = Document::new();

        let root = TreeArena::build(self.clone(), &LayoutConfig::new(10.0, 10.0));
        let mut old = Rectangle::empty();
        for (node, pair) in root.into_iter() {
            println!("{:?}: {}", pair.get_rule(), pair.as_str());
            let area = node.boundary();
            old &= area;

            // match root.find_parent(&node) {
            //     Some(s) => {
            //         let parent_box = s.data.boundary();
            //         let parent_lower = Point { x: (parent_box.min.x + parent_box.max.x) / 2.0, y: parent_box.max.y };
            //         let this_upper = Point { x: (area.min.x + area.max.x) / 2.0, y: area.min.y };
            //         document = document.add(Line::new(parent_lower, this_upper).to_svg())
            //     }
            //     None => {}
            // }

            let mut text = Text::new().set("x", area.min.x + area.width() / 2.0).set("y", area.min.y + area.height() / 2.0);
            if pair.has_child(false) {
                text = text.add(svg::node::Text::new(format!("{:?}", pair.get_rule()))).set("class", "node");
                document = document.add(area.to_svg().set("rx", 5).set("ry", 5).set("class", "node"));
            }
            else {
                text = text.add(svg::node::Text::new(format!("{}", pair.get_string()))).set("class", "leaf");
                document = document.add(area.to_svg().set("rx", 5).set("ry", 5).set("class", "leaf"));
            }
            document = document.add(text);
        }
        document
            .add(svg::node::element::Style::new(include_str!("style.css")))
            .set("viewBox", (old.min.x, old.min.y, old.max.x, old.max.y))
    }
}

impl SvgPlotter {
    /// Draw a svg
    pub fn draw<R>(&self, tree: TokenTree<R>) -> SVG
    where
        R: YggdrasilRule,
    {
        SvgTree { cst: tree }.as_svg()
    }
}
