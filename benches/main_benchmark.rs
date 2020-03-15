mod bst_benchmark;
mod fib_benchmark;

use criterion::{criterion_group, criterion_main};
use fib_benchmark::{fib_benches};
use bst_benchmark::{bst_benches};

criterion_group!(benches, fib_benches, bst_benches);
criterion_main!(benches);