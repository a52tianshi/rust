fn main() {
    trival_solve();
}

//const mod_base = 8
const MOD_BASE: i64 = 8;

//[7,2,1] : 7*x^2 + 2*x + 1
static POLYNOMIAL_COEFFICIENTS: [i64; 3] = [0, 2, 1];

fn trival_solve() {
    for x in 0..MOD_BASE {
        let mut result: i64 = 0;
        let highest_c: u32 = (POLYNOMIAL_COEFFICIENTS.len() - 1) as u32;
        for j in 0..POLYNOMIAL_COEFFICIENTS.len() {
            result += POLYNOMIAL_COEFFICIENTS[j] * x.pow(highest_c - j as u32);
        }
        let y = result % MOD_BASE;
        println!("x = {}, y = {}", x, y);
    }
}
