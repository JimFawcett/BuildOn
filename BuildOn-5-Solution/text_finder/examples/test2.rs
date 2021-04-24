/////////////////////////////////////////////////////////////////
// text_finder::test2.rs - search dir tree for files with text //
// ver 1.0                                                     //
// Jim Fawcett, https://JimFawcett.github.io, 22 Feb 2021      //
/////////////////////////////////////////////////////////////////
/*
  This version uses Executive struct.  Prefer this test2.rs
*/
#![allow(unused_imports)]

use integration::{Executive};
use cmdln_parser::{show_parse};

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
    else {
        // ex.set_attribute_item("p", "rs");
        let cp = ex.get_cparser();
        if let Some(vec) = cp.get("v") {
            if vec[0] == "true" {
                print!("{}\n", show_parse(ex.get_cparser_mut()));
            }
        }
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