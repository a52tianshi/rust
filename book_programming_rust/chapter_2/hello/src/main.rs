fn main() {
    println!("Hello, world!");
    println!("gcd(12, 18) = {}", gcd(12, 18));
    println!("gcd(33, 34) = {}", gcd(33, 34));
}

// greatest common divisor , not recursive
fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// unit test
#[test]
fn test_gcd() {
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(33, 34), 1);
    assert_eq!(gcd(5, 10), 3);
}
