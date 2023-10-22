use crate::{id::NodeId, node::Node};
use std::fmt::Debug;

pub enum NodeAccess<'a, T: Debug> {
    ById(&'a NodeId),
    Owned(Node<T>),
}

impl<'a, T: Debug> From<&'a NodeId> for NodeAccess<'a, T> {
    fn from(item: &'a NodeId) -> Self {
        Self::ById(item)
    }
}

impl<'a, T: Debug> From<Node<T>> for NodeAccess<'a, T> {
    fn from(item: Node<T>) -> Self {
        Self::Owned(item)
    }
}
