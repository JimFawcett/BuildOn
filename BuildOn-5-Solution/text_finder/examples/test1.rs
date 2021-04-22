///////////////////////////////////////////////////////////////
// text_finder::test1 - search dir tree for files with text  //
// ver 1.0                                                   //
// Jim Fawcett, https://JimFawcett.github.io, 22 Feb 2021    //
///////////////////////////////////////////////////////////////
/*
  Early version - does not use Executive struct.  
  Prefer test2.rs
*/
#![allow(unused_imports)]
#![allow(dead_code)]
use std::{fmt::*, marker::PhantomData};
use std::path::{PathBuf, Path};
use cmdln_parser::{CmdParser, show_args};
use dir_nav::{DirNav, DirEvent, replace_sep};
use text_search::{Finder};
use display::GenOut;

/*-- call after parsing command line --*/
fn set_defaults(cp:&mut CmdParser) {
    cp.set_default("P", ".");       // Path
    cp.set_default("s", "true");    // recurse
    cp.set_default("H", "true");    // hide unmatched
    cp.set_default("v", "true");    // verbose
    cp.set_default("T", " ");       // Text to find
    cp.set_default("D", "false");   // Debug out
    cp.set_default("A", "false");
}
/*-- call if key "h" exists, then return --*/
fn help_msg() -> String {
  let mut hm = String::new();
  hm.push_str("\nCommand Line Format:");
  hm.push_str("\n  /P path /p rs /p h cpp /T text_to_find");
  hm.push_str("\nOptional attributes:");
  hm.push_str("\n  /H true|false, /v true|false, /s true|false");
  hm
}

fn get_first(key: &str, cp: &CmdParser) -> String {
    if let Some(vals) = cp.get(key) {
        if vals.len() > 0 {
            return vals[0].clone();
        }
    }
    "".to_string()
}

fn main() {
    let put_msg = |msg:&str| print!("\n  -- {:?} --", msg);
    put_msg("TextFinder, v1.0");

    let mut cp = CmdParser::new();
    cp.parse();
    cp.empty_values_to_true();

    if get_first("h", &cp) == "true" {
        print!("\n{}\n\n", help_msg());
        return;
    }
    set_defaults(&mut cp);

    let attrib = cp.get_parse();

    if get_first("v", &cp) == "true" {
        for arg in attrib {
            print!("\n  {:?}", arg);
        }
    }

    let mut dn = DirNav::<Finder<GenOut>>::new();

    if let Some(pats) = cp.get("p") {
        for pat in pats {
            let p = Path::new(pat);
            dn.add_patt(p);
        }
    }

    if let Some(txts) = cp.get("T") {
        if txts.len() > 0 {
            dn.get_app().set_txt(&txts[0]);
        }
    }

    let out = dn.get_app().get_app();

    if let Some(hides) = cp.get("H") {
        if hides.len() > 0 {
            out.set_hide_unmatched(hides[0] == String::from("true"));
        }
    }

    if let Some(outs) = cp.get("A") {
        if outs.len() > 0 {
            out.show_all(outs[0] == String::from("true"));
        }
    }
    if let Some(paths) = cp.get("P") {
        let mut path_string = String::new();
        if paths.len() > 0 {
            path_string = paths[0].clone();
        }
        let start_path = Path::new(&path_string);

        /*-- show Finder --*/
        print!("\n\n{:#?}\n", dn.get_app());
        let _ = dn.visit(start_path);
    }

    println!("\n\n  That's all Folks!\n\n");
}
