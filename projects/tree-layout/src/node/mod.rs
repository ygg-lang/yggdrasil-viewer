use std::{collections::VecDeque, ptr::NonNull};

use shape_core::{Point, Rectangle};

use crate::{layout::BoundingBox, Coordinate};

pub mod basic_layout;

#[derive(Debug)]
pub struct LayoutNode {
    pub id: usize,
    pub width: Coordinate,
    pub height: Coordinate,
    pub point: Point<Coordinate>,
    /// node x position relative to its parent
    pub relative_x: Coordinate,
    /// node y position relative to its parent
    pub relative_y: Coordinate,
    pub bbox: BoundingBox,
    pub parent: Option<NonNull<LayoutNode>>,
    /// Children need boxing to get a stable addr in the heap
    pub children: Vec<Box<LayoutNode>>,
    pub layout_data: Option<Box<LayoutData>>,
}

#[derive(Debug)]
pub struct LayoutData {
    pub thread_left: Option<NonNull<LayoutNode>>,
    pub thread_right: Option<NonNull<LayoutNode>>,
    /// ```text
    /// this.extreme_left == this.thread_left.extreme_left ||
    /// this.extreme_left == this.children[0].extreme_left
    /// ```
    pub extreme_left: Option<NonNull<LayoutNode>>,
    /// ```text
    /// this.extreme_right == this.thread_right.extreme_right ||
    /// this.extreme_right == this.children[-1].extreme_right
    /// ```
    pub extreme_right: Option<NonNull<LayoutNode>>,
    /// Cached change of x position.
    pub shift_acceleration: Coordinate,
    /// Cached change of x position
    pub shift_change: Coordinate,
    /// this.x = parent.x + modifier_to_subtree
    pub modifier_to_subtree: Coordinate,
    /// this.x + modifier_thread_left == thread_left.x
    pub modifier_thread_left: Coordinate,
    /// this.x + modifier_thread_right == thread_right.x
    pub modifier_thread_right: Coordinate,
    /// this.x + modifier_extreme_left == extreme_left.x
    pub modifier_extreme_left: Coordinate,
    /// this.x + modifier_extreme_right == extreme_right.x
    pub modifier_extreme_right: Coordinate,
}

impl Clone for LayoutNode {
    fn clone(&self) -> Self {
        let mut root = Self {
            id: self.id,
            width: self.width,
            height: self.height,
            point: self.point,
            relative_x: self.relative_x,
            relative_y: self.relative_y,
            bbox: self.bbox.clone(),
            parent: None,
            children: self.children.clone(),
            layout_data: None,
        };

        if self.parent.is_none() {
            root.post_order_traversal_mut(|node| {
                let node_ptr = node.into();
                for child in node.children.iter_mut() {
                    child.parent = Some(node_ptr);
                }
            });
        }

        root
    }
}

impl Default for LayoutNode {
    fn default() -> Self {
        Self {
            id: usize::MAX,
            width: 0.0,
            height: 0.0,
            point: Point::default(),
            relative_x: 0.0,
            relative_y: 0.0,
            children: vec![],
            parent: None,
            bbox: Default::default(),
            layout_data: None,
        }
    }
}

impl LayoutNode {
    pub fn new(id: usize, width: Coordinate, height: Coordinate) -> Self {
        LayoutNode { id, width, height, ..Default::default() }
    }

    pub fn new_with_child(id: usize, width: Coordinate, height: Coordinate, child: Self) -> Self {
        let mut node = LayoutNode::new(id, width, height);
        node.append_child(child);
        node
    }

    pub fn new_with_children(id: usize, width: Coordinate, height: Coordinate, children: Vec<Self>) -> Self {
        let mut node = LayoutNode::new(id, width, height);
        for child in children {
            node.append_child(child);
        }
        node
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut node = self;
        while node.parent.is_some() {
            node = node.get_parent().unwrap();
            depth += 1;
        }
        depth
    }
    pub fn boundary(&self) -> Rectangle<Coordinate> {
        Rectangle::from_center(self.point, self.width, self.height)
    }
    pub fn top_center(&self) -> Point<Coordinate> {
        Point { x: self.point.x, y: self.point.y - self.height }
    }
    pub fn bottom_center(&self) -> Point<Coordinate> {
        Point { x: self.point.x, y: self.point.y + self.height }
    }

    pub fn bottom(&self) -> Coordinate {
        self.height + self.point.y
    }
    pub fn get_parent(&self) -> Option<&Self> {
        unsafe { self.parent.map(|node| node.as_ref()) }
    }

    pub fn mut_parent(&mut self) -> Option<&mut Self> {
        unsafe { self.parent.map(|mut node| node.as_mut()) }
    }
    pub fn get_layout(&self) -> &LayoutData {
        self.layout_data.as_ref().unwrap()
    }
    pub fn mut_layout(&mut self) -> &mut LayoutData {
        self.layout_data.as_mut().unwrap()
    }

    fn reset_parent_link_of_children(&mut self) {
        if self.children.is_empty() {
            return;
        }

        let ptr = self.into();
        for child in self.children.iter_mut() {
            child.parent = Some(ptr);
        }
    }

    pub fn append_child(&mut self, mut child: Self) -> NonNull<Self> {
        child.parent = Some(self.into());
        let mut boxed = Box::new(child);
        boxed.reset_parent_link_of_children();
        let ptr = boxed.as_mut().into();
        self.children.push(boxed);
        ptr
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.point.x - self.width / 2.0 < other.point.x + other.width / 2.0
            && self.point.x + self.width / 2.0 > other.point.x - other.width / 2.0
            && self.point.y < other.point.y + other.height
            && self.point.y + self.height > other.point.y
    }

    pub fn post_order_traversal<F>(&self, mut f: F)
    where
        F: FnMut(&LayoutNode),
    {
        let mut stack: Vec<(NonNull<Self>, bool)> = vec![(self.into(), true)];
        while let Some((mut node_ptr, is_first)) = stack.pop() {
            let node = unsafe { node_ptr.as_mut() };
            if !is_first {
                f(node);
                continue;
            }

            stack.push((node_ptr, false));
            for child in node.children.iter_mut() {
                stack.push((child.as_mut().into(), true));
            }
        }
    }

    pub fn post_order_traversal_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut LayoutNode),
    {
        let mut stack: Vec<(NonNull<Self>, bool)> = vec![(self.into(), true)];
        while let Some((mut node_ptr, is_first)) = stack.pop() {
            let node = unsafe { node_ptr.as_mut() };
            if !is_first {
                f(node);
                continue;
            }

            stack.push((node_ptr, false));
            for child in node.children.iter_mut() {
                stack.push((child.as_mut().into(), true));
            }
        }
    }
    pub fn pre_order_traversal<F>(&self, mut f: F)
    where
        F: FnMut(&LayoutNode),
    {
        let mut stack: Vec<NonNull<Self>> = vec![self.into()];
        while let Some(mut node) = stack.pop() {
            let node = unsafe { node.as_mut() };
            f(node);
            for child in node.children.iter_mut() {
                stack.push(child.as_mut().into());
            }
        }
    }

    pub fn pre_order_traversal_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut LayoutNode),
    {
        let mut stack: Vec<NonNull<Self>> = vec![self.into()];
        while let Some(mut node) = stack.pop() {
            let node = unsafe { node.as_mut() };
            f(node);
            for child in node.children.iter_mut() {
                stack.push(child.as_mut().into());
            }
        }
    }

    pub fn bfs_traversal_with_depth_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut LayoutNode, usize),
    {
        let mut queue: VecDeque<(NonNull<Self>, usize)> = VecDeque::new();
        queue.push_back((self.into(), 0));
        while let Some((mut node, depth)) = queue.pop_front() {
            let node = unsafe { node.as_mut() };
            f(node, depth);
            for child in node.children.iter_mut() {
                queue.push_back((child.as_mut().into(), depth + 1));
            }
        }
    }

    pub fn remove_child(&mut self, id: usize) {
        let pos = self.children.iter().position(|node| node.id == id);
        if let Some(index) = pos {
            self.children.remove(index);
        }
    }

    pub fn pre_order_traversal_with_depth_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut LayoutNode, usize),
    {
        let mut stack: Vec<(NonNull<Self>, usize)> = vec![(self.into(), 0)];
        while let Some((mut node, depth)) = stack.pop() {
            let node = unsafe { node.as_mut() };
            f(node, depth);
            for child in node.children.iter_mut() {
                stack.push((child.as_mut().into(), depth + 1));
            }
        }
    }

    pub fn str(&self) -> String {
        let mut s = String::new();
        if self.layout_data.is_some() {
            s.push_str(&format!(
                "x: {}, y: {}, width: {}, height: {}, rx: {}, mod: {}, id: {}\n",
                self.point.x,
                self.point.y,
                self.width,
                self.height,
                self.relative_x,
                self.get_layout().modifier_to_subtree,
                self.id
            ));
        }
        else {
            s.push_str(&format!(
                "x: {}, y: {}, width: {}, height: {}, rx: {}, id: {}\n",
                self.point.x, self.point.y, self.width, self.height, self.relative_x, self.id
            ));
        }
        for child in self.children.iter() {
            for line in child.str().split('\n') {
                if line.is_empty() {
                    continue;
                }

                s.push_str(&format!("    {}\n", line));
            }
        }

        s
    }
}
