use redblacktree::rbtree::RBTree;

fn main() {
    let mut d = RBTree::new();
    d.insert(20);
    d.insert(25);
    dbg!(&d);
    d.delete(20);
    dbg!(&d);
    d.delete(25);
    dbg!(&d);
    d.delete(2);
    dbg!(&d);

    let mut e = RBTree::new();
    e.insert("mjh");
    e.insert("b");
    e.insert("ckl");
    e.insert("p");
    e.insert("m");
    dbg!(&e);
    e.delete("c");
    dbg!(&e);
    dbg!(e.height());
    e.print_inorder();
    e.print_preorder();
    dbg!(e.min());

    let mut a = RBTree::new();
    a.insert(455);
    a.insert(32);
    a.insert(4);
    a.insert(9);
    a.insert(12);
    a.insert(1);
    dbg!(a.min());
    dbg!(a.max());
    a.print_levelorder();
    dbg!(a.search(4));
}


