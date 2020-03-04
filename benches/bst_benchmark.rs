use criterion::{Criterion, BenchmarkId, criterion_group, criterion_main};
use rusty_algo_structs::data_structures::BST;

pub fn bst_benches(c: &mut Criterion) {
    let mut bst = BST::new(10);
    for i in 0..100 {
        bst.insert(i);
    }
    let mut group = c.benchmark_group("BST Inorder");
    group.bench_with_input(BenchmarkId::new("Recursive", 1), &1, |b, _| b.iter(|| bst.inorder_traverse()));
    group.bench_with_input(BenchmarkId::new("Iterative", 2), &2, |b, _| b.iter(|| bst.inorder_iterate()));
    group.finish();
}

criterion_group!(benches, bst_benches);
criterion_main!(benches);