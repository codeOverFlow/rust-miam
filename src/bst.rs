use std::fmt::{Debug, Display, Formatter};

use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum BST<T: Ord + Display + Debug> {
    Node {
        data: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Empty,
}

impl<T: Ord + Display + Debug + Clone> PartialEq for BST<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            BST::Empty => {
                match other {
                    BST::Empty => true,
                    BST::Node { .. } => false,
                }
            }
            BST::Node { ref data, .. } => {
                match other {
                    BST::Empty => false,
                    BST::Node { data: ref data2, .. } => data == data2
                }
            }
        }
    }
}

impl<T: Ord + Display + Debug + Clone> Eq for BST<T> {}

impl<T: Ord + Display + Debug + Clone> PartialOrd for BST<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            BST::Empty => {
                match other {
                    BST::Empty => Some(Ordering::Equal),
                    BST::Node { .. } => Some(Ordering::Less),
                }
            }
            BST::Node { ref data, .. } => {
                match other {
                    BST::Empty => Some(Ordering::Greater),
                    BST::Node { data: ref data2, .. } => data.partial_cmp(data2)
                }
            }
        }
    }
}

impl<T: Ord + Display + Debug + Clone> Ord for BST<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Ord + Display + Debug + Clone> Display for BST<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BST::Node {
                ref data,
                ref left,
                ref right
            } => writeln!(f, "Node(data: {}, left: {}, right: {}", data, left, right),
            BST::Empty => writeln!(f, "Empty")
        }
    }
}

impl<T: Ord + Display + Debug + Clone> From<T> for BST<T> {
    fn from(value: T) -> Self {
        BST::Node {
            data: value,
            left: Box::new(BST::Empty),
            right: Box::new(BST::Empty),
        }
    }
}

impl<T: Ord + Display + Debug + Clone> BST<T> {
    pub fn new() -> BST<T> {
        BST::Empty
    }

    pub fn has_value(&self, value: T) -> bool {
        match self {
            BST::Node {
                ref data,
                ref left,
                ref right
            } => {
                match value.cmp(data) {
                    Ordering::Greater => right.has_value(value),
                    Ordering::Less => left.has_value(value),
                    Ordering::Equal => true
                }
            }
            BST::Empty => false
        }
    }

    pub fn get_value(&self, value: T) -> Option<&T> {
        match self {
            BST::Node {
                ref data,
                ref left,
                ref right
            } => {
                match value.cmp(data) {
                    Ordering::Greater => right.get_value(value),
                    Ordering::Less => left.get_value(value),
                    Ordering::Equal => Some(&data)
                }
            }
            BST::Empty => None
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            BST::Empty => true,
            _ => false
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            BST::Node {
                ref left,
                ref right, ..
            } => left.is_empty() && right.is_empty(),
            BST::Empty => false
        }
    }

    pub fn get_data(&self) -> Option<&T> {
        match self {
            BST::Empty => None,
            BST::Node { ref data, .. } => Some(data),
        }
    }

    fn get_data_mut(&mut self) -> Option<&mut T> {
        match self {
            BST::Empty => None,
            BST::Node { ref mut data, .. } => Some(data),
        }
    }

    pub fn get_left(&self) -> Option<&Box<BST<T>>> {
        match self {
            BST::Empty => None,
            BST::Node { ref left, .. } => Some(left),
        }
    }

    pub fn get_right(&self) -> Option<&Box<BST<T>>> {
        match self {
            BST::Empty => None,
            BST::Node { ref right, .. } => Some(right),
        }
    }

    fn get_right_mut(&mut self) -> Option<&mut Box<BST<T>>> {
        match self {
            BST::Empty => None,
            BST::Node { ref mut right, .. } => Some(right),
        }
    }

    fn get_min_node(&self) -> Option<&Box<BST<T>>> {
        match self {
            BST::Empty => None,
            BST::Node { ref left, .. } => {
                if left.is_leaf() {
                    Some(left)
                } else {
                    left.get_min_node()
                }
            }
        }
    }

    pub fn has_left(&self) -> bool {
        match self {
            BST::Node {
                ref left, ..
            } => !left.is_empty(),
            BST::Empty => false
        }
    }

    pub fn has_right(&self) -> bool {
        match self {
            BST::Node {
                ref right, ..
            } => !right.is_empty(),
            BST::Empty => false
        }
    }

    pub fn insert(&mut self, value: T) -> () {
        match self {
            BST::Node {
                ref data,
                ref mut left,
                ref mut right
            } => {
                match value.cmp(data) {
                    Ordering::Greater => right.insert(value),
                    Ordering::Less => left.insert(value),
                    _ => ()
                }
            }
            BST::Empty => *self = BST::from(value)
        }
    }

    pub fn delete(&mut self, value: T) -> () {
        match self {
            BST::Node {
                ref mut data,
                ref mut left,
                ref mut right
            } => {
                if value.lt(data) {
                    // LEFT
                    if !left.is_empty() {
                        if let Some(v) = left.get_data() {
                            match value.cmp(v) {
                                Ordering::Less => {
                                    if left.has_left() {
                                        match left.as_mut() {
                                            BST::Node { ref mut left, .. } => left.delete(value),
                                            _ => ()
                                        }
                                    }
                                }
                                Ordering::Greater => {
                                    if left.has_right() {
                                        match left.as_mut() {
                                            BST::Node { ref mut right, .. } => right.delete(value),
                                            _ => ()
                                        }
                                    }
                                }
                                Ordering::Equal => {
                                    if left.is_leaf() { // CASE 1
                                        *left = Box::new(BST::new())
                                    }
                                    else if left.has_left() && left.has_right() { // CASE 3
                                        let mut right_sub_tree = left.get_right_mut().unwrap().clone();
                                        let min_node = right_sub_tree.get_min_node().unwrap().clone();
                                        *left.get_data_mut().unwrap() = min_node.get_data().unwrap().clone();
                                        if let Some(min_data) = min_node.get_data() {
                                            right_sub_tree.delete(min_data.clone())
                                        }
                                    }
                                    else if left.has_left() { // CASE 2
                                        if let Some(child_left) = left.get_left() {
                                            *left = child_left.clone();
                                        }
                                    }
                                    else if left.has_right() { // CASE 2
                                        if let Some(child_right) = left.get_right() {
                                            *left = child_right.clone();
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if value.gt(data) {
                    // RIGHT
                    if !right.is_empty() {
                        if let Some(v) = right.get_data() {
                            match value.cmp(v) {
                                Ordering::Less => {
                                    if right.has_left() {
                                        match right.as_mut() {
                                            BST::Node { ref mut left, .. } => left.delete(value),
                                            _ => ()
                                        }
                                    }
                                }
                                Ordering::Greater => {
                                    if right.has_right() {
                                        match right.as_mut() {
                                            BST::Node { ref mut right, .. } => right.delete(value),
                                            _ => ()
                                        }
                                    }
                                }
                                Ordering::Equal => {
                                    if right.is_leaf() { // CASE 1
                                        *right = Box::new(BST::new())
                                    }
                                    else if right.has_left() && right.has_right() { // CASE 3
                                        let mut right_sub_tree = right.get_right_mut().unwrap().clone();
                                        let min_node = right_sub_tree.get_min_node().unwrap().clone();
                                        *right.get_data_mut().unwrap() = min_node.get_data().unwrap().clone();
                                        if let Some(min_data) = min_node.get_data() {
                                            right_sub_tree.delete(min_data.clone())
                                        }
                                    }
                                    else if right.has_left() { // CASE 2
                                        if let Some(child_left) = right.get_left() {
                                            *right = child_left.clone();
                                        }
                                    }
                                    else if right.has_right() { // CASE 2
                                        if let Some(child_right) = right.get_right() {
                                            *right = child_right.clone();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => ()
        }
    }

    pub fn pre_order_print(&self) -> () {
        match self {
            BST::Empty => (),
            BST::Node {
                ref data,
                ref left,
                ref right
            } => {
                println!("{}", data);
                left.pre_order_print();
                right.pre_order_print();
            }
        }
    }

    pub fn in_order_print(&self) -> () {
        match self {
            BST::Empty => (),
            BST::Node {
                ref data,
                ref left,
                ref right
            } => {
                left.in_order_print();
                println!("{}", data);
                right.in_order_print();
            }
        }
    }

    pub fn post_order_print(&self) -> () {
        match self {
            BST::Empty => (),
            BST::Node {
                ref data,
                ref left,
                ref right
            } => {
                left.post_order_print();
                right.post_order_print();
                println!("{}", data);
            }
        }
    }
}