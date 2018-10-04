pub fn find() -> Option<u32> {
    let product: Option<u32> = None;
    let (mut a, mut b) = (0, 0);
    let mut c = 0.0f64;
    for x in (1..1000) {
        a = x;
        for y in (1..1000) {
            b = y;
            c = (a*a + b*b) as f64;
            println!("c = {}", c.sqrt() );
        }
        println!("{}", x);
        break
    }

    product
}
