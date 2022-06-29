mod algorithms;
mod helpers;

fn reduce_fraction(a:u64, b:u64) -> String {
    let d:u64 = algorithms::gcd(a, b);
    format!("The reduced fraction is {}/{}", a/d, b/d)
}

fn instructions_for(name: &str) -> String {
    helpers::recipe_instructions(name)
}

fn main() {
    println!("{}", reduce_fraction(378, 868));
    println!("{}", instructions_for("cobblestone_wall"));
}

#[cfg(test)]
mod tests {
    use crate::reduce_fraction;
    use crate::instructions_for;

    #[test]
    fn it_reduces_fractions() {
        assert_eq!(
            "The reduced fraction is 1/9",
            reduce_fraction(11, 99)
        );
    }

    #[test]
    fn it_gives_cake_instructions() {
        assert_eq!(
            "It takes 3 wheat 1 egg 3 milk 2 sugar to craft cake",
            instructions_for("cake")
        );
    }
    #[test]

    fn it_gives_cobblestone_wall_instructions() {
        assert_eq!(
            "It takes 6 cobblestone to craft cobblestone_wall",
            instructions_for("cobblestone_wall")
        );
    }
}
