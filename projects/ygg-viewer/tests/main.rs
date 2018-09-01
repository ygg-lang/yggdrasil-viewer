use std::str::FromStr;
use yggdrasil_parser::bootstrap::{BootstrapRule, ClassStatementNode};
use yggdrasil_parser::BootstrapParser;
use yggdrasil_rt::YggdrasilParser;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_classes() {
    let text = r##"text class RegexInner {
    A | B
}"##;
    let cst = BootstrapParser::parse_cst(text, BootstrapRule::ClassStatement).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = ClassStatementNode::from_str(text).unwrap();
    println!("{ast:#?}")
}