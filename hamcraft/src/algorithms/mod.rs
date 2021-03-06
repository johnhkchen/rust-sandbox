pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        let t:u64 = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::gcd;
    #[test]
    fn it_can_gcd_simple() {

        assert_eq!(gcd(15, 9), 3);
        assert_eq!(gcd(10, 12), 2);
        assert_eq!(gcd(10, 13), 1);
        assert_eq!(gcd(20, 40), 20);
        assert_eq!(gcd(24, 24), 24);
        assert_eq!(gcd(24, 8), 8);
    }

    #[test]
    fn it_can_gcd_moderate() {
        assert_eq!(gcd(10, 31), 1);
        assert_eq!(gcd(20, 24), 4);
        assert_eq!(gcd(40, 96), 8);
        assert_eq!(gcd(48, 12), 12);
        assert_eq!(gcd(12, 24), 12);
        assert_eq!(gcd(24, 64), 8);
    }

    #[test]
    fn it_can_gcd_hard() {
        assert_eq!(gcd(11571, 1767), 57);
        assert_eq!(gcd(69119, 19440), 1);
    }

    #[test]
    fn it_can_reduce_fractions() {
        assert_eq!(gcd(861, 984), 123);
        assert_eq!(861/984, 7/8);

        assert_eq!(gcd(132, 924), 132);
        assert_eq!(132/924, 1/7);

        assert_eq!(gcd(378, 868), 14);
        assert_eq!(378/868, 27/62);
    }

}
