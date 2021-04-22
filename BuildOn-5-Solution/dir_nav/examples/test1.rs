/////////////////////////////////////////////////////////////
// rust_dir_nav::test1.rs                                  //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 12 Apr 2020  //
/////////////////////////////////////////////////////////////

#![allow(dead_code)]
#[allow(unused_imports)]
use dir_nav::*;
use std::env::current_dir;
use std::io;
use std::path::{Path, PathBuf};

/*---------------------------------------------------------
  Appl processes DirEvent events.  It defines application
  specific processing of files and directories.

  Here is where you define how the program output is
  structured and any options that affect output.
*/
struct Appl {
    curr_dir: PathBuf,      // save dir for display in do_file
    dir_out: bool,          // has directory name be displayed? 
    show_no_match: bool     // show dirs with no file matches?
}
impl DirEvent for Appl {
    fn new() -> Self {
        Self {
            curr_dir: PathBuf::new(),
            dir_out: false,
            show_no_match: false
        }
    }
    /*-----------------------------------------------------
      - save directory name
      - display only if show_no_match is true 
    */
    fn do_dir(&mut self, d: &Path) {
        self.curr_dir = PathBuf::from(d);
        self.dir_out = false;
        if self.show_no_match {
            print!("\n  {:?}", replace_sep(&d));
        }
    }
    /*-----------------------------------------------------
      - If show_no_match and dir.out are false, then
        display current directory.
      - display file name.
    */
    fn do_file(&mut self, f: &Path) {
        if !self.dir_out && !self.show_no_match {
            print!("\n  {:?}", replace_sep(&self.curr_dir));
            self.dir_out = true;
        }
        print!("\n      {:?}", f);
    }
}
impl Default for Appl {
    fn default() -> Self {
        Self {
            curr_dir: PathBuf::new(),
            dir_out: false,
            show_no_match: false
        }
    }
}
impl Appl {
    pub fn show_no_match(&mut self, show: bool) {
        self.show_no_match = show;
    } 
}

/*---------------------------------------------------------
  - Demonstrate output searching any of several 
    specified paths
  - Demonstrate output searching test directories.
*/
#[allow(unused_variables)]
fn main() -> io::Result<()> {

    let mut dn = DirNav::<Appl>::new();
    
    /*-- uncomment lines below to try options --*/
    // dn.get_app().show_no_match(true);
    // dn.recurse(false);

    /*-- takes a variety of formats --*/

    let _pat1 = Path::new("rs");
    let _pat4 = Path::new("rlib");

    dn.add_patt(&_pat1);
    dn.add_patt(Path::new("toml"));
    dn.add_patt(&Path::new("txt"));
    dn.add_patt(_pat4);
    dn.add_patt(Path::new("exe"));

    /*-- choose a path to search --*/
    let path1 = current_dir()?;
    let path2 = std::path::Path::new("..");
    let path3 = std::path::Path::new("..\\..");

    let path = path2;
    print!("\n  Searching path {:?}\n", &path);

    let path = std::path::Path::new(&path);
    let _rslt = dn.visit(path)?;
    
    print!(
        "\n\n  processed {} files and {} dirs",
        dn.get_files(),
        dn.get_dirs()
    );
    print!("\n");

    /*-- search test dirs, create with cargo test --*/
    dn.clear();
    dn.get_app().show_no_match(true);
    dn.add_patt(Path::new("rs"))
      .add_patt(Path::new("toml"))
      .add_patt(Path::new("exe"))
      .add_patt(Path::new("txt"));
    let mut path = std::path::PathBuf::new();
    path.push(".\\test_dir");
    print!("\n  Searching path {:?}\n", &path);
    let _rslt = dn.visit(&path)?;
    print!(
        "\n\n  processed {} files in {} dirs",
        dn.get_files(),
        dn.get_dirs()
    );
    ///////////////////////////////////////////////
    // uncomment lines below to see error return
    //---------------------------------------------
    // print!("\n");
    // path.pop();
    // path.push("foobar");
    // dn.visit(&path)?;
    
    print!("\n\n");
    Ok(())
}