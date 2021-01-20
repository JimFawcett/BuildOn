/////////////////////////////////////////////////////////////
// Step#2::DirNav::lib.rs - demos mock DirNav operations   //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 15 Jan 2021  //
/////////////////////////////////////////////////////////////

use std::path::{Path};

/*-----------------------------------------------
  Rust traits are similar to interfaces.

  Trait DirEvent requires implementer to be
  constructible and have members to handle
  DirNav events.

  Constructability allows DirNav to create an
  instance for a type defined by a generic 
  parameter.
*/
pub trait DirEvent {
    /*-- constructible --*/
    fn new() -> Self;
    /*-- event handlers --*/
    fn do_dir(&mut self, dir:&str);
    fn do_file(&mut self, file: &str);
}

//#region : Sample App implementing DirEvent
/*-----------------------------------------------
  App is a type that implements application
  specific processing of a DirEvent sent by
  DirNav.  This is a demo for the DirNav library
  that illustrates how an application can use
  DirNav.
  -----------------------------------------------
pub struct App {
    curr_dir: String
}
impl DirEvent for App {
    fn new() -> Self {
        App { curr_dir: String::new() }
    }
    fn do_dir(&mut self, dir: &str) {
        print!("\n  directory: {:?}",dir);
        self.curr_dir = dir.to_string();
    }
    fn do_file(&mut self, file: &str) {
        print!("\n    file: {:?}", file);
    }
}
*/
//#endregion

/*-----------------------------------------------
  DirNav searches directory tree looking for
  files with specified patterns (extensions).

  This is a mock type defined as an illustration
  of what Step #2 needs.
*/
pub struct DirNav<T> where T:DirEvent {
    app : T
}
impl<T> DirNav<T> where T:DirEvent {
    pub fn new() -> DirNav<T> {
        DirNav {
          app: T::new()
        }
    }
    pub fn get_app(&self) -> &T {
        &self.app
    }
    pub fn visit(&mut self, path: &Path) {
        /* pretending to search a dir tree */
        let path_string = Self::path_to_string(path);
        self.app.do_dir(&path_string);
        self.app.do_file("file1");
        self.app.do_file("file2");
        self.app.do_dir("dir2");
        self.app.do_file("file3");
    }
    pub fn path_to_string(path:Path) -> String {
        format!("{}", path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
