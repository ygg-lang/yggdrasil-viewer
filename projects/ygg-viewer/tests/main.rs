use std::str::FromStr;
use yggdrasil_parser::bootstrap::{BootstrapRule, ClassStatementNode};
use yggdrasil_parser::BootstrapParser;
use yggdrasil_rt::YggdrasilParser;
use tree_layout::layout_position;
use yggdrasil_viewer::SvgPlotter;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_classes() {
    let plotter = SvgPlotter::default();
    let text = r##"text class RegexInner {
    A | B
}"##;
    let cst = BootstrapParser::parse_cst(text, BootstrapRule::ClassStatement).unwrap();
    println!("Short Form:\n{}", cst);
    let tree = plotter.draw(cst);
    for node in tree.position() {
        node.data

    }



}

// fn main() {
//     let root = tree();
//     let layout = layout_position(&Tree, &root);
//     println!("{:?}", layout)
// }