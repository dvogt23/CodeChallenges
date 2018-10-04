pub fn start(input: String) {
    println!("Input: {:?}", input.trim());
    let result: i32 = input.split(" ")
        .into_iter()
        .map(|i| {
            match i.parse::<i32>() {
                Ok(n) => n,
                Err(_e) => 0,
            }
        })
        .sum();
        
    println!("Result: {:?}", result);
}
