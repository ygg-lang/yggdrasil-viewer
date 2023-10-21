use std::ptr::NonNull;

use num::Float;
use tinyset::SetUsize;

use crate::{node::LayoutData, utils::erase_lifetime, Coordinate, LayoutNode};

use super::linked_y_list::LinkedYList;

pub struct LayoutConfig {
    /// margin between parent and child
    pub margin: Coordinate,
    pub peer_margin: Coordinate,
    pub is_layered: bool,
    /// only for layered layout
    pub depth_to_y: Vec<Coordinate>,
}

impl LayoutConfig {
    pub fn new(margin: Coordinate, peer_margin: Coordinate) -> Self {
        LayoutConfig { margin, peer_margin, is_layered: false, depth_to_y: vec![] }
    }
    pub fn with_layered(self, layered: bool) -> Self {
        Self { is_layered: layered, ..self }
    }
}

struct Contour {
    is_left: bool,
    pub current: Option<NonNull<LayoutNode>>,
    modifier_sum: Coordinate,
}

impl Contour {
    pub fn new(is_left: bool, current: &LayoutNode) -> Self {
        Contour { is_left, current: Some(current.into()), modifier_sum: current.get_layout().modifier_to_subtree }
    }

    fn node(&self) -> &LayoutNode {
        match self.current {
            Some(node) => {
                let node = unsafe { node.as_ref() };
                node
            }
            None => panic!(),
        }
    }

    pub fn is_none(&self) -> bool {
        self.current.is_none()
    }

    pub fn left(&self) -> Coordinate {
        let node = self.node();
        self.modifier_sum + node.relative_x - node.width / 2.
    }

    pub fn right(&self) -> Coordinate {
        let node = self.node();
        self.modifier_sum + node.relative_x + node.width / 2.
    }

    pub fn bottom(&self) -> Coordinate {
        match self.current {
            Some(node) => {
                let node = unsafe { node.as_ref() };
                node.point.y + node.height
            }
            None => 0.,
        }
    }

    pub fn next(&mut self) {
        if let Some(mut current) = self.current {
            let node = unsafe { current.as_mut() };
            if self.is_left {
                if !node.children.is_empty() {
                    self.current = Some((&**node.children.first().unwrap()).into());
                    let node = self.node();
                    self.modifier_sum += node.layout_data.as_ref().unwrap().modifier_to_subtree;
                }
                else {
                    self.modifier_sum += node.get_layout().modifier_thread_left;
                    self.current = node.get_layout().thread_left;
                }
            }
            else if !node.children.is_empty() {
                self.current = Some((&**node.children.last().unwrap()).into());
                let node = self.node();
                self.modifier_sum += node.layout_data.as_ref().unwrap().modifier_to_subtree;
            }
            else {
                self.modifier_sum += node.get_layout().modifier_thread_right;
                self.current = node.get_layout().thread_right;
            }
            if self.current.is_some() {
                let _ = self.node();
            }
        }
    }
}

impl LayoutNode {
    fn set_extreme(&mut self) {
        let self_ptr: NonNull<LayoutNode> = self.into();
        let tidy = self.layout_data.as_mut().unwrap();
        if self.children.is_empty() {
            tidy.extreme_left = Some(self_ptr);
            tidy.extreme_right = Some(self_ptr);
            tidy.modifier_extreme_left = 0.;
            tidy.modifier_extreme_right = 0.;
        }
        else {
            let first = self.children.first().unwrap().layout_data.as_ref().unwrap();
            tidy.extreme_left = first.extreme_left;
            tidy.modifier_extreme_left = first.modifier_to_subtree + first.modifier_extreme_left;
            let last = self.children.last().unwrap().layout_data.as_ref().unwrap();
            tidy.extreme_right = last.extreme_right;
            tidy.modifier_extreme_right = last.modifier_to_subtree + last.modifier_extreme_right;
        }
    }

    fn extreme_left(&mut self) -> &mut LayoutNode {
        unsafe { self.layout_data.as_mut().unwrap().extreme_left.as_mut().unwrap().as_mut() }
    }

    fn extreme_right(&mut self) -> &mut LayoutNode {
        unsafe { self.layout_data.as_mut().unwrap().extreme_right.as_mut().unwrap().as_mut() }
    }

    fn position_root(&mut self) {
        let first = self.children.first().unwrap();
        let first_child_pos = first.relative_x + first.get_layout().modifier_to_subtree;
        let last = self.children.last().unwrap();
        let last_child_pos = last.relative_x + last.get_layout().modifier_to_subtree;
        self.relative_x = (first_child_pos + last_child_pos) / 2.;
        // make modifier_to_subtree + relative_x = 0. so that
        // there will always be collision in `separation()`'s first loop
        self.mut_layout().modifier_to_subtree = -self.relative_x;
    }

    fn add_child_spacing(&mut self) {
        let mut speed = 0.;
        let mut delta = 0.;
        for child in &mut self.children.iter_mut() {
            let child = child.mut_layout();
            speed += child.shift_acceleration;
            delta += speed + child.shift_change;
            child.modifier_to_subtree += delta;
            child.shift_acceleration = 0.;
            child.shift_change = 0.;
        }
    }
}

impl LayoutConfig {
    fn separate(&mut self, node: &mut LayoutNode, child_index: usize, mut y_list: LinkedYList) -> LinkedYList {
        // right contour of the left
        let mut left = Contour::new(false, &node.children[child_index - 1]);
        // left contour of the right
        let mut right = Contour::new(true, &node.children[child_index]);
        while !left.is_none() && !right.is_none() {
            if left.bottom() > y_list.bottom() {
                let b = y_list.bottom();
                let top = y_list.pop();
                if top.is_none() {
                    println!("Err\n\n{}\n\nleft.bottom={}\nyList.bottom={}", node.str(), left.bottom(), b);
                }

                y_list = top.unwrap();
            }

            let dist = left.right() - right.left() + self.peer_margin;
            if dist > 0. {
                // left and right are too close. move right part with distance of dist
                right.modifier_sum += dist;
                self.move_subtree(node, child_index, y_list.index, dist);
            }

            let left_bottom = left.bottom();
            let right_bottom = right.bottom();
            if left_bottom <= right_bottom {
                left.next();
            }
            if left_bottom >= right_bottom {
                right.next();
            }
        }

        if left.is_none() && !right.is_none() {
            self.set_left_thread(node, child_index, right.node(), right.modifier_sum);
        }
        else if !left.is_none() && right.is_none() {
            self.set_right_thread(node, child_index, left.node(), left.modifier_sum);
        }

        y_list
    }

    fn set_left_thread(&mut self, node: &mut LayoutNode, current_index: usize, target: &LayoutNode, modifier: Coordinate) {
        let first = erase_lifetime!(node.children[0]);
        let current = &mut node.children[current_index];
        let diff = modifier - first.mut_layout().modifier_extreme_left - first.mut_layout().modifier_to_subtree;
        first.extreme_left().mut_layout().thread_left = Some(target.into());
        first.extreme_left().mut_layout().modifier_thread_left = diff;
        first.mut_layout().extreme_left = current.mut_layout().extreme_left;
        first.mut_layout().modifier_extreme_left = current.mut_layout().modifier_extreme_left
            + current.mut_layout().modifier_to_subtree
            - first.mut_layout().modifier_to_subtree;
    }

    fn set_right_thread(&mut self, node: &mut LayoutNode, current_index: usize, target: &LayoutNode, modifier: Coordinate) {
        let current = erase_lifetime!(node.children[current_index]);
        let diff = modifier - current.mut_layout().modifier_extreme_right - current.mut_layout().modifier_to_subtree;
        current.extreme_right().mut_layout().thread_right = Some(target.into());
        current.extreme_right().mut_layout().modifier_thread_right = diff;
        let prev = node.children[current_index - 1].mut_layout();
        current.mut_layout().extreme_right = prev.extreme_right;
        current.mut_layout().modifier_extreme_right =
            prev.modifier_extreme_right + prev.modifier_to_subtree - current.mut_layout().modifier_to_subtree;
    }

    fn move_subtree(&mut self, node: &mut LayoutNode, current_index: usize, from_index: usize, distance: Coordinate) {
        let child = &mut node.children[current_index];
        let child_tidy = child.mut_layout();
        // debug_assert!(distance <= 1e6);
        child_tidy.modifier_to_subtree += distance;

        // distribute extra space to nodes between from_index to current_index
        if from_index != current_index - 1 {
            let index_diff = (current_index - from_index) as Coordinate;
            node.children[from_index + 1].mut_layout().shift_acceleration += distance / index_diff;
            node.children[current_index].mut_layout().shift_acceleration -= distance / index_diff;
            node.children[current_index].mut_layout().shift_change -= distance - distance / index_diff;
        }
    }

    fn set_y_recursive(&mut self, root: &mut LayoutNode) {
        if !self.is_layered {
            root.pre_order_traversal_mut(|node| {
                self.set_y(node);
            });
        }
        else {
            let depth_to_y = &mut self.depth_to_y;
            depth_to_y.clear();
            let margin = self.margin;
            root.bfs_traversal_with_depth_mut(|node, depth| {
                while depth >= depth_to_y.len() {
                    depth_to_y.push(0.);
                }

                if node.parent.is_none() || depth == 0 {
                    node.point.y = 0.0;
                    return;
                }

                let parent = node.get_parent().unwrap();
                depth_to_y[depth] = Float::max(depth_to_y[depth], depth_to_y[depth - 1] + parent.height + margin);
            });
            root.pre_order_traversal_with_depth_mut(|node, depth| {
                node.point.y = depth_to_y[depth];
            })
        }
    }

    fn set_y(&mut self, node: &mut LayoutNode) {
        node.point.y = if let Some(parent) = node.parent {
            let parent_bottom = unsafe { parent.as_ref().bottom() };
            parent_bottom + self.margin
        }
        else {
            0.0
        };
    }

    fn first_walk(&mut self, node: &mut LayoutNode) {
        if node.children.is_empty() {
            node.set_extreme();
            return;
        }

        self.first_walk(node.children.first_mut().unwrap());
        let mut y_list = LinkedYList::new(0, node.children[0].extreme_right().bottom());
        for i in 1..node.children.len() {
            let current_child = node.children.get_mut(i).unwrap();
            self.first_walk(current_child);
            let max_y = current_child.extreme_left().bottom();
            y_list = self.separate(node, i, y_list);
            y_list = y_list.update(i, max_y);
        }

        node.position_root();
        node.set_extreme();
    }

    fn first_walk_with_filter(&mut self, node: &mut LayoutNode, set: &SetUsize) {
        if !set.contains(node as *const _ as usize) {
            invalidate_extreme_thread(node);
            return;
        }

        if node.children.is_empty() {
            node.set_extreme();
            return;
        }

        self.first_walk_with_filter(node.children.first_mut().unwrap(), set);
        let mut y_list = LinkedYList::new(0, node.children[0].extreme_right().bottom());
        for i in 1..node.children.len() {
            let current_child = node.children.get_mut(i).unwrap();
            current_child.mut_layout().modifier_to_subtree = -current_child.relative_x;
            self.first_walk_with_filter(current_child, set);
            let max_y = current_child.extreme_left().bottom();
            y_list = self.separate(node, i, y_list);
            y_list = y_list.update(i, max_y);
        }

        node.position_root();
        node.set_extreme();
    }

    fn second_walk(&mut self, node: &mut LayoutNode, mut mod_sum: Coordinate) {
        mod_sum += node.mut_layout().modifier_to_subtree;
        node.point.x = node.relative_x + mod_sum;
        node.add_child_spacing();

        for child in node.children.iter_mut() {
            self.second_walk(child, mod_sum);
        }
    }

    fn second_walk_with_filter(&mut self, node: &mut LayoutNode, mut mod_sum: Coordinate, set: &SetUsize) {
        mod_sum += node.mut_layout().modifier_to_subtree;
        let new_x = node.relative_x + mod_sum;
        if (new_x - node.point.x).abs() < 1e-8 && !set.contains(node as *const _ as usize) {
            return;
        }

        node.point.x = new_x;
        node.add_child_spacing();

        for child in node.children.iter_mut() {
            self.second_walk_with_filter(child, mod_sum, set);
        }
    }
}

impl LayoutConfig {
    pub fn layout(&mut self, root: &mut LayoutNode) {
        root.pre_order_traversal_mut(init_node);
        self.set_y_recursive(root);
        self.first_walk(root);
        self.second_walk(root, 0.);
    }

    pub fn partial_layout(&mut self, root: &mut crate::LayoutNode, changed: &[std::ptr::NonNull<crate::LayoutNode>]) {
        // not implemented for layered
        if self.is_layered {
            self.layout(root);
            return;
        }

        for node in changed.iter() {
            let node = unsafe { &mut *node.as_ptr() };
            if node.layout_data.is_none() {
                init_node(node);
            }

            // TODO: can be lazy
            self.set_y_recursive(node);
        }

        let mut set: SetUsize = SetUsize::new();
        for node in changed.iter() {
            set.insert(node.as_ptr() as usize);
            let mut node = unsafe { &mut *node.as_ptr() };
            while node.parent.is_some() {
                invalidate_extreme_thread(node);
                set.insert(node.parent.unwrap().as_ptr() as usize);
                node = node.mut_parent().unwrap();
            }
        }

        self.first_walk_with_filter(root, &set);
        // TODO: this can be optimized with onscreen detection,
        // then all nodes' absolute x position can be evaluate lazily
        self.second_walk_with_filter(root, 0., &set);
    }
}

fn init_node(node: &mut LayoutNode) {
    if node.layout_data.is_some() {
        let tidy = node.mut_layout();
        tidy.extreme_left = None;
        tidy.extreme_right = None;
        tidy.shift_acceleration = 0.;
        tidy.shift_change = 0.;
        tidy.modifier_to_subtree = 0.;
        tidy.modifier_extreme_left = 0.;
        tidy.modifier_extreme_right = 0.;
        tidy.thread_left = None;
        tidy.thread_right = None;
        tidy.modifier_thread_left = 0.;
        tidy.modifier_thread_right = 0.;
    }
    else {
        node.layout_data = Some(Box::new(LayoutData {
            extreme_left: None,
            extreme_right: None,
            shift_acceleration: 0.,
            shift_change: 0.,
            modifier_to_subtree: 0.,
            modifier_extreme_left: 0.,
            modifier_extreme_right: 0.,
            thread_left: None,
            thread_right: None,
            modifier_thread_left: 0.,
            modifier_thread_right: 0.,
        }));
    }

    node.point.x = 0.0;
    node.point.y = 0.0;
    node.relative_x = 0.;
    node.relative_y = 0.;
}

fn invalidate_extreme_thread(node: &mut LayoutNode) {
    node.set_extreme();
    let e_left = node.extreme_left().mut_layout();
    e_left.thread_left = None;
    e_left.thread_right = None;
    e_left.modifier_thread_left = 0.;
    e_left.modifier_thread_right = 0.;
    let e_right = node.extreme_right().mut_layout();
    e_right.thread_left = None;
    e_right.thread_right = None;
    e_right.modifier_thread_left = 0.;
    e_right.modifier_thread_right = 0.;
}

#[cfg(test)]
mod test {
    use crate::node::LayoutNode;

    use super::*;

    #[test]
    fn test_tidy_layout() {
        let mut tidy = LayoutConfig::new(1., 1.);
        let mut root = LayoutNode::new(0, 1., 1.);
        let first_child =
            LayoutNode::new_with_child(1, 1., 1., LayoutNode::new_with_child(10, 1., 1., LayoutNode::new(100, 1., 1.)));
        root.append_child(first_child);

        let second =
            LayoutNode::new_with_child(2, 1., 1., LayoutNode::new_with_child(11, 1., 1., LayoutNode::new(101, 1., 1.)));
        root.append_child(second);

        root.append_child(LayoutNode::new(3, 1., 2.));
        tidy.layout(&mut root);
        println!("{}", root.str());
    }
}
