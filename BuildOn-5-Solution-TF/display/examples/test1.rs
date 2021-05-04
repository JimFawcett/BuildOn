/////////////////////////////////////////////////////////////
// display::test1.rs - send text_finder data to console    //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 06 Mar 2012  //
/////////////////////////////////////////////////////////////

use display::*;
use text_search::{SearchEvent};
use std::path::Path;

fn main() {
    let mut out = GenOut::new(); 
    // out.show_all(true);
    // out.set_hide_unmatched(false);
    out.set_dir(Path::new("dir1"));
    let mut d = (Path::new("file1"), true, "struct");
    out.set_file(d);
    d.0 = Path::new("file2");
    out.set_file(d);
    out.set_dir(Path::new("dir2"));
    out.set_dir(Path::new("dir3"));
    d.0 = Path::new("file3");
    d.1 = false;
    d.2 = "foobar";
    out.set_file(d);
    
    print!("\n\n  That's all Folks!\n\n");
}