use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use Rust_Trees::rbtree::RBTree;
use Rust_Trees::avltree::AvlTree;

fn bench_rbtree(tree_size: i32){
    let mut t = RBTree::new();
    for i in 0..tree_size{
        t.insert(i);
    }
    let lowest = tree_size/10;
    for i in 0..lowest{
        t.search(i);
    }

}

fn criterion_benchmark_rbtree(c: &mut Criterion) {
    //c.bench_function("RBTree", |b| b.iter(|| bench_trees(black_box(20))));
    
    let mut test_group = c.benchmark_group("RBTree");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        test_group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                bench_rbtree(size);
            }
            )
        },
        );
    }
    test_group.finish();
}

fn bench_avltree(tree_size: i32){
    let mut t = AvlTree::new();
    for i in 0..tree_size{
        t.insert(i);
    }
    let lowest = tree_size/10;
    for i in 0..lowest{
        t.search(i);
    }

}

fn criterion_benchmark_avltree(c: &mut Criterion) {
    //c.bench_function("AvlTree", |b| b.iter(|| bench_trees(black_box(20))));
    
    let mut test_group = c.benchmark_group("AvlTree");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        test_group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                bench_avltree(size);
            }
            )
        },
        );
    }
    test_group.finish();
}

criterion_group!(benches, criterion_benchmark_rbtree, criterion_benchmark_avltree);
criterion_main!(benches);