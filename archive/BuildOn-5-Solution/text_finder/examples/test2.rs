/////////////////////////////////////////////////////////////////
// text_finder::test2.rs - search dir tree for files with text //
// ver 1.0                                                     //
// Jim Fawcett, https://JimFawcett.github.io, 22 Feb 2021      //
/////////////////////////////////////////////////////////////////
/*
  This version uses Executive struct.  Prefer this test2.rs
*/

use integration::{Executive};

fn main() {
    print!("\n  -- TextFinder, v1.0 --\n");

    /*-----------------------------------------------------
      set up processing pipeline:
      - Executive => DirNav<Finder> => Finder<Genout> => Genout 
      - Data flow:     ------->------------->-------------->
    */
    let mut ex = Executive::new();

    /* apply command line options */
    if !ex.process_cmdln() {
        return;
    }

    /* start searching dir tree rooted at specified path */
    if !ex.start() {
        print!("\n  can't start dir nav\n");
    }
    print!(
        "\n\n  processed {} files in {} dirs\n", 
        ex.get_files(), 
        ex.get_dirs()
    );
}