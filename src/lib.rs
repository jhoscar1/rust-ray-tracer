mod point;
pub use crate::point::Point;

mod vector;
pub use crate::vector::Vector;

const EPSILON: f64 = 0.00001;

pub fn float_eq(f1: f64, f2: f64) -> bool {
    println!("{}", (f1 - f2).abs());
    (f1 - f2).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn float_eq_works() {
        assert_eq!(float_eq(2.000005, 2.000006), true);
        assert_eq!(float_eq(2.00007, 2.00006), false);
    }
}
