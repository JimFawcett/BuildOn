/////////////////////////////////////////////////////////////
// basic_dip::main.rs                                      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 19 Jan 2021  //
/////////////////////////////////////////////////////////////
/*
    Demonstrates Dependency Inversion Principle:
      "High level modules should not depend on
       low level modules.  Both should depend
       on abstractions."

    This demonstration builds a basic demo with self
    annunciating low level components.
    
      - High level part: Demo<T>
      - Low level parts: First, Second
      - Abstraction defined in this package: 
        - trait Say
    The definitons of First and Second could be
    changed in any way that is compatible with 
    trait Say without affecting compilation of
    Demo<T>.
*/


#![allow(dead_code)]

/*-----------------------------------------------
  Trait Say provides an abstraction that Demo<T>
  uses to avoid depending on types First and Second.
*/
pub trait Say {
    fn new() -> Self;  // factory function
    fn set_id(&mut self, id: u8);
    fn get_id(&self) -> u8;
    fn say(&self);
}
/*-----------------------------------------------
  First is a component that Demo<T> depends on
  when the executive declares Demo<First>.
  Demo's compilation only depends on Say, not
  on the details of First.
*/
pub struct First {
    id: u8
}
impl Say for First {
    fn new() -> First {
        First {
            id: 0
        }
    }
    fn set_id(&mut self, id: u8) {
        self.id = id;
    }
    fn get_id(&self) -> u8 {
        self.id
    }
    fn say(&self) {
        print!("\n  First here with id = {:?}",self.id);
    }
}
/*-----------------------------------------------
  Second is a component that Demo<T> depends on
  when the executive declares Demo<Second>.
  Demo's compilation only depends on Say, not
  on the details of Second.
*/
pub struct Second {
    id: u8
}
impl Say for Second {
    fn new() -> Second {
        Second {
            id: 0
        }
    }
    fn set_id(&mut self, id: u8) {
        self.id = id;
    }
    fn get_id(&self) -> u8 {
        self.id
    }
    fn say(&self) {
        print!("\n  Second here with id = {:?}",self.id);
    }
}
/*-----------------------------------------------
  Demo is a high level type that uses low level
  types First and Second without incurring
  compilation dependencies on their implementations.
*/
struct Demo<T> where T: Say {
    my_say: T
}
impl<T> Demo<T> where T: Say {
    fn new() -> Demo<T> {
        Demo {
            my_say: T::new()  // using factory function
        }
    }
    fn set_id(&mut self, id:u8) {
        self.my_say.set_id(id);  // using trait method
    }
    fn get_id(&self) -> u8 {
        self.my_say.get_id()     // using trait method
    }
    fn say_it(&self) {
        self.my_say.say();       // using trait method
    }
}
/*-----------------------------------------------
  main() is the program executive.  It depends 
  directly on Demo, First, and Second.

  DIP allows a reusable component, which Demo<T>
  pretends to be, to be used in applications 
  without any changes, even though the parts
  it uses through traits change.
*/

fn main() {
    print!("\n  -- basic_dip demo --\n");

    let mut demo = Demo::<First>::new();
    demo.set_id(1);
    demo.say_it();
    let mut demo = Demo::<Second>::new();
    demo.set_id(2);
    demo.say_it();
    println!("\n\n  That's all Folks!\n\n");
}
