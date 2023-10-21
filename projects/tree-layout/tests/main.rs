#![feature(test)]
#![allow(soft_unstable, unused)]
extern crate test;

mod aesthetic_rules;
mod binary;
mod generator;
mod layout_bench;
mod layout_test;

use rand::prelude::*;

use crate::generator::{gen_node, gen_tree};
use rand::{prelude::StdRng, SeedableRng};
use std::{panic::catch_unwind, ptr::NonNull, time::Instant};
use test::bench::{black_box, Bencher};
use tree_layout::{Coordinate, LayoutConfig, LayoutNode};
#[test]
fn test() {}
