pub fn fib_iter(target: usize) -> u128 {
    let mut sequence: Vec<u128> = vec![0, 1];
    while sequence.len() - 1 < target {
        let sequence_len = sequence.len();
        let next = sequence[sequence_len - 1] + sequence[sequence_len - 2];
        sequence.push(next);
    }
    sequence[target]
}

pub fn fib_recurse(target: u128) -> u128 {
    if target == 0 {
        return 0;
    }
    if target == 1 {
        return 1;
    }
    fib_recurse(target - 1) + fib_recurse(target - 2)
}

#[cfg(test)]
mod tests {
    use super::{fib_iter, fib_recurse};

    #[test]
    fn iterative_fibonacci() {
        assert_eq!(fib_iter(4), 3);
        assert_eq!(fib_iter(5), 5);
        assert_eq!(fib_iter(6), 8);
    }

    #[test]
    fn iterative_fibonacci_soak() {
        assert_eq!(fib_iter(100), 354_224_848_179_261_915_075);
    }

    #[test]
    fn recursive_fibonacci() {
        assert_eq!(fib_recurse(4), 3);
    }
}