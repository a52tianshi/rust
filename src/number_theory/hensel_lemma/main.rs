

fn main(){
    trival_solve();
}

//const mod_base = 8
const  mod_base:i64  = 8;

//[7,1] : 1*x + 7
static polynomial_coefficients:[i64;2] = [6,2];
  

fn trival_solve()    {
    for i in 0..mod_base {
        if (i * polynomial_coefficients[1] + polynomial_coefficients[0]) % mod_base == 0 {
            println!("x = {}", i);
        }
    }
}