#![no_std]

extern crate alloc;

use core::hash::Hash;

use shape_core::Rectangle;

pub use crate::tree::{layout, layout_position};

mod tree;

#[allow(unused_variables)]
pub trait NodeInfo<N>
where
    Self::Index: Eq + Hash,
    N: Copy,
{
    type Index;
    type Children: IntoIterator<Item = N>;
    fn query(&self, node: N) -> Self::Index;
    fn children(&self, node: N) -> Self::Children;
    fn dimensions(&self, node: N) -> Rectangle<f64> {
        Rectangle::from_origin(0.0, 0.0)
    }
    fn border(&self, node: N) -> Rectangle<f64> {
        Rectangle::from_origin(1.0, 1.0)
    }
}
