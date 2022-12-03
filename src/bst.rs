use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;
use std::cmp::max;

type TreeNode<T> = Rc<RefCell<Node<T>>>;
type Tree<T> = Option<TreeNode<T>>;

#[derive(Clone, PartialEq)]
pub struct Node<T: Ord+Display+Debug>{
    key: T,
    left : Tree<T>,
    right : Tree<T>,
}

impl <T> Node<T>
where T: Debug+Ord+Display+Copy{
    pub fn new(key :T) -> Tree<T>{
        Some(Rc::new(RefCell::new(Node{key:key,left:None,right:None})))
    }
}

pub struct BST<T:Ord + Display + Debug + Copy>{
    root : Tree<T>,

}

impl<T> fmt::Debug for Node<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
         .field("key", &self.key)
         .field("right", &self.right)
         .field("left", &self.left)
         .finish()
    }
}

#[derive(Clone, Debug, PartialEq)]

trait _Tree<T>
where T: Ord+Display+Debug+Clone+Copy{
    fn new(key:T) -> Tree<T>;
    fn do_insert(&self, root:Tree<T>,val: T) -> Tree<T>;
}
impl <T> _Tree<T> for Tree<T>
where T: Ord+Display+Debug+Clone+Copy{
    fn new(key:T) -> Tree<T> {
        Node::new(key)
    }
    

    fn do_insert(&self,tree:Tree<T>,key: T) -> Tree<T> {
        match tree {
            None => {
                let add_node = Self::new(key);
                add_node.clone()
            }
            Some(root) => {
                let clone_node = root.borrow().clone();
                let balanced_tree :Tree<T>;
                let updated_tree:Tree<T>;
                let sub_node:Tree<T>;
                if key == clone_node.key {
                    Some(root.clone())
                } 
                else if key < clone_node.key {
                    sub_node = root.borrow().left.clone();
                    let result = self.do_insert(sub_node,key);
                    let result_node = result;
                    root.borrow_mut().left = result_node;
                
                    Some(root.clone())
                }
                //进入右子树递归插入
                else {
                    sub_node = root.borrow().right.clone();
                    let result = self.do_insert(sub_node,key);
                    let result_node = result;
                    root.borrow_mut().right = result_node;
                    Some(root.clone())
                }
            }
        }
    }




}
impl <T> BST<T>
where T: Ord+Display+Debug+Clone+Copy{

    pub fn new() -> Self{
        BST { root: None}
    }

    pub fn insert(&mut self,key:T){
        let root_node = self.root.clone();
        let res_tree = self.root.do_insert(root_node,key);
        self.root = res_tree;
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
 
}
