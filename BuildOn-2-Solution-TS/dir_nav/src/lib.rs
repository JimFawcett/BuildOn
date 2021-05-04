/////////////////////////////////////////////////////////////
// TextFinder::dir_nav::lib.rs                             //
//   - DFS navigation of specified directory root          //
// Jim Fawcett, https://JimFawcett.github.io, 27 Oct 2020  //
/////////////////////////////////////////////////////////////

use std::path::{Path};

/// trait required of the App generic parameter type
pub trait DirEvent {
    fn new() -> Self;
    fn do_dir(&mut self, d: &Path);
    fn do_file(&mut self, f: &Path);
}

pub fn replace_sep(path: &Path) -> std::ffi::OsString {
    let rtn = path.to_string_lossy();
    let mod_path = rtn.replace("\\", "/");
    let mut os_str: std::ffi::OsString = std::ffi::OsString::new();
    os_str.push(mod_path);
    os_str
}
