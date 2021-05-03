/////////////////////////////////////////////////////////////
// TextFinder::text_search::lib.rs                         //
//   - find specified text in file                         //
//   - ver 1/1                                             //
// Jim Fawcett, https://JimFawcett.github.io, 26 Oct 2020  //
/////////////////////////////////////////////////////////////

use std::path::{Path, PathBuf};
use std::fs::{OpenOptions};
use std::io::*;
use dir_nav::{DirEvent};
use regex::Regex;

pub trait SearchEvent {
    fn new() -> Self;
    fn set_dir(&mut self, dir: &Path);
    fn set_file(&mut self, rslt:(&Path, bool, &str));
}

/*---------------------------------------------------------
  Trait declared by dir_nav
*/
// pub trait DirEvent {
//     fn new() -> Self;
//     fn do_dir(&mut self, d: &Path);
//     fn do_file(&mut self, f: &Path);
// }

/*---------------------------------------------------------
  Finder implements finding text strings in files
*/
#[derive(Debug, Default)]
pub struct Finder<T: SearchEvent> {
    dir : PathBuf,
    stxt : String,
    regx : Option<Regex>,
    rtxt : String,
    out : T
}
impl<T: SearchEvent> DirEvent for Finder<T> {
    fn new() -> Self {
        Self {
            dir: PathBuf::from("."),
            stxt: String::new(),
            regx: None,
            rtxt: String::new(),
            out: T::new()
        }
    }
    /*-- called by DirNav --*/
    fn do_dir(&mut self, dir: &Path) {
        self.dir = dir.to_path_buf();
        self.out.set_dir(dir);
    }
    /*-- called by DirNav --*/
    fn do_file(&mut self, file_name: &Path) {
        let path = Path::new(&self.dir);
        let path = path.join(file_name);
        let rslt = OpenOptions::new().read(true).open(path);
        if rslt.is_err() {
            self.out.set_file((file_name,false,"can't open file"));
            return;
        }
        let mut file = rslt.unwrap();
        
        let mut buffer = String::new();
        let rslt = file.read_to_string(&mut buffer);
        if rslt.is_ok() {
            if self.rtxt.is_empty() {
                let found:bool = buffer.contains(&self.stxt);
                self.out.set_file((file_name,found,&self.stxt));
            }
            else {
                let re_opt = &mut self.regx;
                if let Some(re) = re_opt {
                    let found:bool = re.is_match(buffer.as_str());
                    self.out.set_file((file_name,found,&self.stxt));
                }
            }
        }
    }
}
impl<T: SearchEvent> Finder<T> {
    /*-- called by Executive based on Cmdln opts --*/
    pub fn set_txt(&mut self, srctxt: &str) {
        self.stxt = srctxt.to_string();
    }
    /*-- called by Executive based on Cmdln opts --*/
    pub fn set_regex(&mut self, regex: &str) {
        self.rtxt = regex.to_string();
        let re = Regex::new(regex).expect("unvalid regex");
        self.regx = Some(re);
    }
    /*-- called by Executive to config GenOut --*/
    pub fn get_app(&mut self) -> &mut T {
        &mut self.out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dir_nav::{DirEvent};
    struct MockOut {
        dir: PathBuf,
        file: PathBuf,
        txt: String
    }
    impl SearchEvent for MockOut {
        fn new() -> Self {
            MockOut {
                dir: PathBuf::from("."),
                file: PathBuf::new(),
                txt: String::new()
            }
        }
        fn set_dir(&mut self, dir: &Path) {
            self.dir = PathBuf::from(dir);

        }
        fn set_file(&mut self, rslt:(&Path, bool, &str)) {
            self.file = PathBuf::from(rslt.0);
            self.txt = rslt.2.to_string();
        }
    }
    #[test]
    fn test_sets() {
        let mut f = Finder::<MockOut>::new();
        let dir = Path::new(".\\src");
        f.do_dir(&Path::new(".\\src"));
        assert_eq!(f.dir, dir);
        let text = "text";
        f.set_txt("text");
        assert_eq!(f.txt, text.to_string());
    }
    #[test]
    fn test_trait() {
        let mut f = Finder::<MockOut>::new();
        let dir = Path::new(".\\src");
        f.do_dir(dir);
        assert_eq!(f.dir, dir);
        let text = "text";
        let file = Path::new("lib.rs");
        f.set_txt(text);
        f.do_file(file);
        assert_eq!(f.out.dir, dir);
        assert_eq!(f.out.txt, text.to_string());
        assert_eq!(f.out.file, file);
    }
}
