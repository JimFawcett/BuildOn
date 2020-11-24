/////////////////////////////////////////////////////////////
/// BuildOn::step_1::lib.rs
///
/// Jim Fawcett, https://JimFawcett.github.io, 23 Nov 2020 
/////////////////////////////////////////////////////////////
/// Notes:
/// 1. Two string types are defined in the std library
///    a. String is a type that manages a collection of
///       utf-8 characters in the native heap.
///    b. str is a type that represents literal strings
///       stored in stack or static memory, e.g., fixed
///       size arrays of utf-8 characters.
///    c. str instances are almost always managed through
///       references, e.g., &str
///    d. It is easy to convert between String and str 
///       types - see code below.
/// https://github.com/Dhghomon/easy_rust#strings
/// 2. Rust uses Structs where C++ and C# use classes.
///    e. Methods are defined using impl blocks, as shown
///       below.
///    f. Associated methods, like new are not declared 
///       with a reference to self.  They act like C++
///       static methods.
///    g. All other methods are declared with a reference
///       to self, which may be mutable.  On invocation,
///       &self or &mut self is passed to the method code
///       implicitly, e.g., not supplied by the using
///       code. 
/// https://github.com/Dhghomon/easy_rust#implementing-structs-and-enums

#[derive(Debug, Clone, Default)]
pub struct Demo {
    value: f64,
    name: String
}

impl Demo {
    pub fn new() -> Self {
        Demo {
            value: 0.0,
            name: String::from("un_named"),
        }
    }
    pub fn set_value(&mut self, v:f64) {
        self.value = v
    }
    pub fn get_value(&self) -> f64 {
        self.value
    }
    pub fn set_name(&mut self, nm:&str) {
        self.name = String::from(nm);
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}////////////////////////////////////////////////////////////
/// Testing:
/// ---------------------------------------------------------
/// Rust libraries can be configured with unit tests,
/// as you see below.  You run them with the command:
///   cargo test
/// By default, cargo runs each test on a separate thread,
/// so you can not count on the ordering of tests.
/// If you want to run all tests on a single thread, which
/// will preserve the test order, use the command: 
///   cargo test -- --test-threads=1
#[cfg(test)]
/// Notes:
/// 1. test_construct() uses typ_name method from std::any
/// 2. All tests use the fact that private data in a struct
///    is accessible (only) within the defining library
mod tests {
    use super::*;
    #[test]
    fn test_construct() {
        let demo = Demo::new();
        let tn = std::any::type_name::<Demo>();
        assert_eq!(tn, "step_1::Demo");
        assert_eq!(demo.value, 0.0);
        assert_eq!(demo.name, "un_named".to_string());
    }
   #[test]
   fn test_setters() {
        let mut demo = Demo::new();
        demo.set_name("test2");
        demo.set_value(3.14159);
        assert_eq!(demo.name, "test2".to_string());
        assert_eq!(demo.value, 3.14159);
   }
   #[test]
   fn test_getters() {
       let mut demo = Demo::new();
       demo.name = String::from("test3");
       demo.value = 0.33333;
       assert_eq!(demo.get_name(), "test3".to_string());
       assert_eq!(demo.get_value(), 0.33333);
   }
}
