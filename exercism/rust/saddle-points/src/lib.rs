pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec![];
    for (r, row) in input.iter().enumerate() {
        for (c, &col) in row.iter().enumerate() {
            let max_in_row = row.iter().map(|&d| d).max().unwrap();
            let min_in_col = input.iter().map(|row| row[c]).min().unwrap();

            println!("min: {} max: {}", min_in_col, max_in_row);

            if col == min_in_col && col == max_in_row {
                saddle_points.push((r, c));
            }
        }
    }

    saddle_points
}
