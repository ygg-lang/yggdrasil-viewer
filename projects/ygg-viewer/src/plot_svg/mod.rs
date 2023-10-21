use std::{borrow::Cow, cmp::max};

use shape_svg::ToSVG;
use svg::{
    node::element::{Text, SVG},
    Document,
};
use tree_layout::{Coordinate, LayoutConfig, Line, Point, TreeArena, TreeInfo};
use yggdrasil_rt::{TokenPair, TokenTree, YggdrasilRule};

/// Plot a svg structure
#[derive(Debug, Default)]
pub struct SvgPlotter {
    color: String,
}

#[derive(Debug)]
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

    fn root(&self) -> Cow<Self::Node> {
        Cow::Owned(self.cst.clone().into_iter().next().unwrap())
    }

    fn children<'a>(&self, node: &'a Self::Node) -> impl Iterator<Item = Cow<'a, Self::Node>> {
        node.clone().into_inner().map(Cow::Owned)
    }

    fn width(&self, _: &Self::Node) -> Coordinate {
        1.0
    }
    fn height(&self, _: &Self::Node) -> Coordinate {
        1.0
    }
}

fn width_hint<R>(node: TokenPair<R>) -> f64
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
        println!("{:?}", self);
        let root = TreeArena::build(self, &LayoutConfig::new(10.0, 10.0));
        let mut max = Point::default();
        for (node, pair) in root.into_iter() {
            let area = node.boundary();
            if area.max.x > max.x {
                max.x = area.max.x;
            }
            if area.max.y > max.y {
                max.y = area.max.y;
            }

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
        document.add(svg::node::element::Style::new(include_str!("style.css"))).set("viewBox", (0, 0, max.x, max.y))
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
