use std::rc::Rc;
use std::cell::RefCell;

/// O(n - 1), possibly, still learning to calculate properly
pub fn fib_iter(target: usize) -> u128 {
    let mut sequence: Vec<u128> = vec![0, 1];
    while sequence.len() - 1 < target {
        let sequence_len = sequence.len();
        let next = sequence[sequence_len - 1] + sequence[sequence_len - 2];
        sequence.push(next);
    }
    sequence[target]
}

/// Recursive implementation is superior in time performance over
/// dynamic and iterative implementations until about the 11th term
/// in the fibonacci sequence, where it's overtaken by other two.
/// O((n - 1)**2), possibly, still learning to calculate properly
pub fn fib_recurse(target: u128) -> u128 {
    if target == 0 {
        return 0;
    }
    if target == 1 {
        return 1;
    }
    fib_recurse(target - 1) + fib_recurse(target - 2)
}

/// O(n - 1), possibly, still learning to calculate properly
pub fn fib_dyn(target: u128) -> u128 {
    let cache: Rc<RefCell<Vec<u128>>> = Rc::new(RefCell::new(vec![0, 1]));
    fn recurse(n: u128, cache: Rc<RefCell<Vec<u128>>>) -> u128 {
        if let Some(value) = cache.borrow().get(n as usize) {
            return *value;
        }
        let recursive_value = recurse(n - 1, Rc::clone(&cache)) + recurse(n - 2, Rc::clone(&cache));
        cache.borrow_mut().insert(n as usize, recursive_value);
        recursive_value
    }
    recurse(target, Rc::clone(&cache))
}

#[cfg(test)]
mod tests {
    use super::{fib_iter, fib_recurse, fib_dyn};

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
        assert_eq!(fib_recurse(6), 8);
    }

    #[test]
    fn dynamic_fib() {
        assert_eq!(fib_dyn(4), 3);
        assert_eq!(fib_dyn(6), 8);
    }

    #[test]
    fn dynamic_fib_soak() {
        assert_eq!(fib_dyn(100), 354_224_848_179_261_915_075);
    }
}