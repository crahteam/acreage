#![feature(let_chains)]
pub mod access;
pub mod arena;
pub mod id;
pub mod macros;
pub mod node;

pub use crate::{arena::Arena, id::NodeId, node::Node};

pub mod error {

    #[derive(Debug)]
    pub enum AcreageError {
        NodeNotFound(String),
    }

    impl std::error::Error for AcreageError {}

    impl std::fmt::Display for AcreageError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AcreageError::NodeNotFound(msg) => write!(f, "Node not found: {}", msg),
            }
        }
    }
}

pub mod prelude {
    pub use crate::node::Compare;
}
