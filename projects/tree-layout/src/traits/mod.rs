use crate::Coordinate;

#[allow(unused_variables)]
pub trait NodeInfo<N> {
    type Key: Eq;

    fn key(&self, node: &N) -> Self::Key;

    fn children(&self, node: &N) -> impl Iterator<Item = N>;

    fn width(&self, node: &N) -> Coordinate {
        1.0
    }

    fn height(&self, node: &N) -> Coordinate {
        1.0
    }
}
