/////////////////////////////////////////////////////////////
// test1.rs - demonstrate text_search                      //
// ver 1.1                                                 //
// Jim Fawcett, https://JimFawcett.github.io, 20 Jan 2021  //
/////////////////////////////////////////////////////////////

use text_search::*;
use dir_nav::{DirEvent, replace_sep};
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
            print!("\n--dir: {:?}", replace_sep(dir));
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

fn find_text(t:&str) {
    print!("\n  -- testing find text --");
    print!("\n  searching for text: {:?}", t);
    let mut ts = Finder::<GenOut>::new();
    ts.set_txt(t);
    ts.do_dir(&Path::new(".\\src"));
    ts.do_file(&Path::new("lib.rs"));
    ts.do_dir(&Path::new(".\\examples"));
    ts.do_file(&Path::new("test1.rs"));
    /* these should not find */
    ts.do_file(&Path::new("no_exist"));
    ts.set_txt("foo_bar");
    ts.do_file(&Path::new("lib.rs"));
    println!();
}

fn match_regex(t:&str) {
    print!("\n  -- testing regex match --");
    print!("\n  searching with regex: {:?}", t);
    let mut ts = Finder::<GenOut>::new();
    ts.set_regex(t);
    ts.do_dir(&Path::new(".\\src"));
    ts.do_file(&Path::new("lib.rs"));
    ts.do_dir(&Path::new(".\\examples"));
    ts.do_file(&Path::new("test1.rs"));
    /* these should not match */
    ts.do_file(&Path::new("no_exist"));
    ts.set_regex("foo_bar");
    ts.do_file(&Path::new("lib.rs"));
    println!();
}

fn main() {
    print!("\n  {}","-- demo text_search package --\n");

    find_text("DirEvent");
    match_regex("DirEvent|main");

    print!("\n  {}","That's all Folks!\n\n");
}