#[derive(Debug, Default)]
pub struct Test {
    arg1: String,
    arg2: usize
}
impl Test {
    pub fn new() -> Self {
        Self {
            arg1: String::from("a string"),
            arg2: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct() {
        let test = Test::new();
        assert_eq!(&test.arg1, "a string");
        assert_eq!(test.arg2, 0);
    }
}
