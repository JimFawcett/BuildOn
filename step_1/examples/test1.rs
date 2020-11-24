/// BuildOn::step_1::test1.rs
/// 

use step_1::*;

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