pub fn find() -> Option<u32> {
    let max = 1000;
    for a in 1..max {
        for b in a+1..max {
            let c = max - a -b;
            if a*a + b*b - c*c == 0 {
                println!("a:{} b:{} c:{}", a, b, c);
                return Some((a * b * c) as u32)
            }
        } 
    }
    None
}
