use Rust_Trees::rbtree::RBTree;
use Rust_Trees::avltree::AvlTree;

fn main() {
    let mut rb = RBTree::new();
    let mut avl = AvlTree::new();

    // inserting nodes
    rb.insert("c");
    rb.insert("b");
    rb.insert("a");
    avl.insert(3);
    avl.insert(2);
    avl.insert(1);

    rb.print_tree();
    avl.print_tree();
    println!("{}", rb.leaves());
    println!("{}", avl.leaves());
}