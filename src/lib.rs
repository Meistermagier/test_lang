#[derive(Debug, PartialEq)]
pub struct Number(pub i32); //defining the number type as a tupple struct

//Implementing the creation of a Number type from string input
impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        //test if the number is parsed correctly
        assert_eq!(Number::new("123"), Number(123))
    }

    #[test]
    fn test_number() {
        //Test to see if the stored value is called correctly
        let _num = Number(123);
        assert_eq!(_num.0, 123);
    }
}
