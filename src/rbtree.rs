use std::cell::RefCell;
use std::rc::Rc;
use std::mem::replace;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type TreeNode<T> = Rc<RefCell<Node<T>>>;
type Tree<T> = Option<TreeNode<T>>;

#[derive(Clone)]
pub struct Node<T: Ord+Display+Debug> {
    color: NodeColor,
    key: T,
    parent: Tree<T>,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T> Node<T>
where 
    T: Debug+Ord+Display+Copy
{
    pub fn new(key: T) -> Tree<T> {
        Some(Rc::new(RefCell::new(Node {
            color: NodeColor::Red,
            key: key,
            parent: None,
            left: None,
            right: None,
        })))
    }
}

impl<T> fmt::Debug for Node<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
         .field("color", &self.color)
         .field("key", &self.key)
         .field("right", &self.right)
         .field("left", &self.left)
         .finish()
    }
}

enum Direction {
    Left,
    Right
}

#[derive(Clone, Debug)]
pub struct RBTree<T: Ord+Display+Debug+Copy> {
    root: Tree<T>,
    count: u32,
}

impl<T> RBTree<T>
where T: Ord+Display+Debug+Clone+Copy
{
    pub fn new() -> Self {
        RBTree {
            root: None,
            count: 0,
        }
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    // 6- check if tree is empty
    pub fn is_empty(&self) -> bool {
        if self.root.is_none() {
            return true;
        } else {
            return false;
        }
    }

    // 1- insert a node to the red-black tree
    pub fn insert(&mut self, key: T) {
        // check if key already in tree
        if self.search(key).is_none() {
            // need to pass Tree<T> along with RBTree<T> or else we can't call associated functions
            let root = replace(&mut self.root, None);
            let updated_tree = self.insert_node(root, key);
            self.root = self.insert_fix(updated_tree.1);
        } else {
            println!("Key already in tree");
        }
    }

    fn insert_node(&mut self, tree: Tree<T>, key: T) -> (Tree<T>,TreeNode<T>) {
        match tree {
            Some(tree_node) => {
                let sub_tree: TreeNode<T>;
                let node_clone = tree_node.borrow().clone();
                if key < node_clone.key {
                    let res = self.insert_node(node_clone.left, key);
                    let res_tree = res.0;
                    sub_tree = res.1;
                    res_tree.as_ref().unwrap().borrow_mut().parent = Some(tree_node.clone());
                    tree_node.borrow_mut().left = res_tree;
                } else {
                    let res = self.insert_node(node_clone.right, key);
                    let res_tree = res.0;
                    sub_tree = res.1;
                    res_tree.as_ref().unwrap().borrow_mut().parent = Some(tree_node.clone());
                    tree_node.borrow_mut().right = res_tree;
                };
                (Some(tree_node),sub_tree)
            },
            None => {
                self.count += 1;
                let added_node = Node::<T>::new(key);
                (added_node.clone(),added_node.unwrap())
            }
        }
    }

    fn insert_fix(&mut self, tree_node: TreeNode<T>) -> Tree<T> {
        let mut is_root = tree_node.borrow().parent.is_none(); // if parent is none, then we have root node
        let root = if is_root {
            Some(tree_node)
        } else {
            // we don't have root node but we need to return it
            // fix our subtree and then
            // iteratively recurse up until root because we want to return it
            let mut node = tree_node.clone();
            let mut parent_clone = tree_node.borrow().parent.as_ref().unwrap().borrow().clone();
            let mut parent_color = parent_clone.color;
            
            while !is_root && parent_color == NodeColor::Red {
                // these are the conditions under which we want to fix the tree
                // find uncle node
                let node_clone = node.borrow().clone();
                let uncle_return = match node_clone.parent {
                    Some(parent) => {
                        let parent = parent.borrow().clone();
                        match parent.parent {
                            Some(grandparent) => {
                                let grandparent = grandparent.borrow().clone();
                                if grandparent.key < parent.key {
                                    Some((grandparent.left.clone(), Direction::Left))
                                } else {
                                    Some((grandparent.right.clone(), Direction::Right))
                                }
                            },
                            None => {None}
                        }
                    },
                    None => { None }
                };
                
                match uncle_return {
                    Some(uncle) => {
                        let uncle_node = uncle.0;
                        let side = uncle.1;

                        match side {
                            Direction::Right => {
                                let mut parent = node.borrow().parent.as_ref().unwrap().clone();
                                // uncle is on right side
                                if uncle_node.is_some() && uncle_node.as_ref().unwrap().borrow().color == NodeColor::Red {
                                    // flip parent and uncle to black
                                    parent.borrow_mut().color = NodeColor::Black;
                                    uncle_node.unwrap().borrow_mut().color = NodeColor::Black;
                                    // flip grandparent to red
                                    parent.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    // iteratively recurse up tree to check for any other red-black violations
                                    node = parent.borrow().clone().parent.clone().unwrap();
                                } else {
                                    // uncle is black (None counts as black too)
                                    // need to know whether current node is either on left or right side
                                    if parent.borrow().clone().key < node.borrow().clone().key {
                                        // node is on right side 
                                        // rotate node left so that node becomes parent and parent becomes left child of node
                                        let parent_tmp = node.borrow().parent.as_ref().unwrap().clone();
                                        node = parent_tmp;
                                        self.rotate_left(node.clone());
                                        parent = node.borrow().parent.as_ref().unwrap().clone();
                                    } 

                                    parent.borrow_mut().color = NodeColor::Black;
                                    parent.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    let grandparent = node.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                                    // rotate parent right so that grandparent becomes right child
                                    self.rotate_right(grandparent);
                                }
                            },
                            Direction::Left => {
                                let mut parent = node.borrow().parent.as_ref().unwrap().clone();
                                // uncle is on left side
                                if uncle_node.is_some() && uncle_node.as_ref().unwrap().borrow().color == NodeColor::Red {
                                    // flip parent and uncle to black
                                    parent.borrow_mut().color = NodeColor::Black;
                                    uncle_node.unwrap().borrow_mut().color = NodeColor::Black;
                                    // flip grandparent to red
                                    parent.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    // iteratively recurse up tree to check for any other red-black violations
                                    node = parent.borrow().clone().parent.clone().unwrap();
                                } else {
                                    // uncle is black
                                    // need to know whether current node is either left or right child of parent
                                    if parent.borrow().clone().key > node.borrow().clone().key {
                                        // node is on left side
                                        // rotate node right so that node becomes parent and parent becomes right child of node
                                        let parent_tmp = node.borrow().parent.as_ref().unwrap().clone();
                                        node = parent_tmp;
                                        self.rotate_right(node.clone());
                                        parent = node.borrow().parent.as_ref().unwrap().clone();
                                    }
                                    parent.borrow_mut().color = NodeColor::Black;
                                    parent.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    let grandparent = node.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                                    self.rotate_left(grandparent);
                                }
                            }
                        }
                    },
                    None => {
                        break;
                    }
                }
                is_root = node.borrow().parent.is_none();
                if !is_root {
                    parent_clone = node.borrow().parent.as_ref().unwrap().borrow().clone();
                    parent_color = parent_clone.color;
                }
            } 

            // done fixing the tree, so recurse back up the tree and return root
            while node.borrow().parent.is_some() {
                let p = node.borrow().parent.as_ref().unwrap().clone();
                node = p;
            }
            Some(node)
        };
        root.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
        root
    }

    fn rotate_left(&self, tree_node: TreeNode<T>) {
        let cur_parent = tree_node;
        let right_child = cur_parent.borrow().right.clone();

        // take the left child of right child and make it the right child of the current parent
        cur_parent.borrow_mut().right = match right_child {
            Some(ref right_child) => {right_child.borrow().left.clone()},
            None => {None}
        };

        if right_child.is_some() {
            // make right child's parent the current grandparent
            right_child.as_ref().unwrap().borrow_mut().parent = cur_parent.borrow().parent.clone();
            if right_child.as_ref().unwrap().borrow().left.is_some() {
                // make right_child's left child's parent the current parent
                let l = right_child.as_ref().unwrap().borrow().left.clone();
                l.unwrap().borrow_mut().parent = Some(cur_parent.clone());
            }
        }

        match cur_parent.borrow().clone().parent {
            Some(grandparent) => {
                if grandparent.borrow().clone().key < cur_parent.borrow().clone().key {
                    grandparent.borrow_mut().right = right_child.clone();
                } else {
                    grandparent.borrow_mut().left = right_child.clone();
                }
            },
            None => {
                // grandparent is None, so make the right_child's parent None
                right_child.as_ref().unwrap().borrow_mut().parent = None;
            },
        }
        // make right_child's left child equal to the parent
        right_child.as_ref().unwrap().borrow_mut().left = Some(cur_parent.clone());
        // make parent's parent equal to right_child
        cur_parent.borrow_mut().parent = right_child.clone();
    }

    fn rotate_right(&self, tree_node: TreeNode<T>) {
        let cur_parent = tree_node;
        let left_child = cur_parent.borrow().left.clone();

        // take the right child of left_child and make it the left child of current parent
        cur_parent.borrow_mut().left = match left_child {
            Some(ref left_child) => {left_child.borrow().right.clone()},
            None => {None}
        };

        if left_child.is_some() {
            // make left child's parent the current grandparent
            left_child.as_ref().unwrap().borrow_mut().parent = cur_parent.borrow().parent.clone();
            if left_child.as_ref().unwrap().borrow().right.is_some() {
                // make left_child's right child's parent the current parent
                let r = left_child.as_ref().unwrap().borrow().right.clone();
                r.unwrap().borrow_mut().parent = Some(cur_parent.clone());
            }
        }

        match cur_parent.borrow().clone().parent {
            Some(grandparent) => {
                if grandparent.borrow().clone().key < cur_parent.borrow().clone().key {
                    grandparent.borrow_mut().right = left_child.clone();
                } else {
                    grandparent.borrow_mut().left = left_child.clone();
                }
            },
            None => {
                // grandparent is None, so make the left_child's parent None
                left_child.as_ref().unwrap().borrow_mut().parent = None;
            },
        }
        // make left_child's right child equal to the parent
        left_child.as_ref().unwrap().borrow_mut().right = Some(cur_parent.clone());
        // make parent's parent equal to left_child
        cur_parent.borrow_mut().parent = left_child.clone();
    }

    pub fn search(&self, key: T) -> Tree<T> {
        let dummy = Node::<T>::new(key).unwrap().borrow().clone();
        self.search_node(&self.root, &dummy)
    }

    fn search_node(&self, tree_node: &Tree<T>, node: &Node<T>) -> Tree<T> {
        match tree_node {
            Some(sub_tree) => {
                let sub_tree_clone = sub_tree.borrow().clone();
                if sub_tree_clone.key == node.key {
                    Some(sub_tree.clone())
                } else {
                    if sub_tree_clone.key > node.key {
                        self.search_node(&sub_tree_clone.left, node)
                    } else {
                        self.search_node(&sub_tree_clone.right, node)
                    }
                }
            },
            None => {None}
        }
    }

    // 2- delete a node from the red-black tree
    pub fn delete(&mut self, key: T) {
        let z = self.search(key);
        if z.is_none() {
            println!("Key not found");
            return;
        }
        // key exists
        let u = z; // node to be deleted
        let p = u.as_ref().unwrap().borrow().parent.clone();
        let v = u.as_ref().unwrap().borrow().left.clone(); 
        let w = u.as_ref().unwrap().borrow().right.clone();

        let mut side = Direction::Left; // set default value to left

        if p.is_some() {
            side = if p.as_ref().unwrap().borrow().clone().key > u.as_ref().unwrap().borrow().clone().key {
                Direction::Right
            } else {
                Direction::Left
            };
        }

        let mut u_original_color = u.as_ref().unwrap().borrow().color.clone();
        let x: Tree<T>;

        if v.is_none() {
            // left node of u is none
            x = w.clone();
            self.transplant(u.clone(),w.clone());
        } else if w.is_none() {
            // right node of u is none
            x = v.clone();
            self.transplant(u.clone(), v.clone());
        } else {
            // both left and right nodes exist
            // find minimum in right branch to replace u
            let y = self.find_min(w.clone());
            // y will always be Some since we only call find_min where left and right both exist
            // if w has no children then find_min will simply return w
            // we can safely unwrap
            // x is right subtree of y
            u_original_color = y.as_ref().unwrap().borrow().color.clone();
            x = y.as_ref().unwrap().borrow().clone().right;
            if y.as_ref().unwrap().borrow().clone().parent.as_ref().unwrap().borrow().clone().key == u.as_ref().unwrap().borrow().clone().key {
                if x.is_some() {
                    x.as_ref().unwrap().borrow_mut().parent = y.clone();
                }
            } else {
                self.transplant(y.clone(), y.as_ref().unwrap().borrow().right.clone());
                y.as_ref().unwrap().borrow_mut().right = u.as_ref().unwrap().borrow().right.clone();
                y.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().parent = y.clone();
            }
            self.transplant(u.clone(), y.clone());
            y.as_ref().unwrap().borrow_mut().left = v.clone();
            v.as_ref().unwrap().borrow_mut().parent = y.clone();
            y.as_ref().unwrap().borrow_mut().color = u.as_ref().unwrap().borrow().color.clone();
        }
        if u_original_color == NodeColor::Black {
            self.delete_fix(x.clone(), p.clone(), side);
        }
        self.count -= 1;
    }

    fn delete_fix(&mut self, x: Tree<T>, p: Tree<T>, side: Direction) {
        // x color is true if black and false if red
        let mut x_color = if x.is_some() {
            x.as_ref().unwrap().borrow().clone().color == NodeColor::Black
        } else {
            // Node is none so it is black
            true
        };
        let mut cur_p = p;
        let mut cur_x = x;
        let mut is_root = cur_p.is_none();
        while !is_root && x_color {
            match side {
                Direction::Right => {
                    // sibling on the right side of p
                    // cur_p exists or else we wouldnt be in this while loop
                    let mut s = cur_p.as_ref().unwrap().borrow().right.clone();
                    if s.is_some() {
                        if s.as_ref().unwrap().borrow().clone().color == NodeColor::Red {
                            // DB's sibling is red
                            // swap color of p with s
                            // rotate parent node left
                            s.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            cur_p.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                            self.rotate_left(cur_p.as_ref().unwrap().clone());
                            s = cur_p.as_ref().unwrap().borrow().right.clone();
                        }
                        let s_left = s.as_ref().unwrap().borrow().clone().left.clone();
                        let s_right = s.as_ref().unwrap().borrow().clone().right.clone();

                        let s_left_color = if s_left.is_some() {
                            s_left.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                        } else {
                            true
                        };

                        let s_right_color = if s_right.is_some() {
                            s_right.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                        } else {
                            true
                        };

                        if s_left_color && s_right_color {
                            s.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                            cur_x = cur_p.clone();
                            let g = cur_p.as_ref().unwrap().borrow().clone().parent.clone();
                            cur_p = g.clone();
                            x_color = if cur_x.is_some() {
                                cur_x.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                            } else {
                                true
                            };
                        } else {
                            if s_right.is_some() && s_right.as_ref().unwrap().borrow().clone().color == NodeColor::Black {
                                if s_left.is_some() {
                                    s_left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                                    s.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    self.rotate_right(s.unwrap());
                                    s = cur_p.as_ref().unwrap().borrow().right.clone();
                                }
                            }
                            s.as_ref().unwrap().borrow_mut().color = cur_p.as_ref().unwrap().borrow().color.clone();
                            cur_p.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            if s_right.is_some() {
                                s_right.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            }
                            self.rotate_left(cur_p.as_ref().unwrap().clone());
                            is_root = true;
                        }
                    }
                },
                Direction::Left => {
                    // siblings are on the left side of p
                    let mut s = cur_p.as_ref().unwrap().borrow().left.clone();
                    if s.is_some() {
                        if s.as_ref().unwrap().borrow().clone().color == NodeColor::Red {
                            // DB's sibling is red
                            // swap color of p with s
                            // rotate parent node right
                            s.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            cur_p.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                            self.rotate_right(cur_p.as_ref().unwrap().clone());
                            s = cur_p.as_ref().unwrap().borrow().left.clone();
                        }
                        let s_left = s.as_ref().unwrap().borrow().clone().left.clone();
                        let s_right = s.as_ref().unwrap().borrow().clone().right.clone();

                        let s_left_color = if s_left.is_some() {
                            s_left.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                        } else {
                            true
                        };

                        let s_right_color = if s_right.is_some() {
                            s_right.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                        } else {
                            true
                        };

                        if s_left_color && s_right_color {
                            s.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                            cur_x = cur_p.clone();
                            let g = cur_p.as_ref().unwrap().borrow().clone().parent.clone();
                            cur_p = g.clone();
                            x_color = if cur_x.is_some() {
                                cur_x.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                            } else {
                                true
                            };
                        } else {
                            if s_right.is_some() && s_right.as_ref().unwrap().borrow().clone().color == NodeColor::Black {
                                if s_left.is_some() {
                                    s_left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                                    s.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    self.rotate_left(s.unwrap());
                                    s = cur_p.as_ref().unwrap().borrow().left.clone();
                                }
                            }
                            s.as_ref().unwrap().borrow_mut().color = cur_p.as_ref().unwrap().borrow().color.clone();
                            cur_p.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            if s_left.is_some() {
                                s_left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            }
                            self.rotate_right(cur_p.as_ref().unwrap().clone());
                            is_root = true;
                        }
                    }
                }
            }
        }
        if cur_x.is_some() {
            cur_x.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
        }
    }

    fn transplant(&mut self, z: Tree<T>, v: Tree<T>) {
        // transplant is responsible for deleting u and replacing it with v
        let u = z.unwrap();
        let u_p = u.borrow().parent.clone();
        if u_p.is_none() {
            // deleting root node
            self.root = v.clone();
        } else {
            if u_p.as_ref().unwrap().borrow().clone().key > u.borrow().clone().key {
                // z is on the left of parent
                u_p.as_ref().unwrap().borrow_mut().left = v.clone();
            } else {
                // z is on the right of parent
                u_p.as_ref().unwrap().borrow_mut().right = v.clone();
            }
        }
        if v.is_some() {
            // replacement node exists
            v.as_ref().unwrap().borrow_mut().parent = u_p.clone();
        }
    }

    fn find_min(&self, tree: Tree<T>) -> Tree<T> {
        match tree {
            Some(sub_tree) => {
                let mut left = Some(sub_tree.clone());
                while left.as_ref().unwrap().borrow().left.clone().is_some() {
                    left = left.unwrap().borrow().left.clone();
                }
                left
            },
            None => {
                tree
            }
        }
    }

    fn find_max(&self, tree: Tree<T>) -> Tree<T> {
        match tree {
            Some(sub_tree) => {
                let mut right = Some(sub_tree.clone());
                while right.as_ref().unwrap().borrow().right.clone().is_some() {
                    right = right.unwrap().borrow().right.clone();
                }
                right
            },
            None => {
                tree
            }
        }
    }

    // 3- count the number of leaves in a tree
    pub fn leaves(&self) -> u32 {
        if self.root.is_none() {
            return 0;
        }
        let root = self.root.as_ref().unwrap().clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        stack.push(Some(root));

        let mut count = 0;
        while !stack.is_empty() {
            let node = stack.pop();
            let mut node_left = None;
            let mut node_right = None;

            if node.is_some() {
                node_left = node.as_ref().unwrap().as_ref().unwrap().borrow().clone().left.clone();
                node_right = node.as_ref().unwrap().as_ref().unwrap().borrow().clone().right.clone();
            }

            if node_left.is_some() {
                stack.push(node_left.clone());
            }

            if node_right.is_some() {
                stack.push(node_right.clone());
            }

            if node_left.is_none() && node_right.is_none() {
                count += 1;
            }
        }
        count
    }

    // 4- return the height of a tree
    pub fn height(&self) -> u32 {
        if self.root.is_none() {
            return 0;
        }
        let root = self.root.as_ref().unwrap().clone();
        let mut queue: VecDeque<Tree<T>> = VecDeque::new();
        queue.push_back(Some(root));

        let mut height = 0;
        // find height by breadth first search traversal
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let node = queue.pop_front().unwrap().unwrap();
                for child in [node.borrow().left.clone(), node.borrow().right.clone()] {
                    if child.is_some() {
                        queue.push_back(child);
                    }
                }
            }
            height += 1;
        }
        height-1
    }

    // 5- print in-order traversal of tree
    pub fn print_inorder(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        }
        let mut root = self.root.clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        while !stack.is_empty() || !root.is_none() {
            if root.is_some() {
                stack.push(root.clone());
                let p = root.as_ref().unwrap().borrow().left.clone();
                root = p.clone();
            } else {
                let pop = stack.pop().unwrap();
                print!(" {} ", pop.as_ref().unwrap().borrow().key.clone());
                root = pop.as_ref().unwrap().borrow().right.clone();
            }
        }
        println!("\n");
    }

    pub fn print_preorder(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        }
        let mut root = self.root.clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        stack.push(root);
        let mut cur: Tree<T>;
        while !stack.is_empty() {
            cur = stack.pop().unwrap();
            root = cur.clone();
            print!(" {} ", root.as_ref().unwrap().borrow().key.clone());
            let root_right = root.as_ref().unwrap().borrow().right.clone();
            let root_left = root.as_ref().unwrap().borrow().left.clone();
            if root_right.is_some() {
                stack.push(root_right.clone());
            }
            if root_left.is_some() {
                stack.push(root_left.clone());
            }
        }
        println!("\n");
    }

    pub fn print_levelorder(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        };
        let root = self.root.as_ref().unwrap().clone();
        let mut queue: VecDeque<Tree<T>> = VecDeque::new();
        queue.push_back(Some(root));
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let node = queue.pop_front().unwrap().unwrap();
                print!(" {} ", node.borrow().key.clone());
                for child in [node.borrow().left.clone(), node.borrow().right.clone()] {
                    if child.is_some() {
                        queue.push_back(child);
                    }
                }
            }
        }
        println!("\n");
    }

    pub fn min(&self) -> Tree<T> {
        self.find_min(self.root.clone())
    }

    pub fn max(&self) -> Tree<T> {
        self.find_max(self.root.clone())
    }
}

impl<T> fmt::Display for RBTree<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RBTree")
         .field("root", &self.root)
         .field("length", &self.count)
         .finish()
    }
}


#[test]
pub fn create_empty_rbtree() {
    // type T must be specified if theres no other node insertions
    let rbtree: RBTree<u32> = RBTree::new();
    assert!(rbtree.root.is_none());
}

#[test]
pub fn insert_into_rbtree_1() {
    let mut x = RBTree::new();
    assert_eq!(0,x.count);
    x.insert(3);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 3);
    assert_eq!(x.count,1);
    x.insert(2);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key, 2);
    assert_eq!(x.count, 2);
}

#[test]
pub fn insert_into_rbtree_2() {
    // left, straight line case
    let mut x = RBTree::new();
    assert_eq!(0,x.count);
    x.insert(3);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 3);
    assert_eq!(x.count,1);
    x.insert(2);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key, 2);
    assert_eq!(x.count, 2);
    x.insert(1);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,2);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,1);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key,3);
    assert_eq!(x.count, 3);
}

#[test]
pub fn insert_into_rbtree_3() {
    // left, elbow case
    let mut x = RBTree::new();
    assert_eq!(0,x.count);
    x.insert(4);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 4);
    assert_eq!(x.count,1);
    x.insert(2);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key, 2);
    assert_eq!(x.count, 2);
    x.insert(3);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,3);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,2);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key,4);
    assert_eq!(x.count, 3);
}

#[test]
pub fn insert_into_rbtree_4() {
    // right, straight line case
    let mut x = RBTree::new();
    assert_eq!(0,x.count);
    x.insert(4);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 4);
    assert_eq!(x.count,1);
    x.insert(5);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key, 5);
    assert_eq!(x.count, 2);
    x.insert(6);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,5);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,4);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key,6);
    assert_eq!(x.count, 3);
}

#[test]
pub fn insert_into_rbtree_5() {
    // right, elbow case
    let mut x = RBTree::new();
    assert_eq!(0,x.count);
    x.insert(4);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 4);
    assert_eq!(x.count,1);
    x.insert(6);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key, 6);
    assert_eq!(x.count, 2);
    x.insert(5);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,5);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,4);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key,6);
    assert_eq!(x.count, 3);
}

#[test]
pub fn is_empty_1() {
    let x: RBTree<u32> = RBTree::new();
    assert!(x.is_empty());
}

#[test]
pub fn search_1() {
    let mut x = RBTree::new();
    x.insert(9);
    x.insert(8);
    x.insert(12);
    x.insert(3);

    let y = x.search(8);
    assert_eq!(8,y.as_ref().unwrap().borrow().clone().key);
    let z = x.search(81);
    assert!(z.is_none());
}

#[test]
pub fn test_delete_1() {
    let mut x = RBTree::new();
    x.insert(12);
    x.insert(8);
    x.insert(15);
    x.delete(12);

    assert_eq!(x.root.as_ref().unwrap().borrow().key,15);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,8);
}

#[test]
pub fn insert_test_1() {
    let mut x = RBTree::new();
    x.insert(15);
    x.insert(11);
    x.insert(19);
    x.insert(8);
    x.insert(13);
    x.insert(16);
    x.insert(23);
    x.insert(12);
    x.insert(14);
    //x.delete(13);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,15);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,19);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,11);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,23);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,16);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,8);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,13);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left_right_right = x_left_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right_right.as_ref().unwrap().borrow().key,14);
    assert_eq!(x_left_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_right_left = x_left_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_right_left.as_ref().unwrap().borrow().key,12);
    assert_eq!(x_left_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn delete_test_1() {
    let mut x = RBTree::new();
    x.insert(15);
    x.insert(11);
    x.insert(19);
    x.insert(8);
    x.insert(13);
    x.insert(16);
    x.insert(23);
    x.insert(12);
    x.insert(14);
    x.delete(13);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,15);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,19);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,11);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,23);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,16);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,8);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,14);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left_right_left = x_left_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_right_left.as_ref().unwrap().borrow().key,12);
    assert_eq!(x_left_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn insert_test_2() {
    let mut x = RBTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(30);
    x.insert(2);
    x.insert(9);
    x.insert(25);
    x.insert(40);
    x.insert(38);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,40);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,25);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right_left = x_right_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().key,38);
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,2);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,9);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn delete_test_2() {
    let mut x = RBTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(30);
    x.insert(2);
    x.insert(9);
    x.insert(25);
    x.insert(40);
    x.insert(38);
    x.delete(30);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,38);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,40);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,25);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,2);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,9);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn insert_test_3() {
    let mut x = RBTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(15);
    x.insert(30);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,15);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn delete_test_3() {
    let mut x = RBTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(15);
    x.insert(30);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn insert_test_4() {
    let mut x = RBTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(1);
    x.insert(7);
    x.insert(15);
    x.insert(30);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,15);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn delete_test_4() {
    let mut x = RBTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(1);
    x.insert(7);
    x.insert(15);
    x.insert(30);
    x.delete(15);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn insert_test_5() {
    let mut x = RBTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(1);
    x.insert(7);
    x.insert(15);
    x.insert(30);
    x.insert(25);
    x.insert(40);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,15);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right_left = x_right_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().key,25);
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_right_right = x_right_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().key,40);
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn delete_test_5() {
    let mut x = RBTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(1);
    x.insert(7);
    x.insert(15);
    x.insert(30);
    x.insert(25);
    x.insert(40);
    x.delete(15);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,40);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,20);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_left_right = x_right_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_left_right.as_ref().unwrap().borrow().key,25);
    assert_eq!(x_right_left_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn insert_test_6() {
    let mut x = RBTree::new();
    x.insert(1);
    x.insert(5);
    x.insert(7);
    x.insert(10);
    x.insert(20);
    x.insert(25);
    x.insert(28);
    x.insert(30);
    x.insert(40);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,25);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,20);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right_left = x_right_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().key,28);
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_right_right = x_right_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().key,40);
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);
    assert_eq!(x_left_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Black);
}

#[test]
pub fn delete_test_6() {
    let mut x = RBTree::new();
    x.insert(1);
    x.insert(5);
    x.insert(7);
    x.insert(10);
    x.insert(20);
    x.insert(25);
    x.insert(28);
    x.insert(30);
    x.insert(40);
    x.delete(1);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,25);
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,20);
    assert_eq!(x_right_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right_left = x_right_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().key,28);
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_right_right_right = x_right_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().key,40);
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
    assert_eq!(x_left_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn test_letters() {
    let mut x = RBTree::new();
    x.insert("a");
    x.insert("b");
    x.insert("c");
    x.insert("p");
    x.insert("m");
    x.delete("c");

    assert_eq!(x.root.as_ref().unwrap().borrow().key,"b");
    assert_eq!(x.root.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,"m");
    assert_eq!(x_right.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,"a");
    assert_eq!(x_left.as_ref().unwrap().borrow().color,NodeColor::Black);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key, "p");
    assert_eq!(x_right_right.as_ref().unwrap().borrow().color,NodeColor::Red);
}

#[test]
pub fn test_min_max_1() {
    let mut a = RBTree::new();
    a.insert(455);
    a.insert(32);
    a.insert(4);
    a.insert(9);
    a.insert(12);
    a.insert(1);
    assert_eq!(a.min().as_ref().unwrap().borrow().key, 1);
    assert_eq!(a.max().as_ref().unwrap().borrow().key, 455);
}

#[test]
pub fn test_min_max_2() {
    let mut a = RBTree::new();
    a.insert("a");
    a.insert("f");
    a.insert("d");
    a.insert("g");
    a.insert("u");
    a.insert("c");
    assert_eq!(a.min().as_ref().unwrap().borrow().key, "a");
    assert_eq!(a.max().as_ref().unwrap().borrow().key, "u");
}