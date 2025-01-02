fn main() {
    //1. Define the polynomial coefficients
    // 1 * x^3 - 2 * x^2 - 3 * x == 0 in Z(12)
    for x in 0..12 as i64 {
        let mut result: i64 = 0;
        result += 1 * x.pow(3 as u32);
        result -= 2 * x.pow(2 as u32);
        result -= 3 * x;
        let y = result % 12;
        println!("x = {}, y = {}", x, y);
    }
}
