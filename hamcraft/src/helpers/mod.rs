use std::collections::HashMap;

fn convert(ingredients: Vec<(&str, u32)>) -> String {
    let mut instruction = String::new();

    for ingredient in ingredients {
        let step_text: String = format!("{} {}", ingredient.1, ingredient.0);
        let step: &str = &step_text[..];
        instruction.push_str(step);
        instruction.push(' ');
    }

    instruction.trim().to_string()
}

fn from_resource_list(name: &str) -> String {
    let cake_recipe: Vec<(&str, u32)> = Vec::from([
        ("wheat", 3),
        ("egg", 1),
        ("milk", 3),
        ("sugar", 2),
    ]);

    let bread_recipe: Vec<(&str, u32)> = Vec::from([
        ("wheat", 3),
    ]);

    let sword_recipe: Vec<(&str, u32)> = Vec::from([
        ("diamond", 2),
        ("stick", 1),
    ]);

    let dropper_recipe: Vec<(&str, u32)> = Vec::from([
        ("cobblestone", 8),
        ("redstone", 1),
    ]);

    let axe_recipe: Vec<(&str, u32)> = Vec::from([
        ("diamond", 3),
        ("stick", 2),
    ]);

    let cobblestone_wall_recipe: Vec<(&str, u32)> = Vec::from([
        ("cobblestone", 6),
    ]);

    let dummy_recipe: Vec<(&str, u32)> = Vec::from([
        ("air", 1)
    ]);

    let recipes: HashMap<&str, Vec<(&str, u32)>> = HashMap::from([
        ("cake", cake_recipe),
        ("bread", bread_recipe),
        ("diamond_sword", sword_recipe),
        ("dropper", dropper_recipe),
        ("diamond_axe", axe_recipe),
        ("cobblestone_wall", cobblestone_wall_recipe),
    ]);
    
    let queried_recipe = match recipes.get(name) {
        Some(recipe) => {
            recipe
        },
        _ => {
            &dummy_recipe
        }
    };

    let instructions: String = convert(queried_recipe.to_vec());

    instructions
}

pub fn recipe_instructions(name: &str) -> String {
    format!(
        "It takes {} to craft {}",
        from_resource_list(name),
        name
    )
}

#[cfg(test)]
mod tests {
    use super::recipe_instructions;
    use super::from_resource_list;
    use super::convert;

    #[test]
    fn it_creates_bread_instructions() {
        assert_eq!(
            "It takes 3 wheat to craft bread",
            recipe_instructions("bread")
        );
    }

    #[test]
    fn it_has_cake_instructions() {
        assert_eq!(
            "It takes 3 wheat 1 egg 3 milk 2 sugar to craft cake",
            recipe_instructions("cake")
        );
    }

    #[test]
    fn it_has_dropper_instructions() {
        assert_eq!(
            "It takes 8 cobblestone 1 redstone to craft dropper",
            recipe_instructions("dropper")
        );
    }

    #[test]
    fn it_has_axe_recipe() {
        assert_eq!(
            "It takes 3 diamond 2 stick to craft diamond_axe",
            recipe_instructions("diamond_axe")
        );
    }

    #[test]
    fn it_handles_nonsense_recipe() {
        assert_eq!(
            "It takes 1 air to craft wellness",
            recipe_instructions("wellness")
        );
    }

    #[test]
    fn it_generates_resource_list() {
        assert_eq!(
            "2 diamond 1 stick",
            from_resource_list("diamond_sword")
        );
    }

    #[test]
    fn it_converts_sword_to_ingredient_list() {
        let sword_recipe: Vec<(&str, u32)> = Vec::from([
            ("diamond", 2),
            ("stick", 1),
        ]);
        
        assert_eq!(
            "2 diamond 1 stick",
            convert(sword_recipe)
        );
    }

    #[test]
    fn it_converts_cake_to_ingredient_list() {
        let cake_recipe: Vec<(&str, u32)> = Vec::from([
            ("wheat", 3),
            ("egg", 1),
            ("milk", 3),
            ("sugar", 2),
        ]);
        
        assert_eq!(
            "3 wheat 1 egg 3 milk 2 sugar",
            convert(cake_recipe)
        );
    }

}
