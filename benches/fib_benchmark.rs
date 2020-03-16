use criterion::{Criterion, BenchmarkId};
use rusty_algo_structs::algorithms::{fib_iter, fib_recurse, fib_dyn};

pub fn fib_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in 20..21 {
      group.bench_with_input(BenchmarkId::new("Recursive", i), &i, |b, i| b.iter(|| fib_recurse(*i)));
      group.bench_with_input(BenchmarkId::new("Iterative", i), &i, |b, i| b.iter(|| fib_iter(*i as usize)));
      group.bench_with_input(BenchmarkId::new("Dynamic", i), &i, |b, i| b.iter(|| fib_dyn(*i)));
    }
    group.finish();
}