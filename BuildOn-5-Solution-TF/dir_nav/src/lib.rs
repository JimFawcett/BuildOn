/////////////////////////////////////////////////////////////
// TextFinder::dir_nav::lib.rs                             //
//   - DFS navigation of specified directory root          //
// Jim Fawcett, https://JimFawcett.github.io, 27 Oct 2020  //
/////////////////////////////////////////////////////////////
/*
   DirNav<App> is a directory navigator that  uses generic
   parameter App to define how files and directories are
   handled.
   - displays only paths that have file targets by default
   - hide(false) will show all directories traversed
   - recurses directory tree at specified root by default
   - recurse(false) examines only specified path.
*/
#![allow(unused_imports)]
use std::fs::{self, DirEntry};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// trait required of the App generic parameter type
pub trait DirEvent {
    fn new() -> Self;
    fn do_dir(&mut self, d: &Path);
    fn do_file(&mut self, f: &Path);
}
//---------------------------------------------------------
// See examples\test1.rs for Sample implementation of 
// App that implements DirEvent and called by DirNav.
// --------------------------------------------------------

/////////////////////////////////////////////////
// Patterns are a collection of extension strings
// used to identify files as search targets

type SearchPatterns = Vec<PathBuf>;

/// Directory Navigator Structure
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DirNav<App: DirEvent> {
    /// file extensions to process
    pats: SearchPatterns,
    /// instance of App : DirEvent, requires impl of DirEvent
    app: App,
    /// number of files processed
    num_file: usize,
    /// number of dirs processed
    num_dir: usize,
    /// recurse ?
    recurse : bool,
}
impl<App: DirEvent + Default> DirNav<App> {
    pub fn new() -> Self
    where
        App: DirEvent + Default,
    {
        Self {
            pats: SearchPatterns::new(),
            app: App::new(),
            num_file: 0,
            num_dir: 0,
            recurse: true,
        }
    }
    /// visits are recursive?
    pub fn recurse(&mut self, p:bool) {
        self.recurse = p;
    }
    /// return reference to App to configure, get results
    pub fn get_app(&mut self) -> &mut App {
        &mut self.app
    }
    /// return number of dirs processed
    pub fn get_dirs(&self) -> usize {
        self.num_dir
    }
    /// return number of files processed
    pub fn get_files(&self) -> usize {
        self.num_file
    }
    /// return patterns, e.g., file extensions to look for
    pub fn get_patts(&self) -> &SearchPatterns {
        &self.pats
    }

    /// add extention to search for - takes literal path
    pub fn add_patt(&mut self, p: &Path) -> &mut DirNav<App> {
        self.pats.push(p.to_path_buf());
        self
    }
    /// reset to default state
    pub fn clear(&mut self) {
        self.pats.clear();
        self.num_dir = 0;
        self.num_file = 0;
        self.app = App::default();
        self.recurse = true;
    }
    /// Depth First Search for file extentions starting at path dir<br />
    /// Invokes DirEvent::do_dir and DirEvent::do_file
    pub fn visit(&mut self, dir: &Path) -> io::Result<()>
    where App: DirEvent
    {
        self.app.do_dir(dir);
        self.num_dir += 1;
        let mut sub_dirs = Vec::<PathBuf>::new();
        if dir.is_dir() {
            /* search local directory */
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    sub_dirs.push(path);  // save for processing after files
                } else {
                    self.num_file += 1;
                    if self.in_patterns(&entry) | self.pats.is_empty() {
                        self.app.do_file(&Path::new(&entry.file_name()));
                    }
                }
            }
            /*-- recurse into subdirectories --*/
            for sub in sub_dirs {
                let mut pb = std::path::PathBuf::new();
                pb.push(sub);
                if self.recurse {
                    self.visit(&pb)?;
                }
            }
            return Ok(());  // normal return
        }
        Err(Error::new(ErrorKind::Other, "not a directory"))
    }
    /// does store contain d.path().extension() ?
    pub fn in_patterns(&self, d: &DirEntry) -> bool {
        let p = d.path();
        let ext = p.extension();
        match ext {
            Some(extn) => self.pats.contains(&PathBuf::from(extn)),
            // Some(extn) => self.pats.contains(&(extn.to_os_string())),
            None => false,
        }
    }
}
/// replace Windows directory separator with Linux separator
pub fn replace_sep(path: &Path) -> std::ffi::OsString {
    let rtn = path.to_string_lossy();
    let mod_path = rtn.replace("\\", "/");
    let mut os_str: std::ffi::OsString = std::ffi::OsString::new();
    os_str.push(mod_path);
    os_str
}

#[cfg(test)]
/*---------------------------------------------------------
  Note: 
  - if you run tests with multiple threads, the default,
    then the first time you run them the dir walk may fail
    because test directories are not created before the 
    test is run. 
*/
mod tests {
    // test_setup() should run first. To ensure that:
    //     cargo -- --test-threads=1
    // to see console output:
    //     cargo test -- --show-output --test-threads=1
    use super::*;
    #[derive(Debug)]
    struct ApplTest {
        rslt_store: Vec<PathBuf>,
    }
    impl DirEvent for ApplTest {
        fn new() -> ApplTest {
            ApplTest {
                rslt_store: Vec::<PathBuf>::new()
            }
        }
        fn do_dir(&mut self, _d: &Path) {
            //print!("\n  {:?}", d);
        }
        fn do_file(&mut self, f: &Path) {
            //print!("\n    {:?}", f);
            self.rslt_store.push(PathBuf::from(f));
        }
    }
    impl Default for ApplTest {
        fn default() -> Self {
            ApplTest {
                rslt_store: Vec::<PathBuf>::new(),
            }
        }
    }
    #[test]
    fn test_setup() {
        let _ = std::fs::create_dir("./test_dir");
        let _ = std::fs::create_dir("./test_dir/test_sub1_dir");
        let _ = std::fs::create_dir("./test_dir/test_sub2_dir");
        let _ = std::fs::create_dir("./test_dir/test_sub3_dir");
        let _ = std::fs::File::create("./test_dir/test_file.rs");
        let _ = std::fs::File::create("./test_dir/test_sub1_dir/test_file1.rs");
        let _ = std::fs::File::create("./test_dir/test_sub1_dir/test_file2.exe");
        let _ = std::fs::File::create("./test_dir/test_sub2_dir/test_file3.txt");
        let _ = std::fs::File::create("./test_dir/test_sub3_dir/test_file4.bar");
    }
    #[test]
    fn test_walk() {
        let mut dn = DirNav::<ApplTest>::new();
        dn.add_patt(&Path::new("rs"))
          .add_patt(&Path::new("exe"))
          .add_patt(&Path::new("txt"));
        let mut pb = PathBuf::new();
        pb.push("./test_dir".to_string());
        let _ = dn.visit(&pb);
        let rl = &dn.get_app().rslt_store;
        /*
          run exe in target/debug with --nocapture option
          to see output of print statement below.
        */
        print!("\n  {:?}", rl);

        // test for found files
        let l = |s:&str| -> PathBuf { PathBuf::from(s) };

        assert!(rl.contains(&l("test_file.rs")));
        assert!(rl.contains(&l("test_file1.rs")));
        assert!(rl.contains(&l("test_file2.exe")));
        assert!(rl.contains(&l("test_file3.txt")));
        /*
          uncomment line below to make test fail
        */
        // assert!(rl.contains(&l("foobar")));
    }
    #[test]
    fn test_patts() {
        let mut dn = DirNav::<ApplTest>::new();
        dn.add_patt(&Path::new("foo"))
          .add_patt(&Path::new("bar"));
        assert_eq!(dn.get_patts().len(), 2);
        let pats = dn.get_patts();
        let foo_str = PathBuf::from("foo");
        // foo_str.push("foo");
        assert!(pats.contains(&foo_str));
        let bar_str = PathBuf::from("bar");
        assert!(pats.contains(&bar_str));
        dn.clear();
        assert_eq!(dn.get_patts().len(), 0);
    }
}
