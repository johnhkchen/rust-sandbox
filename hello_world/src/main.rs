fn greet() -> &'static str{
    "Hello, John"
}

fn main() {
    let greeting = greet();
    println!("{}", greeting);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_greeting() {
        use crate::greet;
        let greeting = greet();
        assert_eq!(greeting, "Hello, John");
    }
}