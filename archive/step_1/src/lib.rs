/////////////////////////////////////////////////////////////
/// BuildOn::step_1::lib.rs
///
/// Jim Fawcett, https://JimFawcett.github.io, 23 Nov 2020 
/////////////////////////////////////////////////////////////
/// This demo illustrates how you define structs and use them
/// like classes in C++ and C#.
/// 
/// This package builds a static library.  There is a console
/// application, called test1 in the examples directory, sister
/// to the src directory where this file resides.
/// 
/// test1 uses this library by declaring:
///   uses step_1::*;
/// e.g., use any of the facilities provided by this library.
/// The name, step_1, is defined in Cargo.toml.
/// 
/// You build test1 and link to this library with the cargo command:
///   cargo run --example test1
/// Note that the option is example (singular) even though the directory
/// name is examples.
/////////////////////////////////////////////////////////////
/// Rust Notes:
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

/// Debug, Clone, and Default are traits, much like Java or C# interfaces.
/// Some traits, like these can be implemented by the compiler.  The
/// #[derive(Debug, Clone, Default)] annotation is asking the compiler
/// to do that.
/// 
#[derive(Debug, Clone, Default)]
pub struct Demo {
    value: f64,
    name: String
}

impl Demo {
    /// static constructor method
    /// -----------------------------------------
    pub fn new() -> Self {
        Demo {
            value: 0.0,
            name: String::from("un_named"),
        }
    }
    /// method that sets Demo::value to v
    /// -----------------------------------------
    pub fn set_value(&mut self, v:f64) {
        self.value = v
    }
    /// method that retrieves Demo::value
    /// -----------------------------------------
    pub fn get_value(&self) -> f64 {
        self.value
    }
    /// method that sets Demo::name
    /// -----------------------------------------
    /// Note that nm is passed by reference &str
    /// If we passed by value, e.g., nm:String
    /// that would move nm into the function and
    /// it would become invalid in the caller's scope
    /// -----------------------------------------
    pub fn set_name(&mut self, nm:&str) {
        self.name = String::from(nm);
    }
    /// method returns name as a reference
    /// -----------------------------------------
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
