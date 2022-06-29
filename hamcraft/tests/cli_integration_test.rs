extern crate assert_cli;


#[cfg(test)]
mod tests {
    #[test]
    fn test_cli_tutorial() {
        // Running the hamcraft command without a recipe name gives an error with some instructions
        assert_cli::Assert::main_binary()
        .succeeds()
        .stdout()
        .is("What recipe do you want?\n")
        .unwrap();
    }

    #[test]
    fn test_cli_usage() {
        // Running the hamcraft command with a recipe name gives its components
        assert_cli::Assert::main_binary()
        .with_args(&["cobblestone_wall"])
        .succeeds()
        .stdout()
        .is("It takes 6 cobblestone to craft cobblestone_wall\n")
        .unwrap();
    }

    #[test]
    fn test_cli_usage_alternate() {
        // Running the hamcraft command with a recipe name gives its components
        assert_cli::Assert::main_binary()
        .with_args(&["cake"])
        .succeeds()
        .stdout()
        .is("It takes 3 wheat 1 egg 3 milk 2 sugar to craft cake\n")
        .unwrap();
    }

}
