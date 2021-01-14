///////////////////////////////////////////////////////////
// BuildOnStructure::Step1::test1.rs                     //
//   - demonstrate code structure for Step #1            //
// Jim Fawcett, https://JimFawcett.github.io, 14 Jan 21  //
///////////////////////////////////////////////////////////

use text_search::*;

fn main() {
    print!("\n  -- demonstrate structure for Step#1 --\n");
   let mut finder = Finder::new();
   finder.set_dir("Dir1");
   finder.find("file1.txt", "BuildOn");
   finder.find("file2.txt", "BuildOff");
   finder.set_dir("Dir2");
   finder.find("file3.txt", "abc");
   finder.find("file4.txt", "123");

   print!("\n\n  That's all Folks!\n\n");
}