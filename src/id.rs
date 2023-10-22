use crate::access::NodeAccess;
use crate::{arena::Arena, node::Node};
use std::collections::hash_map::Entry;
use std::fmt::Debug;

pub type Idx = usize;
pub type MovesCount = usize;

/// Represents an arena smart `pointer` to a node.
/// Intelligently stores a stamp on the `MovesCount`
/// of the node it points to at the time it was retrived.
///
/// Can only be retrived by appending a node to the arena.
/// It can't be rawly instantiatied.
///
/// # Example
///
/// ```
/// # use acreage::prelude::*;
/// # use acreage::*;
/// let mut arena = Arena::new();
/// let id: NodeId = arena.append_root(Node::new(0));
/// ```
#[derive(Clone)]
pub struct NodeId {
    pub(crate) idx: Idx,
    pub(crate) at_mc: MovesCount,
}

impl NodeId {
    /// Checks if the id is still pointing to
    /// its original node.
    pub fn validate<T: Debug>(&self, arena: &Arena<T>) -> bool {
        if arena.moved_idxs.contains_key(&self.idx) {
            // safe unwraps
            return arena.moved_idxs.get(&self.idx).unwrap() == &self.at_mc;
        }
        true
    }

    pub fn as_content<'a, T: Debug>(&'a self, arena: &'a Arena<T>) -> &'a T {
        if self.validate(&arena) {
            return &arena.get(&self).unwrap().as_content();
        } else {
            panic!()
        }
    }
}

impl<'a, T: Debug> AppendNode<'a, T> for NodeId {
    ///
    fn append_child(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId {
        if !self.validate(&arena) {
            panic!("pinguino offeso");
        }

        let new_idx = arena.next_idx();
        if let Some(c) = arena.get(&self).unwrap().child {
            let mut last_child_idx = c;

            while let Some(i) = arena[c].next {
                last_child_idx = i;
            }

            match node.into() {
                NodeAccess::ById(id) => {
                    if let Some(n) = arena.get_mut(id) {
                        n.prev = Some(last_child_idx);
                        n.parent = Some(self.idx);
                        arena[last_child_idx].next = Some(id.idx);
                    }
                    return id.clone();
                }
                NodeAccess::Owned(mut n) => {
                    n.prev = Some(last_child_idx);
                    n.parent = Some(self.idx);
                    arena[last_child_idx].next = Some(new_idx);
                    arena.push(n);
                    return NodeId {
                        idx: new_idx,
                        at_mc: 0,
                    };
                }
            }
        } else {
            match node.into() {
                NodeAccess::ById(id) => {
                    if let Some(n) = arena.get_mut(&id) {
                        n.parent = Some(self.idx);
                        arena[self.idx].child = Some(id.idx);
                    }
                    return id.clone();
                }
                NodeAccess::Owned(mut n) => {
                    n.parent = Some(self.idx);
                    arena[self.idx].child = Some(new_idx);
                    arena.push(n);
                    return NodeId {
                        idx: new_idx,
                        at_mc: 0,
                    };
                }
            }
        }
    }
}

pub trait AppendNode<'a, T: Debug> {
    fn append_child(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId;
}

pub trait InsertNode<'a, T: Debug> {
    fn insert_child(
        &self,
        position: usize,
        node: impl Into<NodeAccess<'a, T>>,
        arena: &mut Arena<T>,
    ) -> NodeId;
    fn insert_prev(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId;
    fn insert_next(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId;
}

pub trait DetachNode<T: Debug> {
    /// drops from the vec and
    fn detach(&self, arena: &mut Arena<T>) -> NodeId;
    fn remove(self, arena: &mut Arena<T>);
}
