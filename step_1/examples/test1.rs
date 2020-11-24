/////////////////////////////////////////////////////////////
/// BuildOn::step_1::test1.rs
/// 
/// Jim Fawcett, https://JimFawcett.github.io, 24 Nov 2020 
/////////////////////////////////////////////////////////////
///
/// Any Rust file with a main function that resides in the
/// /examples directory can use the library from /src to
/// demonstrate features of the library.  You do that with
/// the command:
///     cargo run --example test1
/// This makes it very easy to provide demonstrations for
/// individual features tuned to specific audiences.

use step_1::*;  /// this provides access to the step_1 library

fn show_title() {
    print!("\n  -- Demonstration of Demo type --\n");
}

fn show_members(rdemo:&Demo) {
    print!("\n  demo.name  = {:?}", rdemo.get_name());
    print!("\n  demo.value = {:?}", rdemo.get_value());
}

fn main() {

    show_title();

    let mut demo = Demo::new();
    show_members(&demo);
    println!();

    demo.set_name("JoeDemo");
    demo.set_value(0.5);
    show_members(&demo);

    print!("\n\n  That's all Folks\n\n");
}