pub fn find() -> Option<u32> {
    let max = 1000;
    for a in 1..max {
        for b in 1..max {
            for c in 1..max {
                if a*a + b*b - c*c == 0 {
                    if a + b + c == 1000 {
                        //println!("a:{} b:{} c:{}", a, b, c);
                        return Some((a * b * c) as u32)
                    }
                } 
            }
        }
    }
    None
}
