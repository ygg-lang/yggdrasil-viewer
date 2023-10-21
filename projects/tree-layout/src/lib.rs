#![feature(return_position_impl_trait_in_trait)]

pub use crate::{
    arena::{ArenaIterator, TreeArena},
    layout::{LayoutConfig, TreeLayout},
    node::LayoutNode,
    traits::TreeInfo,
    traverse::Traverse,
};

mod arena;
mod layout;
mod node;
mod traits;
mod traverse;
mod utils;

pub type Coordinate = f64;
pub const NULL_ID: usize = usize::MAX;

pub type Point = shape_core::Point<Coordinate>;
pub type Rectangle = shape_core::Rectangle<Coordinate>;
pub type Line = shape_core::Line<Coordinate>;
