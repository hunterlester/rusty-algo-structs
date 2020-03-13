use criterion::{Criterion, BenchmarkId, criterion_group, criterion_main};
use rusty_algo_structs::data_structures::{BinarySearchTree, IterativeBinarySearchTree};

pub fn bst_benches(c: &mut Criterion) {
    let mut bst = BinarySearchTree::new(10);
    let mut bst_iter = IterativeBinarySearchTree::new(10);
    for i in 0..50 {
        bst.insert(i);
        bst_iter.insert(i);
    }
    let mut group = c.benchmark_group("BST Inorder");
    group.bench_with_input(BenchmarkId::new("Recursive", 1), &1, |b, _| b.iter(|| bst.inorder()));
    group.bench_with_input(BenchmarkId::new("Iterative", 2), &2, |b, _| b.iter(|| bst_iter.inorder()));
    group.finish();
}

criterion_group!(benches, bst_benches);
criterion_main!(benches);