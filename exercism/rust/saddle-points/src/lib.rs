pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];

    // iter rows
    for i in 0..input.len() {
        let row = &*input.iter().nth(i).unwrap();
        let max_in_row = row.iter().max();

        if max_in_row.is_none() {
            return result
        }

        for j in 0..row.len() {

            let current_value = row.iter().nth(j).unwrap();

            if max_in_row.unwrap() != current_value {
                continue
            }

            // min in max index column
            let min_in_col = input.iter()
                .map(|row| row.iter().nth(j).unwrap())
                .min().unwrap();

            // saddle if max == min
            // add saddle point to result vec
            if current_value == min_in_col {
                result.push((i, j));
            }
        }
    }

    result
}
