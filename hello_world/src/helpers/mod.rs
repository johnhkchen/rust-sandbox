use std::collections::HashMap;

pub fn recipe_instructions(name: &'static str) -> String {
    let instructions = HashMap::from([
        ("bread", "3 wheat"),
        ("cake", "3 wheat, 1 egg, 3 milk, and 2 sugars"),
        ("dropper", "8 cobblestone and 1 redstone"), 
    ]);

    format!(
        "It takes {} to craft {}",
        match instructions.get(name) {
            Some(instructions) => instructions,
            None => "nothing"
        },
        name
    )
}

#[cfg(test)]
mod tests {
    use super::recipe_instructions;

    #[test]
    fn it_creates_bread_instructions() {
        assert_eq!(
            "It takes 3 wheat to craft bread",
            recipe_instructions("bread")
        );
    }

    #[test]
    fn it_creates_cake_instructions() {
        assert_eq!(
            "It takes 3 wheat, 1 egg, 3 milk, and 2 sugars to craft cake",
            recipe_instructions("cake")
        );
    }

    #[test]
    fn it_creates_dropper_instructions() {
        assert_eq!(
            "It takes 8 cobblestone and 1 redstone to craft dropper",
            recipe_instructions("dropper")
        );
    }

}
