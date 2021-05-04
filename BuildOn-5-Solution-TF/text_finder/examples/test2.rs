/////////////////////////////////////////////////////////////////
// text_finder::test2.rs - search dir tree for files with text //
// ver 1.1                                                     //
// Jim Fawcett, https://JimFawcett.github.io, 22 Feb 2021      //
/////////////////////////////////////////////////////////////////
/*
  test2.rs uses Executive struct.  Prefer this over test1.rs
*/
#![allow(unused_imports)]

use integration::{Executive};
use cmdln_parser::{show_parse};

fn main() {
    print!("\n  -- TextFinder, v1.1 --\n");

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
    else {
        /*-------------------------------------------------
           Uncomment next code line if you want rs to be 
           default pattern
        -------------------------------------------------*/
        // ex.set_attribute_item("p", "rs");
    }
    /* start searching dir tree rooted at specified path */
    if !ex.start() {
        print!("\n  can't start dir nav\n");
    }
    print!(
        "\n\n  processed {} files in {} dirs, {} matches\n", 
        ex.get_files(), 
        ex.get_dirs(),
        ex.get_matches()
    );
}