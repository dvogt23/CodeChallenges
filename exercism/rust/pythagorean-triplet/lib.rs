pub fn find() -> Option<u32> {
    let product: Option<u32> = None;
    let (mut a, mut b, mut c) = (0, 0, 0);
    for x in (1..1000) {
        c = 998 - x;
        a = if ((1000 - c) % 2 == 0) {(1000 - c) / 2} else {(1000 - c) % 2};
        b = 1000 - c - a;
        println!("a: {} b: {} c: {}", a,b,c);
        if (a*a + b*b) == c*c {
            break
        }
    }

    product
}
