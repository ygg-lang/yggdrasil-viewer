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
    let text = r##"
class ClassStatement {
    DecoratorCall* ModifierCall* ^KW_CLASS (name:Identifier)
}
"##;
    let cst = BootstrapParser::parse_cst(text, BootstrapRule::Root).unwrap();
    // println!("Short Form:\n{}", cst);
    let tree = plotter.draw(cst);
    svg::save("tests/bootstrap.svg", &tree).unwrap();
}

// fn main() {
//     let root = layered();
//     let layout = layout_position(&Tree, &root);
//     println!("{:?}", layout)
// }
