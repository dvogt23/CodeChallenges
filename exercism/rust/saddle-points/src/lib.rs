pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let height = input.len();
    let width = input.first().unwrap().len();

    for (i, row) in input.iter().enumerate() {
        let max_in_row = row.iter().max().unwrap();
        for (j, col) in row.iter().enumerate() {
            if col == max_in_row {
                let lowest_in_col = input.iter().map(|row| row.iter().nth(j));
                println!("low: {:?}", lowest_in_col);
            }
        }
        
        //println!("max {:?} i: {:?}", max_in_row, i);
    }
    
    println!("{}, {:?}", height, width);

    vec![(0,0)]
}
