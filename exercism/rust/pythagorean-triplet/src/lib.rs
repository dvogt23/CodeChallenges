pub fn find() -> Option<u32> {
    for a in 1..1000 {
        for b in 1..1000 {
            if (a as f64 + b as f64 + ((a*a) as f64 + (b*b) as f64).sqrt()) == 1000.0f64 {
                println!("a: {} b: {}", a, b);
                return Some((a as f64 * b as f64 * ((a*a + b*b) as f64).sqrt()) as u32)
            }
        }
    }
    None
}
