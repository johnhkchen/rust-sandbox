mod algorithms;
mod helpers;

fn greet() -> &'static str{
    "Hello, John"
}

fn reduce_fraction(a:u64, b:u64) -> String {
    let d:u64 = algorithms::gcd(a, b);
    format!("The reduced fraction is {}/{}", a/d, b/d)
}

fn instructions_for(name: &'static str) -> String {
    helpers::recipe_instructions(name)
}

fn main() {
    println!("{}", greet());
    println!("{}", reduce_fraction(378, 868));
    println!("{}", instructions_for("cake"));
    println!("{}", instructions_for("dropper"));
}

#[cfg(test)]
mod tests {
    use crate::greet;
    use crate::reduce_fraction;
    use crate::instructions_for;

    #[test]
    fn it_greets() {
        assert_eq!(
            "Hello, John",
            greet()
        );
    }

    #[test]
    fn it_reduces_fractions() {
        assert_eq!(
            "The reduced fraction is 1/9",
            reduce_fraction(11, 99)
        );
    }
    
    #[test]
    fn it_creates_cake_instructions() {
        assert_eq!(
            "It takes 3 wheat, 1 egg, 3 milk, and 2 sugars to craft cake",
            instructions_for("cake")
        );
    }

    #[test]
    fn it_creates_dropper_instructions() {
        assert_eq!(
            "It takes 8 cobblestone and 1 redstone to craft dropper",
            instructions_for("dropper")
        );
    }
}
