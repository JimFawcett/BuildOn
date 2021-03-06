///////////////////////////////////////////////////////////
// BuildOnStructure::Step1::test1.rs                     //
//   - demonstrate code structure for Step #1            //
// Jim Fawcett, https://JimFawcett.github.io, 14 Jan 21  //
///////////////////////////////////////////////////////////

use text_search::*;
use std::path::{Path};

fn main() {
    print!("\n  -- demonstrate structure for Step#1 --\n");
   let mut finder = Finder::new();
   finder.set_txt("BuildOn");
   finder.do_dir(&Path::new("Dir1"));
   finder.do_file(&Path::new("file1.txt"));
   finder.set_txt("BuildOff");
   finder.do_file(&Path::new("file2.txt"));
   finder.do_dir(&Path::new("Dir2"));
   finder.set_txt("abc");
   finder.do_file(&Path::new("file3.txt"));
   finder.set_txt("123");
   finder.do_file(&Path::new("file4.txt"));

   print!("\n\n  That's all Folks!\n\n");
}