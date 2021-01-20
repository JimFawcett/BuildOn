/////////////////////////////////////////////////////////////
// test1.rs - demonstrates mock DirNav operations          //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 15 Jan 2021  //
/////////////////////////////////////////////////////////////

use dir_nav::*;

/*-----------------------------------------------
  Type MyApp is a test type that:
  - is a generic parameter for DirNav<App>
  - responds to DirNav's events
  - can be retrieved with DirNav::get_app()
    and interrogated using its member functions.
*/
struct MyApp {}  // mock for TextSearch

impl DirEvent for MyApp {
    fn new() -> Self {
        MyApp {}
    }
    fn do_dir(&mut self, dir:&str) {
        print!("\n  directory: {:?}", dir);
    }
    fn do_file(&mut self, file:&str) {
        print!("\n    file: {:?}", file);
    }
}
impl MyApp {
    pub fn show_type(&self) {
        print!(
            "\n  my type is: {:?}", 
            std::any::type_name::<MyApp>()
        );
    }
}

fn main() {
    print!("\n  -- demonstrate mock DirNav --\n");

    let mut dn = DirNav::<MyApp>::new();
    let path = std::path::Path("dir1");
    dn.visit(&path);
    println!();
    dn.get_app().show_type();

    print!("\n\n  That's all Folks!\n\n");
}