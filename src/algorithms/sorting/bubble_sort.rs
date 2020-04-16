/// O(n**2)
pub fn bubble_sort(mut array: Vec<i32>) -> Vec<i32> {
    let mut i: usize = 0;
    let mut swapped = false;
    loop {
        if i == 0 {
            swapped = false;
        }
        if array[i] > array[i + 1] {
            array.swap(i, i + 1);
            swapped = true;
        }
        i += 1;
        if i == array.len() - 1 {
            i = 0;
            if swapped == false {
                break;
            }
        }
    }
    array
}

#[cfg(test)]
mod tests {
    use super::{bubble_sort};

    #[test]
    fn test_bubble_sort() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(bubble_sort(array), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}