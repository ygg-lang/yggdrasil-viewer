use shape_svg::ToSVG;
use svg::Document;
use yggdrasil_parser::{bootstrap::BootstrapRule, BootstrapParser};
use yggdrasil_rt::YggdrasilParser;

use yggdrasil_viewer::SvgPlotter;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_classes() {
    let plotter = SvgPlotter::default();
    let text = r##"text class Test {
    A | B
}"##;
    let cst = BootstrapParser::parse_cst(text, BootstrapRule::ClassStatement).unwrap();
    println!("Short Form:\n{}", cst);
    let tree = plotter.draw(cst);
    let mut document = Document::new().set("viewBox", (0, 0, 2000, 2000));
    for node in tree.position() {
        let tag = node.data.key.get_tag();
        let area = node.data.boundary().to_svg().set("fill", "none").set("stroke", "black").set("stroke-width", 0.1);
        document = document.add(area);
        let text = svg::node::element::Text::new()
            .set("x", node.data.center().x)
            .set("y", node.data.center().y)
            .add(svg::node::Text::new(format!("{:?}({})", node.data.key.get_rule(), node.data.key.get_string())));
        document = document.add(text);
    }

    svg::save("tests/bootstrap.svg", &document).unwrap();
}

// fn main() {
//     let root = tree();
//     let layout = layout_position(&Tree, &root);
//     println!("{:?}", layout)
// }
