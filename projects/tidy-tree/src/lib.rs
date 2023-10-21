// #![allow(dead_code, unused_imports, unused_variables)]

mod layout;
mod node;
mod traverse;
mod utils;
pub use layout::TidyLayout;
pub use node::TidyNode;
use std::{collections::HashMap, ptr::NonNull};
pub use traverse::Traverse;

pub type Coordinate = f64;
pub const NULL_ID: usize = usize::MAX;

pub struct TreeLayout {
    root: TidyNode,
    layered: bool,
    layout: TidyLayout,
    map: HashMap<usize, NonNull<TidyNode>>,
}

impl TreeLayout {
    pub fn new(margin: Coordinate, peer_margin: Coordinate) -> Self {
        TreeLayout {
            layered: false,
            root: Default::default(),
            layout: TidyLayout::new(margin, peer_margin),
            map: HashMap::new(),
        }
    }

    pub fn new_layered(margin: Coordinate, peer_margin: Coordinate) -> Self {
        TreeLayout {
            layered: true,
            root: Default::default(),
            layout: TidyLayout::new(margin, peer_margin).with_layered(true),
            map: HashMap::new(),
        }
    }

    pub fn with_layered(mut self, layered: bool) -> Self {
        if layered != self.layered {
            self.layout = TidyLayout::new(self.layout.margin, self.layout.peer_margin).with_layered(layered);
            self.layered = layered;
        }
        return self;
    }

    pub fn is_empty(&self) -> bool {
        self.root.id == usize::MAX
    }

    pub fn add_node(&mut self, id: usize, width: Coordinate, height: Coordinate, parent_id: usize) {
        let node = TidyNode::new(id, width, height);
        if self.is_empty() || parent_id == usize::MAX {
            self.root = node;
            self.map.insert(id, (&self.root).into());
            return;
        }
        let mut parent = *self.map.get(&parent_id).unwrap();
        let parent = unsafe { parent.as_mut() };
        let ptr = parent.append_child(node);
        self.map.insert(id, ptr);
    }

    pub fn remove_node(&mut self, id: usize) {
        if self.is_empty() {
            return;
        }
        if let Some(node) = self.map.get(&id) {
            let node = unsafe { &mut *node.as_ptr() };
            node.pre_order_traversal(|node| {
                self.map.remove(&node.id);
            });
            node.parent_mut().unwrap().remove_child(id);
        }
    }

    pub fn data(&mut self, id: &[usize], width: &[Coordinate], height: &[Coordinate], parent_id: &[usize]) {
        for (i, &id) in id.iter().enumerate() {
            let width = width[i];
            let height = height[i];
            let parent_id = parent_id[i];
            self.add_node(id, width, height, parent_id);
        }
    }

    pub fn layout(&mut self) {
        if self.is_empty() {
            return;
        }
        self.layout.layout(&mut self.root);
    }

    pub fn get_position(&self) -> Vec<Coordinate> {
        let mut ans = vec![];
        for (id, node) in self.map.iter() {
            let node = unsafe { node.as_ref() };
            ans.push((*id) as Coordinate);
            ans.push(node.x);
            ans.push(node.y);
        }
        ans
    }
}
