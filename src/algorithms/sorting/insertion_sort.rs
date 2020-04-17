/// 0((n**2)/2)
fn insertion_sort(array: &mut Vec<i32>) {
    let mut iterations = 0;
    for i in 1..array.len() {
        let mut sorted_index = i;
        while sorted_index > 0 && &array[sorted_index] < &array[sorted_index - 1] {
            iterations += 1;
            array.swap(sorted_index, sorted_index - 1);
            sorted_index -= 1;
        }
    }
    println!("\n *** Insertion sort iterations: {:?}", iterations);
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn test_insertion_sort() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}