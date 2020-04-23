fn merge(sub_array_1: Vec<i32>, sub_array_2: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut pointer_1: usize = 0;
    let mut pointer_2: usize = 0;
    loop {
        if let Some(value_1) = sub_array_1.get(pointer_1) {
            if let Some(value_2) = sub_array_2.get(pointer_2) {
                if value_1 < value_2 {
                    output.push(*value_1);
                    pointer_1 += 1;
                } else {
                    output.push(*value_2);
                    pointer_2 += 1;
                }
            } else {
                output.push(*value_1);
                pointer_1 += 1;
            }
        } else {
            if let Some(value_2) = sub_array_2.get(pointer_2) {
                output.push(*value_2);
                pointer_2 += 1;
            } else {
                break;
            }
        }
    }
    output
}

fn merge_sort(array: Vec<i32>) -> Vec<i32> {
    let array_len = array.len();
    if array_len <= 1 {
        return array;
    }
    let middle_index = array.len() / 2;
    let sub_array_1 = &array[0..middle_index];
    let sub_array_2 = &array[middle_index..];
    
    merge( merge_sort(sub_array_1.to_vec()), merge_sort(sub_array_2.to_vec()) )
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn test_merge_sort() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(merge_sort(array), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}