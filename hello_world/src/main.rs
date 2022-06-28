fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn greet() -> &'static str{
    "Hello, John"
}

fn main() {
    let greeting = greet();
    println!("{}", greeting);
    let a:u64 = 378;
    let b:u64 = 868;
    println!("I will reduce the fraction {}/{} to lowest terms.", a, b);
    let d:u64 = gcd(a, b);
    println!("The GCD is {}", d);
    println!("The reduced fraction is {}/{}", a/d, b/d)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_greeting() {
        use crate::greet;
        let greeting = greet();
        assert_eq!(greeting, "Hello, John");
    }

    #[test]
    fn it_can_do_euclids_algorithm_simple() {
        use crate::gcd;
        assert_eq!(gcd(15, 9), 3);
        assert_eq!(gcd(10, 12), 2);
        assert_eq!(gcd(10, 13), 1);
        assert_eq!(gcd(20, 40), 20);
        assert_eq!(gcd(24, 24), 24);
        assert_eq!(gcd(24, 8), 8);
    }

    #[test]
    fn it_can_do_euclids_algorithm_moderate() {
        use crate::gcd;
        assert_eq!(gcd(10, 31), 1);
        assert_eq!(gcd(20, 24), 4);
        assert_eq!(gcd(40, 96), 8);
        assert_eq!(gcd(48, 12), 12);
        assert_eq!(gcd(12, 24), 12);
        assert_eq!(gcd(24, 64), 8);
    }

    #[test]
    fn it_can_do_euclids_algorithm_hard() {
        use crate::gcd;
        assert_eq!(gcd(11571, 1767), 57);
        assert_eq!(gcd(69119, 19440), 1);
    }

    #[test]
    fn it_can_help_reduce_fractions() {
        use crate::gcd;
        let mut a:u64 = 861;
        let mut b:u64 = 984;
        let mut d:u64 = gcd(a, b);
        assert_eq!(d, 123);
        assert_eq!(a/b, 7/8);

        a = 132;
        b = 924;
        d = gcd(a, b);
        assert_eq!(d, 132);
        assert_eq!(a/b, 1/7);

        a = 378;
        b = 868;
        d = gcd(a, b);
        assert_eq!(d, 14);
        assert_eq!(a/b, 27/62);
    }

}