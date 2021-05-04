/////////////////////////////////////////////////////////////
// display::lib.rs - send text_finder data to console      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 06 Mar 2012  //
/////////////////////////////////////////////////////////////

use text_search::*;
use dir_nav::{replace_sep};
use std::path::{Path, PathBuf};

/*---------------------------------------------------------
  GenOut sends all program output to user
  - has options to:
    - show all results or only matches
    - show or hide directories with no matches
*/
#[derive(Debug, Default)]
pub struct GenOut {
    dir: PathBuf,
    file: PathBuf,
    txt: String,
    rslt: bool,
    show_all: bool,
    hide_unmatched: bool,
    dir_displayed: bool,
    debug: bool
}
impl SearchEvent for GenOut {
    fn new() -> Self {
        GenOut {
            dir: PathBuf::from("."),
            file: PathBuf::new(),
            txt: String::new(),
            rslt: false,
            show_all: false,
            hide_unmatched: true,
            dir_displayed: false,
            debug: false
        }
    }

    /*-- called by TextSearch::Finder --*/
    fn set_dir(&mut self, dir: &Path) {
        self.dir = dir.to_path_buf();
        self.dir_displayed = false;
        if !self.hide_unmatched {
            print!("\n  dir: {:?}", replace_sep(dir));
            self.dir_displayed = true;
        }
    }
    /*-- called by TextSearch::Finder --*/
    fn set_file(&mut self, rslt:(&Path, bool, &str)) {
        self.file = rslt.0.to_path_buf();
        self.txt = rslt.2.to_string();
        self.rslt = rslt.1;
        if rslt.1 || self.show_all {
            if !self.dir_displayed {
                print!("\n  dir: {:?}", replace_sep(&self.dir));
                self.dir_displayed = true;
            }
            if self.debug {
                print!("\n    ({:?}, {:?}, {:?})", rslt.0, rslt.1, rslt.2);
            }
            else if self.show_all {
                print!("\n    {:?}, {}", rslt.0, rslt.1);
            }
            else {
                print!("\n    {:?}", rslt.0);
            }
        }
    }
}
impl GenOut {
    /*-- called by Executive based on cmdln opts  --*/
    pub fn set_hide_unmatched(&mut self, h: bool) {
        self.hide_unmatched = h;
    }
    /*-- called by Executive based on cmdln opts  --*/
    pub fn show_all(&mut self, a: bool) {
        self.show_all = a;
    }
    pub fn set_debug(&mut self, db: bool) {
        self.debug = db;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct() {
        let go = GenOut::new();
        assert_eq!(go.dir, PathBuf::from("."));
        assert_eq!(go.file, PathBuf::new());
        assert_eq!(go.txt, String::new());
        assert_eq!(go.rslt, false);
        assert_eq!(go.show_all, false);
        assert_eq!(go.hide_unmatched, true);
        assert_eq!(go.dir_displayed, false);
        assert_eq!(go.debug, false);
    }
    #[test]
    fn set_dir() {
        let mut go = GenOut::new();
        go.set_dir(&Path::new("foobar"));
        assert_eq!(go.dir, PathBuf::from("foobar"));
    }
    #[test]
    fn set_file() {
        let mut go = GenOut::new();
        let path = Path::new("feebar");
        let arg = (path, true, "search text");
        go.set_file(arg);
        assert_eq!(go.file, path);
        assert_eq!(go.rslt, true);
        assert_eq!(go.txt, String::from("search text"));
    }
}
