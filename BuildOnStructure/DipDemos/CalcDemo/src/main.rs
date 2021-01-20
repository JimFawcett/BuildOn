/////////////////////////////////////////////////////////////
// diip_calc::main.rs                                      //
//   - demonstrates Dependency Inversion Principle         //
//     with calculator                                     //
// Jim Fawcett, https://JimFawcett.github.io, 19 Jan 2021  //
/////////////////////////////////////////////////////////////
/*
    Demonstrates Dependency Inversion Principle:
      "High level modules should not depend on
       low level modules.  Both should depend
       on abstractions."

    This demonstration builds a calculator for 
    adding and multiplying two copy types.
      - High level part: Demo<U,T>
      - Low level parts: Plust<T>, Times<T>
      - Abstraction defined in this package: 
        - Calc<T> defined here
      - Abstractions defined by the Rust std::libs
        https://doc.rust-lang.org/beta/std/:
        - std::marker::Copy
        - std::ops::Add
        - std::ops::Mul
        - std::default::Default
    The definitons of Plus<T> and Times<T> could be
    changed in any way that is compatible with these
    abstractions without affecting compilation of
    Demo<U,T>.
*/

#![allow(dead_code)]
use std::ops::{Add, Mul};
use std::default::Default;
use std::marker::{Copy, PhantomData};

/*-----------------------------------------------------
  All calculator classes must have these functions.
*/
pub trait Calc<T> {
    fn new() -> Self;
    fn calc(arg1:T, arg2:T) -> T;
}
/*-----------------------------------------------------
  Adds two Copy types
*/
struct Plus<T> where T: Copy + Add + Add<Output = T> {
    phantom: PhantomData<T>
}
impl<T> Calc<T> for Plus<T> where T: Copy + Add + Add<Output = T> {
    fn new() -> Self {
        Plus {
            phantom: PhantomData
        }
    }
    fn calc(arg1:T, arg2:T) -> T {
        arg1 + arg2
    }
}
/*-----------------------------------------------------
   Multiplies two Copy types
*/
struct Times<T> where T: Copy + Mul + Mul<Output = T> {
    phantom: PhantomData<T>
}
impl<T> Calc<T> for Times<T> where T: Copy + Mul + Mul<Output = T> {
    fn new() -> Self {
        Times {
            phantom: PhantomData
        }
    }
    fn calc(arg1:T, arg2:T) -> T {
        arg1 * arg2
    }
}
/*----------------------------------------------------- 
  Demo uses any Calc type without the need to know
  which one.  It only needs to know that it's Calc.

  It uses Calc's factory function new() to generate
  an instance of the calculator function without
  knowing what type its using.

  So Demo is isolated from calculators implemenation,
  just depending on the Calc and Copy traits.

    U is the type of a calculator function.
    T is the type of the calculation data.
*/
struct Demo<U,T> where U: Calc<T>, T: Copy + Default {
    oper: U,
    result: T
}
impl<U,T> Demo<U,T> where U: Calc<T>, T: Copy + Default {
    fn new() -> Demo<U, T> {
        Demo {
           oper: U::new(),
           result: T::default()
        }
    }
    fn do_calc(&mut self, arg1:T, arg2:T) -> T {
        self.result = U::calc(arg1, arg2);
        self.result
    }
    fn get_result(&self) -> T {
        self.result
    }
}
/*----------------------------------------------------- 
  The executive, main(), needs to have all of the type
  information. 
*/
fn main() {
    let mut demo = Demo::<Times<i32>, i32>::new();
    let result = demo.do_calc(21, 2);
    print!("\n  Times(21, 2) = {:?}", result);
    print!("\n  saved result = {:?}", demo.get_result());
    println!();

    let mut demo = Demo::<Plus<f64>, f64>::new();
    let result = demo.do_calc(21.0, 2.0);
    print!("\n  Plus(21.0, 2.0) = {:?}", result);
    print!("\n  saved result    = {:?}", demo.get_result());

    println!("\n\n  That's all Folks!\n\n");
}
