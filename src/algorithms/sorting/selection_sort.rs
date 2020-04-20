/// O((n**2)/2)
fn selection_sort(array: &mut Vec<i32>) {
    let mut iterations = 0;
    for i in 0..array.len() - 1 {
        let mut min_index = i;
        for j in i + 1..array.len() {
            iterations += 1;
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        array.swap(i, min_index);
    }

    println!("\n *** Selection sort iterations: {:?}", iterations);
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn test_selection_sort() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        selection_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}