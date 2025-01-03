use num::Complex;
// 声明模块路径
use crate::parse::parse_pair;

fn complex_square_add_loop(c :Complex<f64>){
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z*z + c;
    }
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z*z + c;
    }
    return None;
}

fn parse_complex(s: &str) -> Option<Complex<f64>>{
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex{re, im}),
        None => None
    }
}