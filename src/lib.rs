struct Interpreter{
    codes: String,
}

impl Interpreter{

    /// Creates a new interpreter instance with the given codes.
    fn new(codes: String) -> Self {
        Self {codes}
    }

    fn run(&self) {
        println!("{}",self.codes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interpreter() {
        let my_interpreter = Interpreter::new(String::from("some codes"));
        assert_eq!(my_interpreter.codes, "some codes");
    }

    #[test]
    fn test_run_interpreter() {
        let code = r#"
        print "Test"
        print "Hello World"
        "#;
        let my_interpreter = Interpreter::new(String::from(code));
        my_interpreter.run();
        // Add assertions here to test the behavior of the run function
    }

}

