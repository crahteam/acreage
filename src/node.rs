use crate::id::Idx;
use std::fmt::Debug;

/// An arena's node that can contain any content that implements Debug.
/// Stores indices to navigate both vertically and horizontally in an arena.
///
/// # Example
///
/// ```
/// # use acreage::*;
/// # use acreage::prelude::*;
///
/// let node = Node::new(1);
/// let mut arena = Arena::new();
/// arena.append_root(node);
/// ```
pub struct Node<T: Debug> {
    pub prev: Option<Idx>,
    pub next: Option<Idx>,
    pub parent: Option<Idx>,
    pub child: Option<Idx>,
    pub content: T,
}

/// Implement this for your own identifier to collect and filter the nodes
/// in the arena.
///
/// # Example
///
/// ```
/// use acreage::*;
/// use acreage::prelude::*;
///
/// #[derive(Debug)]
/// pub enum Identifier {
///    EqualTo(i32),
///    BiggerThan(i32),
///    SmallerThan(i32)
/// }
///
/// impl Compare::<i32> for Identifier {
///     fn compare(&self, content: &i32) -> bool {
///         match self {
///             Identifier::EqualTo(n) => n == content,
///             Identifier::BiggerThan(n) => content > n,
///             Identifier::SmallerThan(n) => content < n
///         }
///     }
/// }
/// ```
pub trait Compare<T: Debug> {
    fn compare(&self, content: &T) -> bool;
}

impl<T: Debug> Node<T> {
    /// Builds a new node still not attached to the arena.
    /// Doesn't have any relationship with other nodes,
    /// simply owns its content and waits to be appended.
    ///
    /// # Example
    ///
    /// ```
    /// use acreage::*;
    /// use acreage::prelude::*;
    ///
    /// ```
    pub fn new(content: T) -> Self {
        Self {
            prev: None,
            next: None,
            parent: None,
            child: None,
            content,
        }
    }

    pub fn as_content(&self) -> &T {
        &self.content
    }

    pub fn as_mut_content(&mut self) -> &mut T {
        &mut self.content
    }
}
