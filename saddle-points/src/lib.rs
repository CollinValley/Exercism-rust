pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_max = vec![u64::min_value(); input.len()];
    let mut column_min = vec![u64::max_value(); input[0].len()];
    let mut saddle_points = Vec::<(usize, usize)>::new();

    for (row_idx, row_vec) in input.iter().enumerate() {
        for (col_idx, element) in row_vec.iter().enumerate() {
            if row_max[row_idx] < *element {
                row_max[row_idx] = *element;
            }
            if column_min[col_idx] > *element {
                column_min[col_idx] = *element;
            }
        }
    }

    for (row_idx, row_vec) in input.iter().enumerate() {
        for (col_idx, element) in row_vec.iter().enumerate() {
            if row_max[row_idx] == *element && column_min[col_idx] == *element {
                saddle_points.push((row_idx, col_idx));
            }
        }
    }
    saddle_points
}
