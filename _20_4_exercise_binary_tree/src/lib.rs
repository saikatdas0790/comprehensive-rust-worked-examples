/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

// Implement `new`, `insert`, `len`, and `has`.
impl<T: Ord> BinaryTree<T> {
    /// Create a new, empty binary tree.
    pub fn new() -> Self {
        Self {
            root: Subtree(None),
        }
    }

    /// Insert a value into the binary tree.
    pub fn insert(&mut self, value: T) {
        fn insert_into<T: Ord>(subtree: &mut Subtree<T>, value: T) {
            match &mut subtree.0 {
                None => {
                    *subtree = Subtree(Some(Box::new(Node {
                        value,
                        left: Subtree(None),
                        right: Subtree(None),
                    })));
                }
                Some(node) if value < node.value => {
                    insert_into(&mut node.left, value);
                }
                Some(node) if value > node.value => {
                    insert_into(&mut node.right, value);
                }
                _ => {}
            }
        }
        insert_into(&mut self.root, value);
    }

    /// Return the number of elements in the binary tree.
    pub fn len(&self) -> usize {
        fn len_of<T: Ord>(subtree: &Subtree<T>) -> usize {
            match &subtree.0 {
                None => 0,
                Some(node) => 1 + len_of(&node.left) + len_of(&node.right),
            }
        }
        len_of(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return whether the binary tree contains a value.
    pub fn has(&self, value: &T) -> bool {
        fn has_in<T: Ord>(subtree: &Subtree<T>, value: &T) -> bool {
            match &subtree.0 {
                Some(node) if value < &node.value => has_in(&node.left, value),
                Some(node) if value > &node.value => has_in(&node.right, value),
                Some(node) => value == &node.value,
                None => false,
            }
        }
        has_in(&self.root, value)
    }
}

impl<T: Ord> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
