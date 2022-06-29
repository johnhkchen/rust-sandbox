use std::env;

use hamcraft::instructions_for;


fn hamcraft(recipe_name: &str) -> String {
    instructions_for(recipe_name)
}

fn main() {
    let args = env::args().skip(1);
    
    if args.len() == 0 {
        println!("What recipe do you want?");
        return
    }

    for recipe_name in env::args().skip(1) {
        println!("{}", hamcraft(&recipe_name));
    }

}

#[cfg(test)]
mod tests {
    use crate::hamcraft;

    #[test]
    fn test_cake_message() {
        assert_eq!(
            hamcraft("cake"),
            "It takes 3 wheat 1 egg 3 milk 2 sugar to craft cake"
        )
    }

    #[test]
    fn test_bogus_message() {
        assert_eq!(
            hamcraft("nothing"),
            "It takes 1 air to craft nothing"
        )
    }
}
