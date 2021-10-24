use std::fmt::Display;
use std::boxed::Box;

type ChildNode<T> = Option<Box<Node<T>>>;

#[derive(Default, Debug)]
pub struct Node<T: PartialOrd + Display + Clone> {
    value: Option<T>,
    left: ChildNode<T>,
    right: ChildNode<T>, 
}

impl<T: PartialOrd + Display + Clone> Node<T> {
    fn new(val: T) -> Self {
        Node {
            value: Some(val),
            left: None,
            right: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct Tree<T: PartialOrd + Display + Clone> {
    root: Node<T>,
    pub length: usize,
}

impl<T: PartialOrd + Display + Clone> Tree<T> {

    /// Create new BinarySearchTree with given value as a root Node
    pub fn new(val: T) -> Self {
        Tree {
            root: Node::new(val),
            length: 1,
        }
    }

    pub fn get_left_child(&self) -> &Node<T> {
        self.root.left.as_ref().unwrap()
    }

    pub fn get_right_child(&self) -> &Node<T> {
        self.root.right.as_ref().unwrap()
    }

    pub fn try_get_left_child(&self) -> Option<&Node<T>> {
        if let Some(n) = self.root.left.as_ref() {
            Some(n)
        } else {
            None
        }
    }

    pub fn try_get_right_child(&self) -> Option<&Node<T>> {
        if let Some(n) = self.root.right.as_ref() {
            Some(n)
        } else {
            None
        }
    }

    pub fn is_leaf(&self, n: &Node<T>) -> bool {
       n.left.is_none() && n.right.is_none() 
    }

    pub fn contains(&self, val: T) -> bool {
        self._contains(&self.root, val)
    }

    fn _contains(&self, n: &Node<T>, val: T) -> bool {
        if let Some(v) = n.value.as_ref() {
            if v == &val {
                true
            } else if v > &val {
                return self._contains(n.left.as_ref().unwrap(), val)
            } else {
                return self._contains(n.right.as_ref().unwrap(), val)
            }
        } else {
            false
        }
    }

    /// Add node to the BinarySearchTree
    pub fn add(&mut self, val: T) {
        if self.root.value.is_none() {
            self.root = Node::new(val);
        } else {
            Tree::_add(&mut self.root, val);
        }
        self.length += 1;
    }

    fn _add(n: &mut Node<T>, val: T) {
        if let Some(v) = &mut n.value {
            if &val == v {
                return;
            } else if &val < v {
                if n.left.is_none() {
                    n.left = Some(Box::new(Node::new(val)));
                } else {
                    Tree::_add(n.left.as_mut().unwrap(), val);
                }
            } else {
               if n.right.is_none() {
                    n.right = Some(Box::new(Node::new(val)));
               } else {
                    Tree::_add(n.right.as_mut().unwrap(), val);
                }
            }
        }
    }

    /// removes node from a tree
    pub fn remove(&mut self, val: T) {
        Tree::_remove(&mut self.root, val);
    }

    fn _remove(n: &mut Node<T>, val: T) {
        if let Some(v) = n.value.as_mut() {
            if v == &val {
            }
        }
    }

    /// prints the value of a given Node
    pub fn visit(n: &Node<T>) {
        println!("{}", n.value.as_ref().unwrap());
    }

    /// get the node with max value
    pub fn max(&self) -> T {
        self._max(&self.root)
    }

    fn _max(&self, n: &Node<T>) -> T {
        if n.right.is_none() {
            n.value.as_ref().unwrap().clone()
        } else {
            self._max(n.right.as_ref().unwrap())
        }
    }

    /// get the node with min value
    pub fn min(&self) -> T {
        self._min(&self.root)
    }

    fn _min(&self, n: &Node<T>) -> T {
        if n.left.is_none() {
            n.value.as_ref().unwrap().clone()
        } else {
            self._min(n.left.as_ref().unwrap())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_constructor() {
        let tree = Tree::new(-5);

        assert_eq!(tree.length, 1);
        assert_eq!(tree.root.value.unwrap(), -5);
    }

    #[test]
    fn test_add_node() {
        let mut tree = Tree::new(5);
        tree.add(7);

        assert_eq!(tree.length, 2);
        assert_eq!(tree.root.right.unwrap().as_ref().value.unwrap(), 7);
    }

    #[test]
    fn test_contains() {
        let mut tree = Tree::new('g');
        tree.add('b');

        assert!(tree.contains('g'));
        assert!(tree.contains('b'));
    }

    #[test]
    fn test_remove_node() {
        let mut tree = Tree::new(1);
        tree.add(2);
        tree.add(3);
        tree.remove(3);

        assert!(!tree.contains(3));
    }

    #[test]
    fn test_get_max_val() {
        let mut tree = Tree::new(1);
        tree.add(2);
        tree.add(3);

        assert_eq!(tree.max(), 3);
    }
    #[test]
    fn test_get_min_val() {
        let mut tree = Tree::new(4);
        tree.add(2);
        tree.add(3);

        assert_eq!(tree.min(), 2);
    }
}
