use criterion::*; //{black_box, criterion_group, criterion_main, BenchmarkId, Criterion，SamplingMode};
use std::time::Duration;

use Rust_Trees::rbtree::RBTree;
use Rust_Trees::avltree::AvlTree;
use Rust_Trees::bst::BST;



fn bench_rbtree_insert(tree_size: u32)->RBTree<u32>{
    let mut t = RBTree::new();

    for i in 0..tree_size{
        t.insert(i);
    }
    t
}

fn bench_rbtree_search(tree_size: u32,tree:RBTree<u32>){
    let lowest = tree_size/10;
    for i in 0..lowest{
        tree.search(i);
    }
}

fn criterion_benchmark_rbtree_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("RB Group");
    group.measurement_time(Duration::from_secs(13));
    group.bench_function("insert 10000 elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_insert(black_box(10000)))
    });
    group.bench_function("insert 40000 elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_insert(black_box(40000)))
    });
    group.bench_function("insert 70000 elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_insert(black_box(70000)))
    });
    group.bench_function("insert 100000 elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_insert(black_box(100000)))
    });
    group.bench_function("insert 130000 elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_insert(black_box(130000)))
    });
    group.finish();
}

fn criterion_benchmark_rbtree_search(c :&mut Criterion) {
    let mut tree = bench_rbtree_insert(10000);
    c.bench_function("search for the 1000 lowest elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_search(black_box(10000),tree.clone()))
    });
    tree = bench_rbtree_insert(40000);
    c.bench_function("4000 lowest elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_search(black_box(40000),tree.clone()))
    });
    tree = bench_rbtree_insert(70000);
    c.bench_function("7000 lowest elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_search(black_box(70000),tree.clone()))
    });
    let mut tree = bench_rbtree_insert(100000);
    c.bench_function("search for the 10000 lowest elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_search(black_box(100000),tree.clone()))
    });
    let mut tree = bench_rbtree_insert(130000);
    c.bench_function("search for the 10000 lowest elements in the Red-Black Tree", |b| {
        b.iter(|| bench_rbtree_search(black_box(130000),tree.clone()))
    });
}

fn bench_avltree_insert(tree_size: u32)->AvlTree<u32>{
    let mut t = AvlTree::new();
    for i in 0..tree_size{
        t.insert(i);
    }
    t
}

fn bench_avltree_search(tree_size: u32,tree:AvlTree<u32>){
    let lowest = tree_size/10;
    for i in 0..lowest{
        tree.search(i);
    }
}


fn criterion_benchmark_avltree_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVL Group");
    group.measurement_time(Duration::from_secs(13));
    group.bench_function("insert 10000 elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_insert(black_box(10000)))
    });
    group.bench_function("insert 40000 elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_insert(black_box(40000)))
    });
    group.bench_function("insert 70000 elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_insert(black_box(70000)))
    });
    group.bench_function("insert 100000 elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_insert(black_box(100000)))
    });
    group.bench_function("insert 130000 elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_insert(black_box(130000)))
    });
    group.finish();
}
fn criterion_benchmark_avltree_search(c :&mut Criterion) {
    let mut tree = bench_avltree_insert(10000);
    c.bench_function("search for the 1000 lowest elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_search(black_box(10000),tree.clone()))
    });
    tree = bench_avltree_insert(40000);
    c.bench_function("4000 lowest elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_search(black_box(40000),tree.clone()))
    });
    tree = bench_avltree_insert(70000);
    c.bench_function("7000 lowest elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_search(black_box(70000),tree.clone()))
    });
    tree = bench_avltree_insert(100000);
    c.bench_function("search for the 10000 lowest elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_search(black_box(100000),tree.clone()))
    });
    tree = bench_avltree_insert(130000);
    c.bench_function("search for the 10000 lowest elements in the AVL Tree", |b| {
        b.iter(|| bench_avltree_search(black_box(130000),tree.clone()))
    });
}

fn bench_bst_insert(tree_size: u32)->BST<u32>{
    let mut t = BST::new();
    for i in 0..tree_size{
        t.insert(i);
    }
   t
}

fn bench_bst_search(tree_size: u32,tree:BST<u32>){
    let lowest = tree_size/10;
    for i in 0..lowest{
        tree.search(i);
    }
}


// This will cause fatal runtime error: stack overflow

fn criterion_benchmark_bst_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bst Group");
    group.measurement_time(Duration::from_secs(13));
    group.bench_function("insert 10000 elements in the BST", |b| {
        b.iter(|| bench_bst_insert(black_box(10000)))
    });
    group.bench_function("insert 40000 elements in the BST", |b| {
        b.iter(|| bench_bst_insert(black_box(40000)))
    });
    group.bench_function("insert 70000 elements in the BST", |b| {
        b.iter(|| bench_bst_insert(black_box(70000)))
    });
    group.bench_function("insert 100000 elements in the BST", |b| {
        b.iter(|| bench_bst_insert(black_box(100000)))
    });
    group.bench_function("insert 130000 elements in the BST", |b| {
        b.iter(|| bench_bst_insert(black_box(130000)))
    });
    group.finish();
}

fn criterion_benchmark_bst_search(c :&mut Criterion) {
    let mut tree = bench_bst_insert(10000);
    c.bench_function("search for the 1000 lowest elements in the BST", |b| {
        b.iter(|| bench_bst_search(black_box(10000),tree.clone()))
    });
    tree = bench_bst_insert(40000);
    c.bench_function("4000 lowest elements in the BST", |b| {
        b.iter(|| bench_bst_search(black_box(40000),tree.clone()))
    });
    tree = bench_bst_insert(70000);
    c.bench_function("7000 lowest elements in the BST", |b| {
        b.iter(|| bench_bst_search(black_box(70000),tree.clone()))
    });
    let mut tree = bench_bst_insert(100000);
    c.bench_function("search for the 10000 lowest elements in the BST", |b| {
        b.iter(|| bench_bst_search(black_box(100000),tree.clone()))
    });
    let mut tree = bench_bst_insert(130000);
    c.bench_function("search for the 10000 lowest elements in the BST", |b| {
        b.iter(|| bench_bst_search(black_box(130000),tree.clone()))
    });
}

criterion_group!(benches,criterion_benchmark_rbtree_insert, criterion_benchmark_avltree_insert);
criterion_main!(benches);