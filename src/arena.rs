use crate::{
    access::NodeAccess,
    id::{Idx, MovesCount, NodeId},
    node::{Compare, Node},
};
use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

/// Arena is the heap allocated tree structure
/// each node can be in relashionship with other nodes.
pub struct Arena<T: Debug> {
    pub nodes: Vec<Node<T>>,
    pub(crate) moved_idxs: HashMap<Idx, MovesCount>,
}

impl<'a, T: Debug> Arena<T> {
    /// Returns an empty arena
    ///
    /// # Example
    ///
    /// ```
    /// # use acreage::*;
    /// # use acreage::prelude::*;
    ///
    /// let mut arena: Arena::<i32> = Arena::new();
    /// ```
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            moved_idxs: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            moved_idxs: HashMap::new(),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    /// Get the number of nodes in the arena.
    ///
    /// # Example
    ///
    /// ```
    /// # use acreage::*;
    /// # use acreage::prelude::*;
    ///
    /// let mut arena = Arena::new();
    /// arena.append_root(Node::new(0));
    /// arena.append_root(Node::new(1));
    /// assert_eq!(arena.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns a reference to the arena's vector of nodes.
    pub fn as_nodes(&self) -> &Vec<Node<T>> {
        &self.nodes
    }

    /// Returns a mutable reference to the arena's vector of nodes.
    pub fn as_mut_nodes(&mut self) -> &mut Vec<Node<T>> {
        &mut self.nodes
    }

    /// Rawly push a node into the arena.
    /// WARNING: don't use to append nodes to the arena.
    pub(crate) fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    /// Returns true if the arena has 0 nodes.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Remove every node in the arena and invalidates all ids.
    ///
    /// # Example
    ///
    /// ```
    /// # use acreage::*;
    /// # use acreage::prelude::*;
    ///
    /// let mut arena = Arena::new();
    /// let node = arena.append_root(Node::new(1));
    /// arena.clear();
    /// assert!(!node.validate(&arena));
    /// ```
    pub fn clear(&mut self) {
        for idx in 0..self.len() {
            self.moved_idxs
                .entry(idx)
                .and_modify(|mc| *mc += 1)
                .or_insert(1);
        }
        self.nodes.clear();
    }

    /// Get a shared reference to a node if the id is valide.
    pub fn get(&self, id: &NodeId) -> Option<&Node<T>> {
        if id.validate(&self) {
            return self.nodes.get(id.idx);
        }
        None
    }

    /// Get a mutable reference to a node if the id is valide.
    pub fn get_mut(&mut self, id: &NodeId) -> Option<&mut Node<T>> {
        if id.validate(&self) {
            return self.nodes.get_mut(id.idx);
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Node<T>> {
        self.nodes.iter()
    }

    /// Returns a mutable iterator over the vector of nodes.
    ///
    /// # Example
    ///
    /// ```
    /// # use acreage::*;
    /// # use acreage::prelude::*;
    ///
    /// let mut arena = Arena::new();
    /// arena.append_root(Node::new(0));
    /// arena.append_root(Node::new(1));
    /// arena.append_root(Node::new(2));
    ///
    /// for mut node in arena.iter_mut() {
    ///		*node.as_mut_content() += 1;
    /// }
    ///
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Node<T>> {
        self.nodes.iter_mut()
    }

    pub(crate) fn next_idx(&self) -> usize {
        self.len()
    }

    /// Append a root node to the arena. Always returns the idx of
    /// the node.
    ///
    /// # Example
    ///
    /// ```
    /// # use acreage::*;
    /// # use acreage::prelude::*;
    ///
    /// let mut arena = Arena::new();
    /// arena.append_root(Node::new(0));
    /// let node = arena.append_root(Node::new(1));
    /// assert_eq!(node.as_content(&arena), &1);
    /// ```
    pub fn append_root(&mut self, node: impl Into<NodeAccess<'a, T>>) -> NodeId {
        let first = self
            .iter()
            .position(|n| n.prev.is_none() && n.parent.is_none());

        let new_idx = self.next_idx();

        if let Some(first_idx) = first {
            let last = {
                let mut idx = first_idx;
                while let Some(i) = self[idx].next {
                    idx = i;
                }
                idx
            };

            self[last].next = Some(new_idx);

            match node.into() {
                NodeAccess::ById(id) => {
                    // should ensure its properly detached
                    if let Some(n) = self.get_mut(id) {
                        n.prev = Some(last);
                    }
                    todo!("detach first here");
                }
                NodeAccess::Owned(mut node) => {
                    node.prev = Some(last);
                    self.push(node);
                }
            }
        } else {
            assert!(self.is_empty());
            match node.into() {
                NodeAccess::ById(id) => {
                    assert!(!id.validate(&self));
                    panic!("Invalide id");
                }
                NodeAccess::Owned(n) => {
                    self.push(n);
                }
            }
        }

        NodeId {
            idx: new_idx,
            at_mc: 0,
        }
    }

    /// Inserts a root node at any position.
    /// The position of the node is its semantic position.
    ///
    /// Returns the vector index of the root node
    ///
    /// # Example
    ///
    /// ```
    /// # use acreage::*;
    /// # use acreage::prelude::*;
    ///
    /// let mut arena = Arena::new();
    /// arena.append_root(Node::new("Root 1"));
    /// arena.append_root(Node::new("Root 2"));
    /// arena.insert_root(0, Node::new("First root"));
    ///
    /// ```
    pub fn insert_root(&mut self, position: usize, node: impl Into<NodeAccess<'a, T>>) -> NodeId {
        let repl = self
            .iter()
            .position(|n| n.prev.is_none() && n.parent.is_none());

        let new_idx = self.next_idx();

        if let Some(repl_idx) = repl {
            let mut counter = 0;
            let mut repl_idx = repl_idx;
            while let Some(i) = self[repl_idx].next && counter != position {
                repl_idx = i;
                counter += 1;
            }

            // repl_idx is now the idx of the node at the current `position`

            let prev = self[repl_idx].prev.clone();

            match node.into() {
                NodeAccess::ById(id) => {
                    if let Some(n) = self.get_mut(id) {
                        n.prev = prev;
                        n.next = Some(repl_idx);
                    }
                }
                NodeAccess::Owned(mut n) => {
                    n.prev = prev;
                    n.next = Some(repl_idx);
                    self.push(n);
                }
            }

            self[repl_idx].prev = Some(new_idx);
        }

        NodeId {
            idx: new_idx,
            at_mc: 0,
        }
    }

    //pub fn find_nodes(&self, ident: &impl Compare<T>) -> impl Iterator<Item = (NodeId, &Node<T>)> {}

    //pub fn filter(&self, ident: &impl Compare<T>) -> impl Iterator<Item = Node<T>> {}
}

impl<T: Debug> Index<usize> for Arena<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Node<T> {
        &self.nodes[index]
    }
}

impl<T: Debug> IndexMut<usize> for Arena<T> {
    fn index_mut(&mut self, index: usize) -> &mut Node<T> {
        &mut self.nodes[index]
    }
}
