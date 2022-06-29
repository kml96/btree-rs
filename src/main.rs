/// Binary Tree data structure
/// It can be Empty or
/// Non empty in which case it points to a Node
enum BTree<T: Ord> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

/// Node of Binary Tree
/// It contains the value of given type
/// Since our data structure is binary tree,
/// It consists of left and right node
struct Node<T: Ord> {
    item: T,
    left: BTree<T>,
    right: BTree<T>,
}

// helper methods for Binary Tree
impl<T: Ord> BTree<T> {
    /// returns new empty Binary Tree
    fn new() -> Self {
        Self::Empty
    }

    /// adds node to the Binary Tree
    fn add(&mut self, item: T) {
        match self {
            BTree::Empty => {
                *self = BTree::NonEmpty(Box::new(Node {
                    item,
                    left: BTree::Empty,
                    right: BTree::Empty,
                }));
            }
            BTree::NonEmpty(ref mut node) => {
                if item <= node.item {
                    node.left.add(item);
                } else {
                    node.right.add(item);
                }
            }
        }
    }
    /// helper function which converts Binary Tree enum into Iter .ie IterBT type
    fn iter(&self) -> IterBT<T> {
        let mut iter = IterBT {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}
// implement Binary Tree as Iterable type
impl<'a, T: Ord> IntoIterator for &'a BTree<T> {
    type Item = &'a T;
    type IntoIter = IterBT<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over Binary Tree
struct IterBT<'a, T: Ord> {
    unvisited: Vec<&'a Node<T>>,
}

// helper methods for IterBT
impl<'a, T: Ord> IterBT<'a, T> {
    /// push node on the left edge of the tree
    fn push_left_edge(&mut self, mut tree: &'a BTree<T>) {
        while let BTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

// Implement IterBT as Iterator
impl<'a, T: Ord> Iterator for IterBT<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.item)
    }
}
fn main() {
    let mut bt = BTree::new();
    bt.add("ram");
    bt.add("shyam");
    bt.add("gopal");
    bt.add("hari");
    bt.add("nabin");

    for item in &bt {
        println!("item: {}", item);
    }
}
