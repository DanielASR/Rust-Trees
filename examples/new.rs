use Rust_Trees::rbtree::RBTree;
use Rust_Trees::avltree::AvlTree;

fn main() {
    let rb = RBTree::<u32>::new();
    let avl = AvlTree::<u32>::new();

    assert_eq!(rb.is_empty(), true);
    assert_eq!(avl.is_empty(), true);
}