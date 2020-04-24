fn get_row(row_index: i32) -> Vec<i32> {
    let mut rows: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];
    while rows.len() - 1 < row_index as usize {
        let mut new_row = vec![1];
        let mut pointer_1: usize = 0;
        let mut pointer_2: usize = 1;
        if let Some(row) = rows.get(rows.len() - 1) {
            while pointer_2 < row.len() {
                new_row.push(row[pointer_1] + row[pointer_2]);
                pointer_1 += 1;
                pointer_2 += 1;
            }
            new_row.push(1);
        }
        rows.push(new_row);
    }
    if let Some(row) = rows.get(row_index as usize) {
        row.clone()
    } else {vec![]}
}

#[cfg(test)]
mod tests {
    use super::{get_row};

    #[test]
    fn test_get_row() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(get_row(9), vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1]);
        assert_eq!(get_row(0), vec![1]);
        assert_eq!(get_row(1), vec![1, 1]);
    }
}