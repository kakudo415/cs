pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(value, None, None)));
        } else {
            self.root.as_mut().unwrap().insert(value);
        }
    }

    pub fn search(&self, value: T) -> bool {
        if self.root.is_none() {
            return false;
        }

        self.root.as_ref().unwrap().search(value)
    }
}

impl<T: Ord> Node<T> {
    fn new(value: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Self {
        Node { value, left, right }
    }

    fn insert(&mut self, value: T) {
        if value <= self.value {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::new(value, None, None)));
            } else {
                self.left.as_mut().unwrap().insert(value);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(value, None, None)));
            } else {
                self.right.as_mut().unwrap().insert(value);
            }
        }
    }

    fn search(&self, value: T) -> bool {
        if value == self.value {
            return true;
        }

        if value < self.value {
            if self.left.is_none() {
                return false;
            } else {
                return self.left.as_ref().unwrap().search(value);
            }
        } else {
            if self.right.is_none() {
                return false;
            } else {
                return self.right.as_ref().unwrap().search(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);
        assert_eq!(bst.search(6), true);
        assert_eq!(bst.search(8), true);
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(9), false);
    }
}
