use crate::Coordinate;
use std::borrow::Cow;

#[allow(unused_variables)]
pub trait TreeInfo {
    type Node: Clone;

    fn root(&self) -> Self::Node;

    fn children(&self, node: &Self::Node) -> impl Iterator<Item = Self::Node>;

    fn count(&self) -> usize {
        1
    }

    fn width(&self, node: &Self::Node) -> Coordinate {
        1.0
    }

    fn height(&self, node: &Self::Node) -> Coordinate {
        1.0
    }
}
