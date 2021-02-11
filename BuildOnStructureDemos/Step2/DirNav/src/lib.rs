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
    fn do_dir(&mut self, dir:&Path);
    fn do_file(&mut self, file: &Path);
}

/*-----------------------------------------------
  DirNav searches directory tree looking for
  files with specified patterns (extensions).

  App is a DirNav member that implements 
  application specific processing of a 
  DirEvent sent by DirNav::visit.  
  
  See test1.rs in examples directory for an
  example App type.

  This DirNav is a mock type defined as an 
  illustration of what Step #2 needs.
*/
pub struct DirNav<T> where T:DirEvent {
    app : T
}
impl<T> DirNav<T> where T:DirEvent {
    pub fn new() -> DirNav<T> {
        DirNav {
          app: T::new()  // using factory function
        }
    }
    pub fn get_app(&self) -> &T {
        &self.app
    }
    pub fn visit(&mut self, path: &Path) {
        /* pretending to search a dir tree */
        self.app.do_dir(path);
        self.app.do_file(&Path::new("file1"));
        self.app.do_file(&Path::new("file2"));
        self.app.do_dir(&Path::new("dir2"));
        self.app.do_file(&Path::new("file3"));
    }
    pub fn path_to_string(path:&Path) -> String {
        format!("{:?}", path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    struct App {
        test_file: PathBuf,
        test_dir: PathBuf
    }
    impl DirEvent for App {
        fn new() -> App {
            App {
                test_file: PathBuf::from(""),
                test_dir: PathBuf::from("")
            }
        }
        fn do_dir(&mut self, dir: &Path) {
            self.test_dir = dir.to_path_buf();
        }
        fn do_file(&mut self, file: &Path) {
            self.test_file = file.to_path_buf();
        }
    }
    #[test]
    fn test_events() {
        let mut dn = DirNav::<App>::new();  
        let test_path = Path::new("test");
        dn.visit(test_path);
        let app = dn.get_app();
        let dir_app:String = format!("{:?}", app.test_dir);
        let dir_rqt:String = format!("{:?}", "dir2");
        assert_eq!(dir_app, dir_rqt);   
        let file_app = format!("{:?}", app.test_file);
        let file_rqt = format!("{:?}", "file3");
        assert_eq!(file_app, file_rqt);   
    }
}
