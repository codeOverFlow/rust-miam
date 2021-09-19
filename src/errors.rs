use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub enum BSTError {
    UnfoundValue(&'static str),
    EmptyTree,
}

impl Display for BSTError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BSTError::UnfoundValue(reason) => writeln!(f, "{}", reason),
            BSTError::EmptyTree => writeln!(f, "BST Tree is empty"),
        }
    }
}

impl Error for BSTError {}